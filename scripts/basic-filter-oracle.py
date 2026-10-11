"""Test-only source-audited BASIC oracle over supplied public protocol inputs.

Knots blockfilter.cpp, util/golombrice.h and streams.h define selection,
high-half range mapping, MSB-first Rice and zero padding. Bun owns orchestration.
This is corpus-cross-validated Python, not a directly linked Knots binary.
"""

import io
import json
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parents[1] / "packages/bitcoin-knots/test/functional"))
from test_framework.crypto.siphash import siphash
from test_framework.messages import CBlock, CBlockHeader, hash256, ser_compact_size
from test_framework.messages import (
    msg_getcfilters, msg_cfilter, msg_getcfheaders, msg_cfheaders,
    msg_getcfcheckpt, msg_cfcheckpt,
)


def compute(case, predecessors):
    outputs = [bytes.fromhex(script) for script in case["outputs"]]
    block_hash = bytes.fromhex(case["block_hash_raw"])
    if case["raw_block"]:
        stream = io.BytesIO(bytes.fromhex(case["raw_block"]))
        block = CBlock()
        block.deserialize(stream)
        if stream.read():
            raise ValueError("trailing raw block bytes")
        block_hash = hash256(CBlockHeader.serialize(block))
        outputs = [bytes(output.scriptPubKey) for tx in block.vtx for output in tx.vout]
    if len(block_hash) != 32:
        raise ValueError("block hash must have 32 raw bytes")
    spent = [bytes.fromhex(script) for script in case["spent"]]
    elements = set(script for script in spent if script)
    elements.update(script for script in outputs if script and script[0] != 0x6A)
    count = len(elements)
    k0 = int.from_bytes(block_hash[:8], "little")
    k1 = int.from_bytes(block_hash[8:16], "little")
    mapped = sorted((siphash(k0, k1, script) * (count * 784931)) >> 64 for script in elements)
    previous = 0
    bits = []
    for value in mapped:
        delta = value - previous
        bits.append("1" * (delta >> 19) + "0" + format(delta & ((1 << 19) - 1), "019b"))
        previous = value
    bit_string = "".join(bits)
    bit_string += "0" * ((-len(bit_string)) % 8)
    encoded = ser_compact_size(count) + bytes(int(bit_string[i:i + 8], 2) for i in range(0, len(bit_string), 8))
    digest = hash256(encoded)
    predecessor = case["predecessor_raw"]
    if case["previous_name"]:
        predecessor = predecessors[case["previous_name"]]
    raw_predecessor = bytes.fromhex(predecessor)
    if len(raw_predecessor) != 32:
        raise ValueError("predecessor must have 32 raw bytes")
    header = hash256(digest + raw_predecessor)
    predecessors[case["name"]] = header.hex()
    return dict(case, outputs=[script.hex() for script in outputs], block_hash_raw=block_hash.hex(),
                block_hash_display=block_hash[::-1].hex(), predecessor_raw=predecessor,
                encoded=encoded.hex(), hash_raw=digest.hex(), hash_display=digest[::-1].hex(),
                header_raw=header.hex(), header_display=header[::-1].hex(), count=count,
                k0=str(k0), k1=str(k1), mapped=mapped)


def peer_wire(case):
    """Use the pinned serializers; raw digest bytes map to uint256 little endian."""
    fields = case["input"]
    command = case["command"]
    filter_type = fields["filter_type"]
    raw_number = lambda value: int.from_bytes(bytes.fromhex(value), "little")
    if command in ("getcfilters", "getcfheaders"):
        constructor = msg_getcfilters if command == "getcfilters" else msg_getcfheaders
        message = constructor(filter_type, fields["start_height"], raw_number(fields["stop_hash_raw"]))
    elif command == "cfilter":
        message = msg_cfilter(filter_type, raw_number(fields["block_hash_raw"]), bytes.fromhex(fields["filter_data"]))
    elif command == "cfheaders":
        message = msg_cfheaders(filter_type, raw_number(fields["stop_hash_raw"]),
                                raw_number(fields["prev_header_raw"]), [raw_number(h) for h in fields["hashes_raw"]])
    elif command == "getcfcheckpt":
        message = msg_getcfcheckpt(filter_type, raw_number(fields["stop_hash_raw"]))
    elif command == "cfcheckpt":
        message = msg_cfcheckpt(filter_type, raw_number(fields["stop_hash_raw"]),
                                [raw_number(h) for h in fields["headers_raw"]])
    else:
        raise ValueError("unknown peer serializer")
    payload = message.serialize()
    # v1 header: pinned p2p.py build_message; hash256 is the pinned helper.
    frame = (bytes.fromhex("f9beb4d9") + message.msgtype.ljust(12, b"\x00") +
             len(payload).to_bytes(4, "little") + hash256(payload)[:4] + payload)
    return dict(case, expected=dict(payload_hex=payload.hex(), frame_hex=frame.hex()))


def peer_proof(case):
    """Independent integers mirroring chain.cpp and arith_uint256.cpp, including wrap."""
    fields = case["input"]
    bits = fields["bits"]
    size = bits >> 24
    word = bits & 0x007fffff
    if size <= 3:
        word >>= 8 * (3 - size)
        target = word
    else:
        target = word << (8 * (size - 3))
    mask = (1 << 256) - 1
    target &= mask
    negative = word != 0 and (bits & 0x00800000) != 0
    overflow = word != 0 and (size > 34 or (word > 0xff and size > 33) or (word > 0xffff and size > 32))
    proof = 0 if negative or overflow or target == 0 else ((mask - target) // (target + 1)) + 1
    to_work, from_work = int(fields["to_work"]), int(fields["from_work"])
    sign = 1 if to_work > from_work else -1
    saturated = False
    equivalent = None
    if proof:
        seconds = ((abs(to_work - from_work) * fields["spacing"]) & mask) // proof
        saturated = seconds.bit_length() > 63
        equivalent = sign * min(seconds, (1 << 63) - 1)
    allowed = fields["active"] or (fields["scripts_valid"] and fields["best_header"] and
                                  fields["timestamp_age"] < 2592000 and equivalent is not None and equivalent < 2592000)
    return dict(case, expected=dict(target_hex=target.to_bytes(32, "big").hex(), proof=str(proof),
                                   equivalent_time=None if equivalent is None else str(equivalent),
                                   sign=sign, saturated=saturated, allowed=allowed))


def peer_handler(case):
    """Source-derived assertions, never invented peer RPC/error response bytes."""
    fields = case["input"]
    outcome, reason = "respond", "complete"
    if fields["filter_type"] != 0 or not fields["advertised"]:
        outcome, reason = "disconnect", "type-or-service"
    elif not fields["known_stop"] or not fields["allowed_stop"]:
        outcome, reason = "disconnect", "stop"
    elif fields["family"] != "getcfcheckpt" and fields["start_height"] > fields["stop_height"]:
        outcome, reason = "disconnect", "reversed"
    elif fields["family"] != "getcfcheckpt" and fields["stop_height"] - fields["start_height"] + 1 > (
            1000 if fields["family"] == "getcfilters" else 2000):
        outcome, reason = "disconnect", "range"
    elif not fields["backend"]:
        outcome, reason = "no-normal-response", "index"
    elif not fields["complete"]:
        outcome, reason = "no-normal-response", "missing"
    return dict(case, expected=dict(outcome=outcome, reason=reason))


def peer_malformed(case, wire_cases):
    """Trailing extraction uses helpers; short-read refusal is C++ streams.h evidence."""
    source = next(row for row in wire_cases if row["command"] == case["command"])
    payload = bytes.fromhex(peer_wire(source)["expected"]["payload_hex"])
    payload = payload + b"\xff" if case["mutation"] == "append-ff" else payload[:-1]
    constructor = {"getcfilters": msg_getcfilters, "getcfheaders": msg_getcfheaders,
                   "getcfcheckpt": msg_getcfcheckpt}[case["command"]]
    stream = io.BytesIO(payload)
    if case["mutation"] == "append-ff":
        constructor().deserialize(stream)
        if stream.read() != b"\xff":
            raise ValueError("pinned trailing-byte extraction disagrees")
    # messages.py deser_uint256 uses a permissive read; do not confuse it with
    # the C++ DataStream::read end-of-data exception used by the real handler.
    return dict(case, payload_hex=payload.hex())


def main():
    payload = json.load(sys.stdin)
    if sys.argv[1:] == ["--peer-wire-proof"]:
        result = dict(wire=[peer_wire(case) for case in payload["wire"]],
                      proof=[peer_proof(case) for case in payload["proof"]],
                      handlers=[peer_handler(case) for case in payload["handlers"]],
                      malformed=[peer_malformed(case, payload["wire"]) for case in payload["malformed"]])
        json.dump(result, sys.stdout, separators=(",", ":"))
        return
    if sys.argv[1:]:
        raise ValueError("expected no arguments or --peer-wire-proof")
    predecessors = {}
    cases = [compute(case, predecessors) for case in payload["cases"]]
    lengths = [64, 255, 256, 257]
    siphash_vectors = [dict(length=n, expected=str(siphash(0x0706050403020100, 0x0F0E0D0C0B0A0908,
                                                       bytes(i % 256 for i in range(n))))) for n in lengths]
    json.dump(dict(cases=cases, siphash=siphash_vectors), sys.stdout, separators=(",", ":"))


if __name__ == "__main__":
    main()

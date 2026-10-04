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


def main():
    payload = json.load(sys.stdin)
    predecessors = {}
    cases = [compute(case, predecessors) for case in payload["cases"]]
    lengths = [64, 255, 256, 257]
    siphash_vectors = [dict(length=n, expected=str(siphash(0x0706050403020100, 0x0F0E0D0C0B0A0908,
                                                       bytes(i % 256 for i in range(n))))) for n in lengths]
    json.dump(dict(cases=cases, siphash=siphash_vectors), sys.stdout, separators=(",", ":"))


if __name__ == "__main__":
    main()

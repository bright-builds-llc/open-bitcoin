// Parity breadcrumbs:
// - packages/bitcoin-knots/src/consensus/tx_check.cpp
// - packages/bitcoin-knots/src/consensus/tx_verify.cpp
// - packages/bitcoin-knots/src/consensus/validation.h
// - packages/bitcoin-knots/src/validation.cpp
// - packages/bitcoin-knots/src/test/data/tx_valid.json
// - packages/bitcoin-knots/src/test/data/tx_invalid.json

pub(super) struct LegacySighashVector {
    pub(super) raw_tx: String,
    pub(super) script: String,
    pub(super) input_index: usize,
    pub(super) hash_type: u32,
    pub(super) expected_hash: String,
}

pub(super) struct ScriptVector {
    pub(super) comment: &'static str,
    pub(super) witness_stack: &'static [&'static str],
    pub(super) amount_sats: i64,
    pub(super) script_sig: &'static str,
    pub(super) script_pubkey: &'static str,
    pub(super) flags: &'static str,
    pub(super) expected: &'static str,
}

#[derive(Clone, Debug)]
pub(super) enum JsonValue {
    String(String),
    Number(String),
    Array(Vec<JsonValue>),
}

pub(super) const SCRIPT_VECTORS: &[ScriptVector] = &[
    ScriptVector {
        comment: "Basic P2WSH",
        witness_stack: &[
            "304402200d461c140cfdfcf36b94961db57ae8c18d1cb80e9d95a9e47ac22470c1bf125502201c8dc1cbfef6a3ef90acbbb992ca22fe9466ee6f9d4898eda277a7ac3ab4b25101",
            "410479be667ef9dcbbac55a06295ce870b07029bfcdb2dce28d959f2815b16f81798483ada7726a3c4655da4fbfc0e1108a8fd17b448a68554199c47d08ffb10d4b8ac",
        ],
        amount_sats: 1,
        script_sig: "",
        script_pubkey: "0 0x20 0xb95237b48faaa69eb078e1170be3b5cbb3fddf16d0a991e14ad274f7b33a4f64",
        flags: "P2SH,WITNESS",
        expected: "OK",
    },
    ScriptVector {
        comment: "Basic P2WPKH",
        witness_stack: &[
            "304402201e7216e5ccb3b61d46946ec6cc7e8c4e0117d13ac2fd4b152197e4805191c74202203e9903e33e84d9ee1dd13fb057afb7ccfb47006c23f6a067185efbc9dd780fc501",
            "0479be667ef9dcbbac55a06295ce870b07029bfcdb2dce28d959f2815b16f81798483ada7726a3c4655da4fbfc0e1108a8fd17b448a68554199c47d08ffb10d4b8",
        ],
        amount_sats: 1,
        script_sig: "",
        script_pubkey: "0 0x14 0x91b24bf9f5288532960ac687abb035127b1d28a5",
        flags: "P2SH,WITNESS",
        expected: "OK",
    },
    ScriptVector {
        comment: "Basic P2SH(P2WSH)",
        witness_stack: &[
            "3044022066e02c19a513049d49349cf5311a1b012b7c4fae023795a18ab1d91c23496c22022025e216342c8e07ce8ef51e8daee88f84306a9de66236cab230bb63067ded1ad301",
            "410479be667ef9dcbbac55a06295ce870b07029bfcdb2dce28d959f2815b16f81798483ada7726a3c4655da4fbfc0e1108a8fd17b448a68554199c47d08ffb10d4b8ac",
        ],
        amount_sats: 1,
        script_sig: "0x22 0x0020b95237b48faaa69eb078e1170be3b5cbb3fddf16d0a991e14ad274f7b33a4f64",
        script_pubkey: "HASH160 0x14 0xf386c2ba255cc56d20cfa6ea8b062f8b59945518 EQUAL",
        flags: "P2SH,WITNESS",
        expected: "OK",
    },
    ScriptVector {
        comment: "Basic P2SH(P2WPKH)",
        witness_stack: &[
            "304402200929d11561cd958460371200f82e9cae64c727a495715a31828e27a7ad57b36d0220361732ced04a6f97351ecca21a56d0b8cd4932c1da1f8f569a2b68e5e48aed7801",
            "0479be667ef9dcbbac55a06295ce870b07029bfcdb2dce28d959f2815b16f81798483ada7726a3c4655da4fbfc0e1108a8fd17b448a68554199c47d08ffb10d4b8",
        ],
        amount_sats: 1,
        script_sig: "0x16 0x001491b24bf9f5288532960ac687abb035127b1d28a5",
        script_pubkey: "HASH160 0x14 0x17743beb429c55c942d2ec703b98c4d57c2df5c6 EQUAL",
        flags: "P2SH,WITNESS",
        expected: "OK",
    },
    ScriptVector {
        comment: "P2WPKH with future witness version",
        witness_stack: &[
            "304402205ae57ae0534c05ca9981c8a6cdf353b505eaacb7375f96681a2d1a4ba6f02f84022056248e68643b7d8ce7c7d128c9f1f348bcab8be15d094ad5cadd24251a28df8001",
            "0479be667ef9dcbbac55a06295ce870b07029bfcdb2dce28d959f2815b16f81798483ada7726a3c4655da4fbfc0e1108a8fd17b448a68554199c47d08ffb10d4b8",
        ],
        amount_sats: 0,
        script_sig: "",
        script_pubkey: "1 0x14 0x91b24bf9f5288532960ac687abb035127b1d28a5",
        flags: "DISCOURAGE_UPGRADABLE_WITNESS_PROGRAM,P2SH,WITNESS",
        expected: "DISCOURAGE_UPGRADABLE_WITNESS_PROGRAM",
    },
    ScriptVector {
        comment: "P2WPKH with wrong witness program length",
        witness_stack: &[
            "3044022064100ca0e2a33332136775a86cd83d0230e58b9aebb889c5ac952abff79a46ef02205f1bf900e022039ad3091bdaf27ac2aef3eae9ed9f190d821d3e508405b9513101",
            "0479be667ef9dcbbac55a06295ce870b07029bfcdb2dce28d959f2815b16f81798483ada7726a3c4655da4fbfc0e1108a8fd17b448a68554199c47d08ffb10d4b8",
        ],
        amount_sats: 0,
        script_sig: "",
        script_pubkey: "0 0x1f 0xb34b78da162751647974d5cb7410aa428ad339dbf7d1e16e833f68a0cbf1c3",
        flags: "P2SH,WITNESS",
        expected: "WITNESS_PROGRAM_WRONG_LENGTH",
    },
    ScriptVector {
        comment: "P2WSH with witness program mismatch",
        witness_stack: &[
            "3044022039105b995a5f448639a997a5c90fda06f50b49df30c3bdb6663217bf79323db002206fecd54269dec569fcc517178880eb58bb40f381a282bb75766ff3637d5f4b4301",
            "400479be667ef9dcbbac55a06295ce870b07029bfcdb2dce28d959f2815b16f81798483ada7726a3c4655da4fbfc0e1108a8fd17b448a68554199c47d08ffb10d4b8ac",
        ],
        amount_sats: 0,
        script_sig: "",
        script_pubkey: "0 0x20 0xb95237b48faaa69eb078e1170be3b5cbb3fddf16d0a991e14ad274f7b33a4f64",
        flags: "P2SH,WITNESS",
        expected: "WITNESS_PROGRAM_MISMATCH",
    },
    ScriptVector {
        comment: "P2WPKH with non-empty scriptSig",
        witness_stack: &[
            "304402201a96950593cb0af32d080b0f193517f4559241a8ebd1e95e414533ad64a3f423022047f4f6d3095c23235bdff3aeff480d0529c027a3f093cb265b7cbf148553b85101",
            "0479be667ef9dcbbac55a06295ce870b07029bfcdb2dce28d959f2815b16f81798483ada7726a3c4655da4fbfc0e1108a8fd17b448a68554199c47d08ffb10d4b8",
        ],
        amount_sats: 0,
        script_sig: "11",
        script_pubkey: "0 0x14 0x91b24bf9f5288532960ac687abb035127b1d28a5",
        flags: "P2SH,WITNESS",
        expected: "WITNESS_MALLEATED",
    },
    ScriptVector {
        comment: "P2SH(P2WPKH) with superfluous push in scriptSig",
        witness_stack: &[
            "304402204209e49457c2358f80d0256bc24535b8754c14d08840fc4be762d6f5a0aed80b02202eaf7d8fc8d62f60c67adcd99295528d0e491ae93c195cec5a67e7a09532a88001",
            "048282263212c609d9ea2a6e3e172de238d8c39cabd5ac1ca10646e23fd5f5150811f8a8098557dfe45e8256e830b60ace62d613ac2f7b17bed31b6eaff6e26caf",
        ],
        amount_sats: 0,
        script_sig: "11 0x16 0x00147cf9c846cd4882efec4bf07e44ebdad495c94f4b",
        script_pubkey: "HASH160 0x14 0x4e0c2aed91315303fc6a1dc4c7bc21c88f75402e EQUAL",
        flags: "P2SH,WITNESS",
        expected: "WITNESS_MALLEATED_P2SH",
    },
    ScriptVector {
        comment: "P2PK with witness",
        witness_stack: &[""],
        amount_sats: 0,
        script_sig: "0x47 0x304402200a5c6163f07b8d3b013c4d1d6dba25e780b39658d79ba37af7057a3b7f15ffa102201fd9b4eaa9943f734928b99a83592c2e7bf342ea2680f6a2bb705167966b742001",
        script_pubkey: "0x41 0x0479be667ef9dcbbac55a06295ce870b07029bfcdb2dce28d959f2815b16f81798483ada7726a3c4655da4fbfc0e1108a8fd17b448a68554199c47d08ffb10d4b8 CHECKSIG",
        flags: "P2SH,WITNESS",
        expected: "WITNESS_UNEXPECTED",
    },
    ScriptVector {
        comment: "Basic P2WSH with compressed key",
        witness_stack: &[
            "304402204256146fcf8e73b0fd817ffa2a4e408ff0418ff987dd08a4f485b62546f6c43c02203f3c8c3e2febc051e1222867f5f9d0eaf039d6792911c10940aa3cc74123378e01",
            "210279be667ef9dcbbac55a06295ce870b07029bfcdb2dce28d959f2815b16f81798ac",
        ],
        amount_sats: 1,
        script_sig: "",
        script_pubkey: "0 0x20 0x1863143c14c5166804bd19203356da136c985678cd4d27a1b8c6329604903262",
        flags: "P2SH,WITNESS,WITNESS_PUBKEYTYPE",
        expected: "OK",
    },
    ScriptVector {
        comment: "Basic P2WPKH with compressed key",
        witness_stack: &[
            "304402204edf27486f11432466b744df533e1acac727e0c83e5f912eb289a3df5bf8035f022075809fdd876ede40ad21667eba8b7e96394938f9c9c50f11b6a1280cce2cea8601",
            "0279be667ef9dcbbac55a06295ce870b07029bfcdb2dce28d959f2815b16f81798",
        ],
        amount_sats: 1,
        script_sig: "",
        script_pubkey: "0 0x14 0x751e76e8199196d454941c45d1b3a323f1433bd6",
        flags: "P2SH,WITNESS,WITNESS_PUBKEYTYPE",
        expected: "OK",
    },
    ScriptVector {
        comment: "P2WSH CHECKMULTISIG with compressed keys",
        witness_stack: &[
            "",
            "304402207eb8a59b5c65fc3f6aeef77066556ed5c541948a53a3ba7f7c375b8eed76ee7502201e036a7a9a98ff919ff94dc905d67a1ec006f79ef7cff0708485c8bb79dce38e01",
            "5121038282263212c609d9ea2a6e3e172de238d8c39cabd5ac1ca10646e23fd5f51508210279be667ef9dcbbac55a06295ce870b07029bfcdb2dce28d959f2815b16f8179852ae",
        ],
        amount_sats: 1,
        script_sig: "",
        script_pubkey: "0 0x20 0x06c24420938f0fa3c1cb2707d867154220dca365cdbfa0dd2a83854730221460",
        flags: "P2SH,WITNESS,WITNESS_PUBKEYTYPE",
        expected: "OK",
    },
    ScriptVector {
        comment: "P2WSH CHECKMULTISIG with first key uncompressed and signing with the first key",
        witness_stack: &[
            "",
            "304402202d092ededd1f060609dbf8cb76950634ff42b3e62cf4adb69ab92397b07d742302204ff886f8d0817491a96d1daccdcc820f6feb122ee6230143303100db37dfa79f01",
            "5121038282263212c609d9ea2a6e3e172de238d8c39cabd5ac1ca10646e23fd5f51508410479be667ef9dcbbac55a06295ce870b07029bfcdb2dce28d959f2815b16f81798483ada7726a3c4655da4fbfc0e1108a8fd17b448a68554199c47d08ffb10d4b852ae",
        ],
        amount_sats: 1,
        script_sig: "",
        script_pubkey: "0 0x20 0x08a6665ebfd43b02323423e764e185d98d1587f903b81507dbb69bfc41005efa",
        flags: "P2SH,WITNESS",
        expected: "OK",
    },
    ScriptVector {
        comment: "P2WSH CHECKMULTISIG with second key uncompressed and signing with the first key should pass as the uncompressed key is not used",
        witness_stack: &[
            "",
            "3044022046f5367a261fd8f8d7de6eb390491344f8ec2501638fb9a1095a0599a21d3f4c02205c1b3b51d20091c5f1020841bbca87b44ebe25405c64e4acf758f2eae8665f8401",
            "5141048282263212c609d9ea2a6e3e172de238d8c39cabd5ac1ca10646e23fd5f5150811f8a8098557dfe45e8256e830b60ace62d613ac2f7b17bed31b6eaff6e26caf210279be667ef9dcbbac55a06295ce870b07029bfcdb2dce28d959f2815b16f8179852ae",
        ],
        amount_sats: 1,
        script_sig: "",
        script_pubkey: "0 0x20 0x230828ed48871f0f362ce9432aa52f620f442cc8d9ce7a8b5e798365595a38bb",
        flags: "P2SH,WITNESS,WITNESS_PUBKEYTYPE",
        expected: "OK",
    },
];

use open_bitcoin_primitives::BlockHeader;

pub(super) fn mainnet_prefix_headers() -> Result<Vec<BlockHeader>, Box<dyn std::error::Error>> {
    let prefix = include_str!("../../docs/parity/fixtures/mainnet-prefix-200.hex");
    let boundary = include_str!("../../docs/parity/fixtures/mainnet-block-201.hex");
    prefix
        .lines()
        .chain(boundary.lines())
        .map(|block| {
            let hex = block.get(..160).ok_or("expected a full wire header")?;
            let bytes = (0..160)
                .step_by(2)
                .map(|offset| u8::from_str_radix(&hex[offset..offset + 2], 16))
                .collect::<Result<Vec<_>, _>>()?;
            Ok(open_bitcoin_codec::block::parse_block_header(&bytes)?)
        })
        .collect()
}

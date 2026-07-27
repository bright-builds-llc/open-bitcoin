use super::*;

#[test]
fn script_num_helper_covers_negative_and_high_bit_cases() {
    assert_eq!(serialized_script_num(-1), vec![1, 0x81]);
    assert_eq!(serialized_script_num(128), vec![2, 0x80, 0x00]);
}

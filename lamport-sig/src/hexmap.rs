pub fn hex_map_u8(utf_byte: &u8) -> u8 {
    match utf_byte {
        48..=57 => utf_byte - 48,
        97..=102 => utf_byte - 87,
        x => panic!("Encountered unexpected hex character: {}", x),
    }
}
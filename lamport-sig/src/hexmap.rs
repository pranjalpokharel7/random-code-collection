pub fn hex_map_u8(utf_byte: &u8) -> u8 {
    match utf_byte {
        48 => 0,
        49 => 1,
        50 => 2,
        51 => 3,
        52 => 4,
        53 => 5,
        54 => 6,
        55 => 7,
        56 => 8,
        57 => 9,
        97 => 10,
        98 => 11,
        99 => 12,
        100 => 13,
        101 => 14,
        102 => 15,
        x => panic!("Encountered unexpected hex character: {}", x),
    }
}
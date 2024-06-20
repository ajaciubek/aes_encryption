use std::io::Write;

pub const BLOCK_LEN: usize = 16;

fn read_block(data: &[u8]) -> [u8; BLOCK_LEN] {
    let mut buffer = [0; BLOCK_LEN];
    let mut test: &mut [u8] = &mut buffer;
    let _ = test.write(data);
    buffer
}

pub fn read_string(s: String) -> Vec<[u8; BLOCK_LEN]> {
    let mut result = Vec::new();
    let bytes = s.as_bytes();
    for i in (0..bytes.len()).step_by(BLOCK_LEN) {
        if (i + BLOCK_LEN) > bytes.len() {
            result.push(read_block(&bytes[i..(bytes.len())]));
            break;
        }
        result.push(read_block(&bytes[i..(i + BLOCK_LEN)]));
    }
    result
}

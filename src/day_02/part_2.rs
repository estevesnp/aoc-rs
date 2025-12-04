#[allow(dead_code)]
const INPUT: &str = include_str!("input.txt");
// const INPUT: &str = include_str!("small.txt");

pub fn run() -> Result<usize, String> {
    INPUT.trim_end().split(',').try_fold(0, |acc, range| {
        let (start, end) = range
            .split_once('-')
            .ok_or_else(|| format!("invalid range: {range}"))?;

        let start: usize = start
            .parse()
            .map_err(|e| format!("invalid start '{start}': {e}"))?;
        let end: usize = end
            .parse()
            .map_err(|e| format!("invalid end '{end}': {e}"))?;

        let sum: usize = (start..=end).filter(|num| is_invalid(*num)).sum();
        Ok(acc + sum)
    })
}

fn is_invalid(num: usize) -> bool {
    let s = num.to_string();
    let bytes = s.as_bytes();
    let len = bytes.len();

    (1..=len / 2)
        .filter(|n| len.is_multiple_of(*n))
        .any(|n| is_invalid_seq(bytes, n))
}

fn is_invalid_seq(bytes: &[u8], seq_size: usize) -> bool {
    let seq = &bytes[..seq_size];
    bytes
        .chunks_exact(seq_size)
        .skip(1)
        .all(|chunk| chunk == seq)
}

#[allow(dead_code)]
const INPUT: &str = include_str!("input.txt");
// const INPUT: &str = include_str!("small.txt");

pub fn run() -> Result<usize, String> {
    let mut count: usize = 0;
    for range in INPUT.split(',') {
        let (start, end) = range
            .split_once('-')
            .ok_or_else(|| format!("invalid range: {range}"))?;

        let start: usize = start
            .parse()
            .map_err(|e| format!("invalid start '{start}': {e}"))?;
        let end: usize = end
            .trim_end()
            .parse()
            .map_err(|e| format!("invalid end '{end}': {e}"))?;

        let sum: usize = (start..=end).filter(|num| is_invalid(*num)).sum();
        count += sum;
    }

    Ok(count)
}

fn is_invalid(num: usize) -> bool {
    let digits = count_digits(num);
    if !digits.is_multiple_of(2) {
        return false;
    }

    let div = 10_usize.pow(digits / 2);
    let left = num / div;
    let right = num % div;

    left == right
}

fn count_digits(num: usize) -> u32 {
    let mut count: u32 = 1;

    while num / (10_usize.pow(count)) != 0 {
        count += 1;
    }

    count
}

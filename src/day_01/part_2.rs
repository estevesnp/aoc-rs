#[allow(dead_code)]
const INPUT: &str = include_str!("input.txt");
// const INPUT: &str = include_str!("small.txt");

#[derive(Debug)]
enum Side {
    Left,
    Right,
}
impl TryFrom<char> for Side {
    type Error = String;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'L' => Ok(Side::Left),
            'R' => Ok(Side::Right),
            _ => Err(format!("invalid value: {value}")),
        }
    }
}

pub fn run() -> Result<usize, String> {
    let mut curr: usize = 50;
    let mut flips: usize = 0;

    for line in INPUT.lines() {
        let mut chars = line.chars();
        let side = Side::try_from(chars.next().ok_or_else(|| "no char found".to_string())?)?;
        let num: usize = chars
            .as_str()
            .parse()
            .map_err(|e| format!("error parsing num: {e}"))?;

        let amnt = num % 100;
        flips += num / 100;

        match side {
            Side::Left => {
                if amnt > curr {
                    if curr != 0 {
                        flips += 1;
                    }
                    curr = 100 - (amnt - curr);
                } else {
                    curr -= amnt;
                    if curr == 0 {
                        flips += 1;
                    }
                }
            }
            Side::Right => {
                let sum = curr + amnt;
                if sum > 99 {
                    flips += 1;
                }
                curr = sum % 100
            }
        }
    }

    Ok(flips)
}

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
    let mut count: usize = 0;

    for line in INPUT.lines() {
        let mut chars = line.chars();
        let side = Side::try_from(chars.next().ok_or_else(|| "no char found".to_string())?)?;
        let amnt: usize = chars
            .as_str()
            .parse()
            .map_err(|e| format!("error parsing num: {e}"))?;
        let amnt = amnt % 100;

        match side {
            Side::Left => {
                if amnt > curr {
                    curr = 100 - (amnt - curr);
                } else {
                    curr -= amnt;
                }
            }
            Side::Right => curr = (curr + amnt) % 100,
        }

        if curr == 0 {
            count += 1;
        }
    }

    Ok(count)
}

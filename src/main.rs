use std::fs;
use std::path::Path;

use clap::{Parser, ValueEnum};

mod day_01;
mod day_02;

fn main() -> Result<(), String> {
    let cli = Cli::parse();

    if let Action::Add = cli.action {
        let day = match cli.day {
            Some(d) => d,
            None => highest_day()?.unwrap_or(0) + 1,
        };
        return add(day);
    }

    let day = match cli.day {
        Some(d) => d,
        None => highest_day()?.ok_or_else(|| "no day provided".to_string())?,
    };

    println!("day {day}");

    match day {
        1 => day_01::runner().run(cli.part)?,
        2 => day_02::runner().run(cli.part)?,
        _ => panic!("day {} not implemented", day),
    };

    Ok(())
}

const MOD_TEMPL: &[u8] = include_bytes!("../template/mod.rs");
const PART_TEMPL: &[u8] = include_bytes!("../template/part_x.rs");

fn add(day: usize) -> Result<(), String> {
    if days_iter()?.any(|d| d == day) {
        return Err(format!("day {day} already exists"));
    }

    let dir_str = format!("src/day_{:02}", day);
    let dir = Path::new(&dir_str);

    fs::create_dir(dir).map_err(|e| format!("error creating dir for {dir_str}: {e}"))?;

    create_write_file(&dir.join("mod.rs"), MOD_TEMPL)?;
    create_write_file(&dir.join("part_1.rs"), PART_TEMPL)?;
    create_write_file(&dir.join("part_2.rs"), PART_TEMPL)?;
    create_write_file(&dir.join("input.txt"), &[])?;

    println!("created {dir_str}");

    Ok(())
}

fn create_write_file(path: &Path, content: &[u8]) -> Result<(), String> {
    fs::write(path, content).map_err(|e| format!("error writing {}: {}", path.display(), e))
}

fn highest_day() -> Result<Option<usize>, String> {
    let mut days: Vec<usize> = days_iter()?.collect();
    days.sort_unstable();
    Ok(days.last().copied())
}

fn days_iter() -> Result<impl Iterator<Item = usize>, String> {
    let iter = fs::read_dir("src")
        .map_err(|e| format!("error opening 'src': {e}"))?
        .filter_map(|entry| {
            let filename = match entry {
                Ok(dir_entry) => dir_entry.file_name(),
                Err(e) => {
                    eprintln!("error reading entry: {e}");
                    return None;
                }
            };

            let filename = filename.to_str()?;
            let day = filename.strip_prefix("day_")?;

            match day.parse() {
                Ok(day) => Some(day),
                Err(e) => {
                    eprintln!("error parsing day from dir '{day}': {e}");
                    None
                }
            }
        });

    Ok(iter)
}

#[derive(Clone, ValueEnum)]
pub enum Action {
    Run,
    Add,
}

#[derive(Clone, ValueEnum)]
pub enum Part {
    One,
    Two,
    All,
}

#[derive(Parser)]
struct Cli {
    #[arg(short, long)]
    day: Option<usize>,
    #[arg(short, long, value_enum, default_value_t = Part::All)]
    part: Part,
    #[arg(short, long, value_enum, default_value_t = Action::Run)]
    action: Action,
}

pub trait Day {
    fn part_1(&self) -> Result<usize, String>;
    fn part_2(&self) -> Result<usize, String>;
    fn run(&self, part: Part) -> Result<(), String> {
        match part {
            Part::One => {
                println!("running part 1");
                println!("RESULT: {}", self.part_1()?);
            }
            Part::Two => {
                println!("running part 2");
                println!("RESULT: {}", self.part_2()?);
            }
            Part::All => {
                println!("running part 1");
                println!("RESULT: {}", self.part_1()?);
                println!("running part 2");
                println!("RESULT: {}", self.part_2()?);
            }
        }

        Ok(())
    }
}

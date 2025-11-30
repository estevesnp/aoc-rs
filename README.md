# aoc rs

scaffolding for easily running and adding new AOC challenge implementations

## run AOC day

simply run `cargo run` to run the latest day
can also specify the day and part to run with `cargo run -- --day <day> --part <part>`

## add AOC day

run `cargo run -- --action add --day <day>` to add a specific day if it doesn't exist.
omit the `day` option to add a new, latest day

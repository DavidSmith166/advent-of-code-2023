use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{char, digit1, newline},
    combinator::{opt, value},
    error::{context, VerboseError},
    multi::fold_many1,
    sequence::{pair, preceded, separated_pair, terminated},
    IResult,
};
use std::{fs, cmp::max};

#[derive(Clone, Debug)]
enum Color {
    RED,
    GREEN,
    BLUE,
}

#[derive(Clone, Debug)]
struct Entry {
    color: Color,
    value: u32,
}

#[derive(Clone, Debug)]
struct Round {
    red: u32,
    green: u32,
    blue: u32,
}

#[derive(Clone, Debug)]
struct Game {
    id: u32,
    rounds: Vec<Round>,
}

fn entry(input: &str) -> IResult<&str, Entry, VerboseError<&str>> {
    let parse_red = value(Color::RED, tag("red"));
    let parse_green = value(Color::GREEN, tag("green"));
    let parse_blue = value(Color::BLUE, tag("blue"));
    let parse_color = context("parse_color", alt((parse_red, parse_green, parse_blue)));
    let mut parse_entry = preceded(
        opt(char(' ')),
        context(
            "parse_entry",
            pair(
                context(
                    "terminated value",
                    terminated(context("entry value", digit1), char(' ')),
                ),
                context(
                    "terminated color",
                    terminated(context("parse_color", parse_color), opt(char(','))),
                ),
            ),
        ),
    );
    let (input, (num_str, color)) = parse_entry(input)?;
    let value = num_str.parse::<u32>().expect("I thought I was a number");
    let entry = Entry { color, value };
    Ok((input, entry))
}

fn round(input: &str) -> IResult<&str, Round, VerboseError<&str>> {
    let mut parse_entries = context(
        "parse_entries",
        terminated(
            context(
                "entry many",
                fold_many1(entry, Vec::new, |mut acc: Vec<_>, item| {
                    acc.push(item);
                    acc
                }),
            ),
            context("round end", opt(char(';'))),
        ),
    );
    let (input, entries) = parse_entries(input)?;
    let mut sum_red = 0;
    let mut sum_green = 0;
    let mut sum_blue = 0;
    for entry in entries {
        match entry.color {
            Color::RED => sum_red += entry.value,
            Color::GREEN => sum_green += entry.value,
            Color::BLUE => sum_blue += entry.value,
        };
    }
    let round = Round {
        red: sum_red,
        green: sum_green,
        blue: sum_blue,
    };
    Ok((input, round))
}

fn game(input: &str) -> IResult<&str, Game, VerboseError<&str>> {
    let mut parse_header = context(
        "parse_header",
        separated_pair(tag("Game"), char(' '), terminated(digit1, tag(": "))),
    );
    let mut parse_rounds = context(
        "parse_rounds",
        terminated(
            context(
                "round many",
                fold_many1(round, Vec::new, |mut acc: Vec<_>, item| {
                    acc.push(item);
                    acc
                }),
            ),
            context("game end", opt(newline)),
        ),
    );
    let (input, (_, id_str)) = parse_header(input)?;
    let (input, rounds) = parse_rounds(input)?;
    let game: Game = Game {
        id: id_str.parse::<u32>().expect("Game id not a number"),
        rounds,
    };
    Ok((input, game))
}

fn games(input: &str) -> IResult<&str, Vec<Game>, VerboseError<&str>> {
    let mut parse_games = context(
        "parse_games",
        fold_many1(game, Vec::new, |mut acc: Vec<_>, item| {
            acc.push(item);
            acc
        }),
    );
    Ok(parse_games(input))?
}

fn main() {
    let document = fs::read_to_string("input.txt").expect("404 File not found");
    let (_, games) = games(&document).unwrap();
    let mut power_sum = 0;
    for game in games {
        let mut red_seen = 0;
        let mut green_seen = 0;
        let mut blue_seen = 0;
        for round in game.rounds {
            red_seen = max(round.red, red_seen);
            green_seen = max(round.green, green_seen);
            blue_seen = max(round.blue, blue_seen);
        }
        let power = red_seen * green_seen * blue_seen;
        println!("The power for game: {} is {}", game.id, power);
        power_sum += power;
    }
    println!("Sum of game powers: {}", power_sum);
}

use core::fmt::Error;

use std::ops::Range;

use nom::{
    bytes::complete::take_until,
    character::complete::{self, line_ending, space1},
    multi::{many1, separated_list1},
    sequence::tuple,
    IResult, Parser,
};
use nom_supreme::{tag::complete::tag, ParserExt};

use std::collections::HashSet;

#[derive(Debug)]
struct Card {
    winning_numbers: HashSet<u32>,
    my_numbers: HashSet<u32>,
}

impl Card {
    fn score(&self) -> u32 {
        let power = self.winning_numbers.intersection(&self.my_numbers).count() as u32;

        match power.checked_sub(1) {
            Some(num) => 2u32.pow(num),
            None => 0,
        }
    }
}

fn main() -> Result<(), Error> {
    let file = include_str!("../../input1.txt");
    let result = process(file)?;
    println!("Solution - Part 1: {}", result); // 993500720
    Ok(())
}

#[derive(Debug)]
struct SeedMap {
    mappings: Vec<(Range<u64>, Range<u64>)>,
}

impl SeedMap {
    fn translate(&self, source: u64) -> u64 {
        let valid_mapping = self
            .mappings
            .iter()
            .find(|(source_range, _)| source_range.contains(&source));

        let Some((source_range, destination_range)) = valid_mapping else {
            return source;
        };

        let offset = source - source_range.start;

        destination_range.start + offset
    }
}

fn line(input: &str) -> IResult<&str, (Range<u64>, Range<u64>)> {
    let (input, (destination, source, num)) = tuple((
        complete::u64,
        complete::u64.preceded_by(tag(" ")),
        complete::u64.preceded_by(tag(" ")),
    ))(input)?;

    // dbg!(destination, num);
    Ok((
        input,
        (source..(source + num), destination..(destination + num)),
    ))
}
fn seed_map(input: &str) -> IResult<&str, SeedMap> {
    take_until("map:")
        .precedes(tag("map:"))
        .precedes(many1(line_ending.precedes(line)).map(|mappings| SeedMap { mappings }))
        .parse(input)
}

fn parse_seedmaps(input: &str) -> IResult<&str, (Vec<u64>, Vec<SeedMap>)> {
    let (input, seeds) = tag("seeds: ")
        .precedes(separated_list1(space1, complete::u64))
        .parse(input)?;
    let (input, maps) = many1(seed_map)(input)?;

    Ok((input, (seeds, maps)))
}

pub fn process(input: &str) -> Result<String, Error> {
    let (_, (seeds, maps)) = parse_seedmaps(input).expect("a valid parse");

    let locations = seeds
        .iter()
        .map(|seed| maps.iter().fold(*seed, |seed, map| map.translate(seed)))
        .collect::<Vec<u64>>();

    Ok(locations
        .iter()
        .min()
        .expect("should have a minimum location value")
        .to_string())
}

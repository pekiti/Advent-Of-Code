use core::fmt::Error;

use nom::{
    bytes::complete::tag,
    character::complete::{self, digit1, line_ending, space0, space1},
    multi::{fold_many1, separated_list1},
    sequence::{delimited, separated_pair, terminated, tuple},
    IResult, Parser,
};

use std::collections::{BTreeMap, HashSet};

#[derive(Debug)]
struct Card {
    winning_numbers: HashSet<u32>,
    my_numbers: HashSet<u32>,
}

impl Card {
    fn score(&self) -> u32 {
        match self.num_matches().checked_sub(1) {
            Some(num) => 2u32.pow(num as u32),
            None => 0,
        }
    }
    fn num_matches(&self) -> usize {
        self.winning_numbers
            .intersection(&self.my_numbers)
            .count()
    }
}

fn main() -> Result<(), Error> {
    let file = include_str!("../../input2.txt");
    let result = process(file)?;
    println!("Solution - Part 2: {}", result); // 5489600
    Ok(())
}

fn set(input: &str) -> IResult<&str, HashSet<u32>> {
    fold_many1(
        terminated(complete::u32, space0),
        HashSet::new,
        |mut acc: HashSet<_>, item| {
            acc.insert(item);
            acc
        },
    )(input)
}

fn card(input: &str) -> IResult<&str, Card> {
    let (input, _) = delimited(
        tuple((tag("Card"), space1)),
        digit1,
        tuple((tag(":"), space1)),
    )(input)?;
    separated_pair(set, tuple((tag("|"), space1)), set)
        .map(|(winning_numbers, my_numbers)| Card {
            winning_numbers,
            my_numbers,
        })
        .parse(input)
}
fn cards(input: &str) -> IResult<&str, Vec<Card>> {
    separated_list1(line_ending, card)(input)
}

pub fn process(
    input: &str,
) -> Result<String, Error> {
    let (_, card_data) =
        cards(&input).expect("a valid parse");
    let data = card_data
        .iter()
        .map(|card| card.num_matches())
        .collect::<Vec<_>>();

    let store = (0..card_data.len())
        .map(|index| (index, 1))
        .collect::<BTreeMap<usize, u32>>();
    let result = data
        .iter()
        .enumerate()
        .fold(store, |mut acc, (index, card_score)| {
            let to_add = *acc.get(&index).unwrap();

            for i in (index + 1)
                ..(index + 1 + *card_score as usize)
            {
                acc.entry(i).and_modify(|value| {
                    *value += to_add;
                });
            }
            acc
        })
        .values()
        .sum::<u32>();

    Ok(result.to_string())
}
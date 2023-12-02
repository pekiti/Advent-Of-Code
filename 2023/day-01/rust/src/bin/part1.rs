use core::fmt::Error;

fn main() -> Result<(), Error> {
    let file = include_str!("../../input1.txt");
    let result = process(file)?;
    println!("Solution - Part 1: {}", result);
    Ok(())
}

pub fn process(
    input: &str,
) -> Result<String, Error> {
    let output = input
        .lines()
        .map(|line| {
            let mut it =
                line.chars().filter_map(|character| {
                    character.to_digit(10)
                });
            let first =
                it.next().expect("should be a number");

            match it.last() {
                Some(num) => first * 10 + num,
                None => first * 10 + first,
            }
        })
        .sum::<u32>();

    Ok(output.to_string())
}
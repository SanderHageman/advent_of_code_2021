use itertools::Itertools;
use nom::{
  character::complete::digit1, character::complete::newline, combinator::all_consuming,
  combinator::map_res, multi::separated_list1, IResult,
};

type TParsed = Vec<TParsedSub>;
type TParsedSub = usize;

pub fn day(input: String) -> (usize, usize) {
  let parsed_input = parse(&input);
  let p1 = part_1(&parsed_input);
  let p2 = part_2(&parsed_input);
  (p1, p2)
}

fn part_1(input: &TParsed) -> usize {
  count_increases(input)
}

fn part_2(input: &TParsed) -> usize {
  count_increases(&input.windows(3).map(|w| w.iter().sum()).collect())
}

fn count_increases(i: &TParsed) -> usize {
  i.iter()
    .tuple_windows()
    .map(|(l, r)| if r > l { 1 } else { 0 })
    .sum()
}

fn parse(input: &str) -> TParsed {
  match try_parse(input) {
    Ok((_, v)) => v,
    Err(e) => panic!("error parsing: {:?}", e),
  }
}

fn try_parse(input: &str) -> IResult<&str, TParsed> {
  all_consuming(separated_list1(
    newline,
    map_res(digit1, |s: &str| s.parse()),
  ))(input)
}

#[test]
fn show_parse() {
  let input = parse(EXAMPLE_INPUT);
  println!("{:?}", input);
}

#[test]
fn test_example_1() {
  let input = parse(EXAMPLE_INPUT);
  assert_eq!(part_1(&input), 7)
}

#[test]
fn test_example_2() {
  let input = parse(EXAMPLE_INPUT);
  assert_eq!(part_2(&input), 5)
}

#[cfg(test)]
const EXAMPLE_INPUT: &str = "\
199
200
208
210
200
207
240
269
260
263";

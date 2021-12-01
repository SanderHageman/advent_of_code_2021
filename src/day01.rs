use itertools::Itertools;
use nom::{
  IResult,
  bytes::complete::{tag},
  combinator::map_res,
  character::complete::digit1,
  multi::separated_list1
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
  input.iter().tuple_windows().map(|(l, r)| if r > l {1} else {0}).sum()
}

fn part_2(input: &TParsed) -> usize {
  0
}

fn parse(input: &str) -> TParsed {
  match try_parse(input) {
    Ok((_, v)) => v,
    Err(e) => panic!("error parsing: {:?}", e)
  }
}

fn try_parse(input :&str) -> IResult<&str, TParsed> {
  separated_list1(tag("\n"),
    map_res(
      digit1,
      |s:&str| s.parse()
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
    assert_eq!(part_2(&input), 286)
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

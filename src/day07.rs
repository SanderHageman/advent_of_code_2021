use nom::character::complete::digit1;
use nom::{character::complete::char, combinator::*, multi::separated_list1, IResult};
use std::cmp::min;

type TParsed = Vec<TParsedSub>;
type TParsedSub = usize;

pub fn day(input: String) -> (usize, usize) {
  let parsed_input = parse(&input);
  let p1 = part_1(&parsed_input);
  let p2 = part_2(&parsed_input);
  (p1, p2)
}

fn part_1(input: &TParsed) -> usize {
  let i = {
    let mut i = input.to_owned();
    i.sort_unstable();
    i
  };
  let mid = i.len() / 2;
  let med = if i.len() % 2 == 1 {
    i[mid]
  } else {
    (i[mid] + i[mid - 1]) / 2
  } as isize;

  input
    .iter()
    .map(|x| ((*x as isize) - med).abs() as usize)
    .sum()
}

fn part_2(input: &TParsed) -> usize {
  let mean = input.iter().sum::<usize>() as f32 / input.len() as f32;

  let get_res = |m: isize| {
    input
      .iter()
      .map(|x| (0..=((*x as isize) - m).abs() as usize).sum::<usize>())
      .sum()
  };

  min(
    get_res(mean.floor() as isize),
    get_res(mean.ceil() as isize),
  )
}

fn parse<'a>(input: &'a str) -> TParsed {
  match try_parse(input) {
    Ok((_, v)) => v,
    Err(e) => panic!("error parsing: {:?}", e),
  }
}

fn try_parse<'a>(i: &'a str) -> IResult<&'a str, TParsed> {
  all_consuming(separated_list1(char(','), get_dig))(i)
}

fn get_dig<'a, A>(i: &'a str) -> IResult<&'a str, A>
where
  A: std::str::FromStr,
{
  map_res(digit1, |x: &str| x.parse())(i)
}

#[test]
fn show_parse_7() {
  let input = parse(EXAMPLE_INPUT);
  println!("{:?}", input);
}

#[test]
fn test_example_1_7() {
  let input = parse(EXAMPLE_INPUT);
  assert_eq!(part_1(&input), 37)
}

#[test]
fn test_example_2_7() {
  let input = parse(EXAMPLE_INPUT);
  assert_eq!(part_2(&input), 168)
}

#[cfg(test)]
const EXAMPLE_INPUT: &str = "\
16,1,2,0,4,2,7,1,2,14";

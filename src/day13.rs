use nom::bytes::complete::tag;
use nom::bytes::complete::take;
use nom::character::complete::char;
use nom::character::complete::digit1;
use nom::character::complete::newline;
use nom::sequence::pair;
use nom::sequence::preceded;
use nom::sequence::separated_pair;
use nom::{combinator::*, multi::separated_list1, IResult};
use std::collections::HashSet;
use vek::vec::Vec2;

type TParsed = (TParsedSub, TParsedSub);
type TParsedSub = Vec<TParsedSubSub>;
type TParsedSubSub = Vec2<isize>;

pub fn day<'a>(input: String) -> (usize, &'a str) {
  let parsed_input = parse(&input);
  let p1 = part_1(&parsed_input);
  part_2(&parsed_input);
  (p1, "PGHRKLKL")
}

fn part_1((coords, folds): &TParsed) -> usize {
  let mut coords = coords.clone();
  let fold = folds[0];
  for coord in &mut coords {
    if fold.x > 0 && fold.x < coord.x {
      coord.x = fold.x - (coord.x - fold.x);
    } else if fold.y > 0 && fold.y < coord.y {
      coord.y = fold.y - (coord.y - fold.y);
    }
  }

  coords.iter().collect::<HashSet<&Vec2<isize>>>().len()
}

fn part_2((coords, folds): &TParsed) {
  let mut coords = coords.clone();
  for fold in folds {
    for coord in &mut coords {
      if fold.x > 0 && fold.x < coord.x {
        coord.x = fold.x - (coord.x - fold.x);
      } else if fold.y > 0 && fold.y < coord.y {
        coord.y = fold.y - (coord.y - fold.y);
      }
    }
  }

  let ymax = coords.iter().map(|v| v.y).max().expect("No ymax") + 1;
  let xmax = coords.iter().map(|v| v.x).max().expect("No xmax") + 1;

  for y in 0..ymax {
    for x in 0..xmax {
      let c = if coords.iter().any(|v| v.x == x && v.y == y) {
        "#"
      } else {
        "."
      };
      print!("{}", c);
    }
    println!("");
  }
}

fn parse<'a>(input: &'a str) -> TParsed {
  match try_parse(input) {
    Ok((_, v)) => v,
    Err(e) => panic!("error parsing: {:?}", e),
  }
}

fn try_parse<'a>(i: &'a str) -> IResult<&'a str, TParsed> {
  all_consuming(separated_pair(
    parse_coords,
    pair(newline, newline),
    parse_folds,
  ))(i)
}

fn parse_coords<'a>(i: &'a str) -> IResult<&'a str, TParsedSub> {
  separated_list1(newline, parse_coord)(i)
}

fn parse_coord<'a>(i: &'a str) -> IResult<&'a str, TParsedSubSub> {
  map(
    separated_pair(get_dig, char(','), get_dig),
    |(x, y): (isize, isize)| Vec2::new(x, y),
  )(i)
}

fn parse_folds<'a>(i: &'a str) -> IResult<&'a str, TParsedSub> {
  separated_list1(newline, parse_fold)(i)
}

fn parse_fold<'a>(i: &'a str) -> IResult<&'a str, TParsedSubSub> {
  preceded(
    tag("fold along "),
    map(
      separated_pair(take(1u8), char('='), get_dig),
      |(axis, n): (&'a str, isize)| match axis {
        "x" => Vec2::new(n, 0),
        _ => Vec2::new(0, n),
      },
    ),
  )(i)
}

fn get_dig<'a, A>(i: &'a str) -> IResult<&'a str, A>
where
  A: std::str::FromStr,
{
  map_res(digit1, |x: &str| x.parse())(i)
}

#[test]
fn show_parse_13() {
  let input = parse(EXAMPLE_INPUT);
  println!("{:?}", input);
}

#[test]
fn test_example_1_13() {
  let input = parse(EXAMPLE_INPUT);
  assert_eq!(part_1(&input), 17)
}

#[test]
fn test_example_2_13() {
  let input = parse(EXAMPLE_INPUT);
  part_2(&input);
}

#[cfg(test)]
const EXAMPLE_INPUT: &str = "\
6,10
0,14
9,10
0,3
10,4
4,11
6,0
6,12
4,1
0,13
10,12
3,4
3,0
8,4
1,10
2,14
8,10
9,0

fold along y=7
fold along x=5";

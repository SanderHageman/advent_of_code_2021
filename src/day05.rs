use nom::bytes::complete::tag;
use nom::character::complete::digit1;
use nom::{character::complete::newline, combinator::*, multi::separated_list1, IResult};
use vek::num_traits::signum;
use vek::vec::Vec2;

type TParsed = Vec<TParsedSub>;
type TParsedSub = (TPoint, TPoint);
type TPoint = Vec2<i32>;

pub fn day(input: String) -> (usize, usize) {
  let parsed_input = parse(&input);
  let p1 = part_1(&parsed_input);
  let p2 = part_2(&parsed_input);
  (p1, p2)
}

fn count_overlapping<'a>(input: &'a TParsed) -> usize {
  let mut m = vec![vec![0; 1000]; 1000];
  for (from, to) in input {
    let dir = (to - from).map(|x| signum(x));
    let mut s = from.to_owned();
    m[s.y as usize][s.x as usize] += 1;
    while s != *to {
      s += dir;
      m[s.y as usize][s.x as usize] += 1;
    }
  }

  m.into_iter().flatten().filter(|n| *n > 1).count()
}

fn part_1(input: &TParsed) -> usize {
  let mut i = input.to_owned();
  i.retain(|(f, t)| f.x == t.x || f.y == t.y);
  count_overlapping(&i)
}

fn part_2(input: &TParsed) -> usize {
  count_overlapping(input)
}

fn parse<'a>(input: &'a str) -> TParsed {
  match try_parse(input) {
    Ok((_, v)) => v,
    Err(e) => panic!("error parsing: {:?}", e),
  }
}

fn try_parse<'a>(i: &'a str) -> IResult<&'a str, TParsed> {
  all_consuming(separated_list1(newline, parse_token))(i)
}

fn parse_token<'a>(i: &'a str) -> IResult<&'a str, TParsedSub> {
  map(
    separated_list1(
      tag(" -> "),
      map(
        separated_list1(tag(","), map_res(digit1, |d: &str| d.parse())),
        |v| Vec2::new(v[0], v[1]),
      ),
    ),
    |v| (v[0], v[1]),
  )(i)
}

#[test]
fn show_parse() {
  let input = parse(EXAMPLE_INPUT);
  println!("{:?}", input);
}

#[test]
fn test_example_1() {
  let input = parse(EXAMPLE_INPUT);
  assert_eq!(part_1(&input), 5)
}

#[test]
fn test_example_2_5() {
  let input = parse(EXAMPLE_INPUT);
  assert_eq!(part_2(&input), 12)
}

#[cfg(test)]
const EXAMPLE_INPUT: &str = "\
0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2";

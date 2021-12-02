use nom::{
  character::complete::{alpha1, digit1, newline, space1},
  combinator::all_consuming,
  combinator::*,
  multi::separated_list1,
  sequence::separated_pair,
  IResult,
};

type TParsed = Vec<TParsedSub>;
type TParsedSub = Token;

pub fn day(input: String) -> (usize, usize) {
  let parsed_input = parse(&input);
  let p1 = part_1(&parsed_input);
  let p2 = part_2(&parsed_input);
  (p1, p2)
}

fn part_1(input: &TParsed) -> usize {
  let (h, d) = input.iter().fold((0, 0), |(h, d), tok| match tok {
    Token::Forward(n) => (h + n, d),
    Token::Up(n) => (h, d - n),
    Token::Down(n) => (h, d + n),
  });

  h * d
}

fn part_2(input: &TParsed) -> usize {
  let (h, d, _) = input.iter().fold((0, 0, 0), |(h, d, a), tok| match tok {
    Token::Forward(n) => (h + n, d + a * n, a),
    Token::Up(n) => (h, d, a - n),
    Token::Down(n) => (h, d, a + n),
  });

  h * d
}

fn parse(input: &str) -> TParsed {
  match try_parse(input) {
    Ok((_, v)) => v,
    Err(e) => panic!("error parsing: {:?}", e),
  }
}

fn try_parse(input: &str) -> IResult<&str, TParsed> {
  all_consuming(separated_list1(newline, parse_token))(input)
}

#[derive(Debug, PartialEq)]
enum Token {
  Forward(usize),
  Up(usize),
  Down(usize),
}

fn parse_token<'a>(i: &'a str) -> IResult<&'a str, Token> {
  map(
    separated_pair(cut(alpha1), space1, cut(digit1)),
    |(tok, n)| match tok {
      "forward" => Token::Forward(n.parse().unwrap()),
      "up" => Token::Up(n.parse().unwrap()),
      "down" => Token::Down(n.parse().unwrap()),
      _ => unreachable!(),
    },
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
  assert_eq!(part_1(&input), 150)
}

#[test]
fn test_example_2() {
  let input = parse(EXAMPLE_INPUT);
  assert_eq!(part_2(&input), 900)
}

#[cfg(test)]
const EXAMPLE_INPUT: &str = "\
forward 5
down 5
forward 8
up 3
down 8
forward 2";

use itertools::Itertools;
use nom::bytes::complete::tag;
use nom::character::complete::alpha1;
use nom::character::complete::anychar;
use nom::character::complete::newline;
use nom::multi::many1;
use nom::sequence::pair;
use nom::sequence::separated_pair;
use nom::{combinator::*, multi::separated_list1, IResult};
use std::collections::HashMap;

type TParsed = (TParsedSub1, TParsedSub2);
type TParsedSub1 = Vec<char>;
type TParsedSub2 = HashMap<(char, char), char>;

pub fn day<'a>(input: String) -> (usize, usize) {
  let parsed_input = parse(&input);
  let p1 = part_1(&parsed_input);
  let p2 = part_2(&parsed_input);
  (p1, p2)
}

fn part_1((template, rules): &TParsed) -> usize {
  let mut res = template.clone();

  for _ in 0..10 {
    let mut res2 = vec![res[0]];
    for wnd in res.into_iter().tuple_windows() {
      res2.push(rules[&wnd]);
      res2.push(wnd.1);
    }
    res = res2;
  }

  let mut arr = vec![0; 26];
  for c in res {
    arr[(c as usize) - 65] += 1;
  }

  arr.iter().max().unwrap() - arr.iter().filter(|&&n| n > 0).min().unwrap()
}

fn part_2((template, rules): &TParsed) -> usize {
  let mut pairs = HashMap::new();

  for wnd in template.clone().into_iter().tuple_windows() {
    *pairs.entry(wnd).or_insert(0) += 1;
  }

  for _ in 0..40 {
    let mut npairs = HashMap::new();
    for (pair, n) in pairs {
      let c = rules[&pair];
      *npairs.entry((pair.0, c)).or_insert(0) += n;
      *npairs.entry((c, pair.1)).or_insert(0) += n;
    }
    pairs = npairs;
  }

  let mut arr = vec![0; 26];
  for (&(c1, c2), n) in pairs.iter() {
    arr[(c1 as usize) - 65] += n;
    arr[(c2 as usize) - 65] += n;
  }

  arr
    .iter_mut()
    .for_each(|n| *n = (*n as f64 / 2.0).ceil() as usize);

  arr.iter().max().unwrap() - arr.iter().filter(|&&n| n > 0).min().unwrap()
}

fn parse<'a>(input: &'a str) -> TParsed {
  match try_parse(input) {
    Ok((_, v)) => v,
    Err(e) => panic!("error parsing: {:?}", e),
  }
}

fn try_parse<'a>(i: &'a str) -> IResult<&'a str, TParsed> {
  all_consuming(separated_pair(parse_templ, many1(newline), parse_rules))(i)
}

fn parse_templ<'a>(i: &'a str) -> IResult<&'a str, TParsedSub1> {
  map(alpha1, |s: &str| s.chars().collect())(i)
}

fn parse_rules<'a>(i: &'a str) -> IResult<&'a str, TParsedSub2> {
  map(separated_list1(newline, parse_rule), |r| {
    HashMap::from_iter(r)
  })(i)
}

fn parse_rule<'a>(i: &'a str) -> IResult<&'a str, ((char, char), char)> {
  separated_pair(pair(anychar, anychar), tag(" -> "), anychar)(i)
}

#[test]
fn show_parse_14() {
  let input = parse(EXAMPLE_INPUT);
  println!("{:?}", input);
}

#[test]
fn test_example_1_14() {
  let input = parse(EXAMPLE_INPUT);
  assert_eq!(part_1(&input), 1588)
}

#[test]
fn test_example_2_14() {
  let input = parse(EXAMPLE_INPUT);
  assert_eq!(part_2(&input), 2188189693529)
}

#[cfg(test)]
const EXAMPLE_INPUT: &str = "\
NNCB

CH -> B
HH -> N
CB -> H
NH -> C
HB -> C
HC -> B
HN -> C
NN -> C
BH -> H
NC -> B
NB -> B
BN -> B
BB -> N
BC -> B
CC -> N
CN -> C";

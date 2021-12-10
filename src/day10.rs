use nom::character::complete::newline;
use nom::character::complete::satisfy;
use nom::multi::many1;
use nom::{combinator::*, multi::separated_list1, IResult};

type TParsed = Vec<TParsedSub>;
type TParsedSub = String;

pub fn day(input: String) -> (usize, usize) {
  let parsed_input = parse(&input);
  let p1 = part_1(&parsed_input);
  let p2 = part_2(&parsed_input);
  (p1, p2)
}

fn part_1(input: &TParsed) -> usize {
  let score = |c: char| match c {
    ')' => 3,
    ']' => 57,
    '}' => 1197,
    '>' => 25137,
    _ => 0,
  };

  input
    .iter()
    .filter_map(|s| s.chars().find(|c| score(*c) > 0))
    .map(score)
    .sum()
}

fn part_2(input: &TParsed) -> usize {
  let not_corrupt = |s: &&String| s.chars().all(|c| !")]}>".contains(c));

  let score = |acc: usize, c: char| {
    acc * 5
      + match c {
        '(' => 1,
        '[' => 2,
        '{' => 3,
        '<' => 4,
        _ => panic!("Unexpected"),
      }
  };

  let mut res = input
    .iter()
    .filter(not_corrupt)
    .map(|s| s.chars().fold(0, score))
    .collect::<Vec<usize>>();

  res.sort_unstable();
  res[res.len() / 2]
}

fn parse<'a>(input: &'a str) -> TParsed {
  match try_parse(input) {
    Ok((_, v)) => remove_pairs(&v),
    Err(e) => panic!("error parsing: {:?}", e),
  }
}

fn try_parse<'a>(i: &'a str) -> IResult<&'a str, TParsed> {
  all_consuming(separated_list1(newline, parse_line))(i)
}

fn parse_line<'a>(i: &'a str) -> IResult<&'a str, TParsedSub> {
  map(many1(satisfy(|c| c.is_ascii_punctuation())), |cs| {
    cs.iter().collect()
  })(i)
}

fn remove_pairs(input: &TParsed) -> TParsed {
  let mut res = input.clone();
  let pts = ["[]", "()", "<>", "{}"];

  for i in &mut res {
    let mut changed = true;
    while changed {
      changed = false;
      for pt in pts {
        let n = i.replace(pt, "");
        changed = changed || n != *i;
        *i = n;
      }
    }
  }

  res
}

#[test]
fn show_parse_10() {
  let input = parse(EXAMPLE_INPUT);
  println!("{:?}", input);
}

#[test]
fn test_example_1_10() {
  let input = parse(EXAMPLE_INPUT);
  assert_eq!(part_1(&input), 26397)
}

#[test]
fn test_example_2_10() {
  let input = parse(EXAMPLE_INPUT);
  assert_eq!(part_2(&input), 288957)
}

#[cfg(test)]
const EXAMPLE_INPUT: &str = "\
[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]";

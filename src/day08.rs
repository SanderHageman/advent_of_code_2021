use nom::bytes::complete::tag;
use nom::character::complete::alpha1;
use nom::character::complete::newline;
use nom::sequence::separated_pair;
use nom::{character::complete::char, combinator::*, multi::separated_list1, IResult};

type TParsed = Vec<TParsedSub>;
type TParsedSub = (TParsedLn, TParsedLn);
type TParsedLn = Vec<String>;

pub fn day(input: String) -> (usize, usize) {
  let parsed_input = parse(&input);
  let p1 = part_1(&parsed_input);
  let p2 = part_2(&parsed_input);
  (p1, p2)
}

fn part_1(input: &TParsed) -> usize {
  let dig_lens = [2, 3, 4, 7];
  input
    .iter()
    .map(|(_, o)| o)
    .flatten()
    .filter(|s| dig_lens.iter().any(|x| *x == s.len()))
    .count()
}

fn part_2(input: &TParsed) -> usize {
  let mut r = Vec::new();

  for (s, o) in input {
    let four = s.iter().find(|x| x.len() == 4).expect("No four found");
    let svn = s.iter().find(|x| x.len() == 3).expect("No seven found");

    fn overlapping(l: &str, r: &str) -> usize {
      l.chars().filter(|c| r.contains(*c)).count()
    }

    let mut res = 0;

    for dig in o {
      let n = match dig.len() {
        2 => 1,
        3 => 7,
        4 => 4,
        7 => 8,
        5 => {
          if overlapping(svn, dig) == 3 {
            3
          } else if overlapping(four, dig) == 3 {
            5
          } else {
            2
          }
        }
        6 => {
          if overlapping(svn, dig) == 2 {
            6
          } else if overlapping(four, dig) == 4 {
            9
          } else {
            0
          }
        }
        _ => panic!("Unexpected length"),
      };

      res = res * 10 + n;
    }

    r.push(res);
  }

  r.iter().sum()
}

fn parse<'a>(input: &'a str) -> TParsed {
  match try_parse(input) {
    Ok((_, v)) => v,
    Err(e) => panic!("error parsing: {:?}", e),
  }
}

fn try_parse<'a>(i: &'a str) -> IResult<&'a str, TParsed> {
  all_consuming(separated_list1(newline, parse_line))(i)
}

fn parse_line<'a>(i: &'a str) -> IResult<&'a str, TParsedSub> {
  separated_pair(parse_sq, tag(" | "), parse_sq)(i)
}

fn parse_sq<'a>(i: &'a str) -> IResult<&'a str, TParsedLn> {
  separated_list1(char(' '), map(alpha1, |s: &str| s.to_string()))(i)
}

#[test]
fn show_parse_8() {
  let input = parse(EXAMPLE_INPUT);
  println!("{:?}", input);
}

#[test]
fn test_example_1_8() {
  let input = parse(EXAMPLE_INPUT);
  assert_eq!(part_1(&input), 26)
}

#[test]
fn test_example_2_8() {
  let input = parse(EXAMPLE_INPUT);
  assert_eq!(part_2(&input), 61229)
}

#[cfg(test)]
const EXAMPLE_INPUT: &str = "\
be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce";

use nom::character::complete::digit1;
use nom::{character::complete::char, combinator::*, multi::separated_list1, IResult};

type TParsed = Vec<TParsedSub>;
type TParsedSub = usize;

pub fn day(input: String) -> (usize, usize) {
  let parsed_input = parse(&input);
  let p1 = part_1(&parsed_input);
  let p2 = part_2(&parsed_input);
  (p1, p2)
}

fn part_1(input: &TParsed) -> usize {
  get_n_fishies(input, 80)
}

fn part_2(input: &TParsed) -> usize {
  get_n_fishies(input, 256)
}

/*
      /`·.¸
     /¸...¸`:·
 ¸.·´  ¸   `·.¸.·´)
: © ):´;      ¸  {
 `·.¸ `·  ¸.·´\`·¸)
     `\\´´\¸.·´
*/

fn get_n_fishies(input: &TParsed, n_days: usize) -> usize {
  let mut fishies = [0; 9];
  for i in input {
    fishies[*i] += 1;
  }

  for _ in 0..n_days {
    let new_fishies = fishies[0];
    for i in 1..9 {
      fishies[i - 1] = fishies[i];
    }
    fishies[6] += new_fishies;
    fishies[8] = new_fishies;
  }

  fishies.iter().sum()
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
fn show_parse_6() {
  let input = parse(EXAMPLE_INPUT);
  println!("{:?}", input);
}

#[test]
fn test_example_1_6() {
  let input = parse(EXAMPLE_INPUT);
  assert_eq!(part_1(&input), 5934)
}

#[test]
fn test_example_2_6() {
  let input = parse(EXAMPLE_INPUT);
  assert_eq!(part_2(&input), 26984457539)
}

#[cfg(test)]
const EXAMPLE_INPUT: &str = "\
3,4,3,1,2";

use nom::{
  bytes::complete::take,
  character::complete::newline,
  combinator::*,
  multi::{many1, separated_list1},
  IResult,
};

type TParsed = Vec<TParsedSub>;
type TParsedSub = Vec<usize>;

pub fn day(input: String) -> (usize, usize) {
  let parsed_input = parse(&input);
  let p1 = part_1(&parsed_input);
  let p2 = part_2(&parsed_input);
  (p1, p2)
}

// mult by 2 and compare with full list length
// to get around rounding issues
fn get_most_common(full_list_len: usize, v: &Vec<usize>) -> usize {
  (full_list_len <= v.iter().sum::<usize>() * 2) as usize
}

fn get_least_common(full_list_len: usize, v: &Vec<usize>) -> usize {
  (v.iter().sum::<usize>() * 2 < full_list_len) as usize
}

fn to_n(v: &Vec<usize>) -> usize {
  v.iter().fold(0, |a, b| a * 2 + b)
}

fn part_1(input: &TParsed) -> usize {
  let tpsd = transpose(input.to_owned());
  let cmp = input.len();

  let g_get_n = |v: &Vec<usize>| get_most_common(cmp, v);
  let e_get_n = |v: &Vec<usize>| get_least_common(cmp, v);

  let gamma = tpsd.iter().map(g_get_n).collect::<Vec<usize>>();
  let epsilon = tpsd.iter().map(e_get_n).collect::<Vec<usize>>();

  to_n(&gamma) * to_n(&epsilon)
}

fn part_2(input: &TParsed) -> usize {
  let filter = |v: &mut TParsed, f: fn(cmp: usize, v: &Vec<usize>) -> usize| {
    for p in 0..input[0].len() {
      let bit_criteria = f(v.len(), &transpose(v.to_owned())[p]);
      v.retain(|v| v[p] == bit_criteria);
      if v.len() == 1 {
        break;
      }
    }
  };

  let mut oxygen_lst = input.to_owned();
  let mut scrubber_lst = input.to_owned();

  filter(&mut oxygen_lst, get_most_common);
  filter(&mut scrubber_lst, get_least_common);

  to_n(&oxygen_lst[0]) * to_n(&scrubber_lst[0])
}

// https://stackoverflow.com/questions/64498617/how-to-transpose-a-vector-of-vectors-in-rust
fn transpose<T>(v: Vec<Vec<T>>) -> Vec<Vec<T>> {
  assert!(!v.is_empty());
  let len = v[0].len();
  let mut iters: Vec<_> = v.into_iter().map(|n| n.into_iter()).collect();
  (0..len)
    .map(|_| {
      iters
        .iter_mut()
        .map(|n| n.next().unwrap())
        .collect::<Vec<T>>()
    })
    .collect()
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
  many1(map_res(take(1usize), |s: &str| s.parse()))(i)
}

#[test]
fn show_parse() {
  let input = parse(EXAMPLE_INPUT);
  println!("{:?}", input);
}

#[test]
fn test_example_1() {
  let input = parse(EXAMPLE_INPUT);
  assert_eq!(part_1(&input), 198)
}

#[test]
fn test_example_2() {
  let input = parse(EXAMPLE_INPUT);
  assert_eq!(part_2(&input), 230)
}

#[cfg(test)]
const EXAMPLE_INPUT: &str = "\
00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010";

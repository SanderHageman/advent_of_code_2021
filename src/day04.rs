use nom::character::complete::{self, digit1};
use nom::multi::{many0, many1};
use nom::sequence::pair;
use nom::sequence::preceded;
use nom::{character::complete::newline, combinator::*, multi::separated_list1, IResult};

type TParsed = (Vec<usize>, Vec<TParsedSub>);
type TParsedSub = Vec<Vec<usize>>;

pub fn day(input: String) -> (usize, usize) {
  let parsed_input = parse(&input);
  let p1 = part_1(&parsed_input);
  let p2 = part_2(&parsed_input);
  (p1, p2)
}

fn part_1(input: &TParsed) -> usize {
  let (ns, i) = input;
  let j = i
    .iter()
    .map(|b| final_score(ns, b))
    .min_by(|(x1, _), (x2, _)| x1.cmp(x2));
  j.unwrap().1
}

fn part_2(input: &TParsed) -> usize {
  let (ns, i) = input;
  let j = i
    .iter()
    .map(|b| final_score(ns, b))
    .max_by(|(x1, _), (x2, _)| x1.cmp(x2));
  j.unwrap().1
}

fn final_score(xs: &Vec<usize>, board: &TParsedSub) -> (usize, usize) {
  let mut ys = Vec::new();
  for x in xs {
    ys.push(*x);
    for ln in board {
      if ln.iter().all(|y| ys.contains(y)) {
        let res: usize = board.concat().iter().filter(|y| !ys.contains(y)).sum();
        return (ys.len(), res * x);
      }
    }
    for ln in transpose(board.to_owned()) {
      if ln.iter().all(|y| ys.contains(y)) {
        let res: usize = board.concat().iter().filter(|y| !ys.contains(y)).sum();
        return (ys.len(), res * x);
      }
    }
  }

  panic!("No score found");
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

fn get_dig<'a, A>(i: &'a str) -> IResult<&'a str, A>
where
  A: std::str::FromStr,
{
  map_res(digit1, |x: &str| x.parse())(i)
}

fn try_parse<'a>(i: &'a str) -> IResult<&'a str, TParsed> {
  let a = separated_list1(complete::char(','), get_dig);
  let b = preceded(
    pair(pair(newline, newline), many0(complete::char(' '))),
    separated_list1(
      pair(pair(newline, newline), many0(complete::char(' '))),
      parse_token,
    ),
  );
  all_consuming(pair(a, b))(i)
}

fn parse_token<'a>(i: &'a str) -> IResult<&'a str, TParsedSub> {
  separated_list1(
    pair(newline, many0(complete::char(' '))),
    separated_list1(many1(complete::char(' ')), get_dig),
  )(i)
}

#[test]
fn show_parse_4() {
  let input = parse(EXAMPLE_INPUT);
  println!("{:?}", input);
}

#[test]
fn test_example_1_4() {
  let input = parse(EXAMPLE_INPUT);
  assert_eq!(part_1(&input), 4512)
}

#[test]
fn test_example_2_4() {
  let input = parse(EXAMPLE_INPUT);
  assert_eq!(part_2(&input), 1924)
}

#[cfg(test)]
const EXAMPLE_INPUT: &str = "\
7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

 2 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7";

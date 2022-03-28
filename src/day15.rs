use nom::bytes::complete::take;
use nom::character::complete::newline;
use nom::multi::many1;
use nom::{combinator::*, multi::separated_list1, IResult};
use priority_queue::PriorityQueue;
use std::cmp::Reverse;

type TParsed = Vec<TParsedSub>;
type TParsedSub = Vec<usize>;

pub fn day<'a>(input: String) -> (usize, usize) {
  let parsed_input = parse(&input);
  let p1 = part_1(&parsed_input);
  let p2 = part_2(&parsed_input);
  (p1, p2)
}

fn part_1(input: &TParsed) -> usize {
  let mut pq = PriorityQueue::<(_, _), Reverse<_>>::new();
  pq.push((0, 0), Reverse(0));

  let xlim = input[0].len() - 1;
  let ylim = input.len() - 1;

  let mut v = vec![vec![false; xlim + 1]; ylim + 1];

  while !pq.is_empty() {
    let ((x, y), p) = pq.pop().unwrap();

    if x == xlim && y == ylim {
      return p.0;
    }

    v[y][x] = true;

    if x > 0 && !v[y][x - 1] {
      pq.push_increase((x - 1, y), Reverse(p.0 + input[y][x - 1]));
    }
    if x < xlim && !v[y][x + 1] {
      pq.push_increase((x + 1, y), Reverse(p.0 + input[y][x + 1]));
    }
    if y > 0 && !v[y - 1][x] {
      pq.push_increase((x, y - 1), Reverse(p.0 + input[y - 1][x]));
    }
    if y < ylim && !v[y + 1][x] {
      pq.push_increase((x, y + 1), Reverse(p.0 + input[y + 1][x]));
    }
  }

  unimplemented!()
}

fn part_2(input: &TParsed) -> usize {
  let mut pq = PriorityQueue::<(_, _), Reverse<_>>::new();
  pq.push((0, 0), Reverse(0));

  let rlim = input.len() - 1;

  let xlim = input[0].len() * 5 - 1;
  let ylim = input.len() * 5 - 1;

  let mut v = vec![vec![false; xlim + 1]; ylim + 1];

  let get_weight = |(x, y): (usize, usize)| {
    let lim = rlim + 1;
    let offset = (x / lim) + (y / lim);
    ((input[y % lim][x % lim] + offset - 1) % 9) + 1
  };

  while !pq.is_empty() {
    let ((x, y), p) = pq.pop().unwrap();

    if x == xlim && y == ylim {
      return p.0;
    }

    v[y][x] = true;

    if x > 0 && !v[y][x - 1] {
      let t = (x - 1, y);
      pq.push_increase(t, Reverse(p.0 + get_weight(t)));
    }
    if x < xlim && !v[y][x + 1] {
      let t = (x + 1, y);
      pq.push_increase(t, Reverse(p.0 + get_weight(t)));
    }
    if y > 0 && !v[y - 1][x] {
      let t = (x, y - 1);
      pq.push_increase(t, Reverse(p.0 + get_weight(t)));
    }
    if y < ylim && !v[y + 1][x] {
      let t = (x, y + 1);
      pq.push_increase(t, Reverse(p.0 + get_weight(t)));
    }
  }

  panic!("Unable to find destination!")
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
  many1(map_res(take(1u8), |s: &str| s.parse()))(i)
}

#[test]
fn show_parse_15() {
  let input = parse(EXAMPLE_INPUT);
  println!("{:?}", input);
}

#[test]
fn test_example_1_15() {
  let input = parse(EXAMPLE_INPUT);
  assert_eq!(part_1(&input), 40)
}

#[test]
fn test_example_2_15() {
  let input = parse(EXAMPLE_INPUT);
  assert_eq!(part_2(&input), 315)
}

#[cfg(test)]
const EXAMPLE_INPUT: &str = "\
1163751742
1381373672
2136511328
3694931569
7463417111
1319128137
1359912421
3125421639
1293138521
2311944581";

use itertools::Itertools;
use nom::bytes::complete::take;
use nom::character::complete::newline;
use nom::multi::many1;
use nom::{combinator::*, multi::separated_list1, IResult};

type TParsed = Vec<TParsedSub>;
type TParsedSub = Vec<u8>;

pub fn day(input: String) -> (usize, usize) {
  let parsed_input = parse(&input);
  let p1 = part_1(&parsed_input);
  let p2 = part_2(&parsed_input);
  (p1, p2)
}

fn part_1(input: &TParsed) -> usize {
  let mut map = input.to_owned();
  let mut flash_cnt = 0;

  for _ in 0..100 {
    step_1(&mut map);
    loop {
      match step_2(&mut map) {
        0 => break,
        n => flash_cnt += n,
      }
    }
  }

  flash_cnt
}

fn part_2(input: &TParsed) -> usize {
  let mut map = input.to_owned();
  let mut steps = 0;

  loop {
    steps += 1;
    let mut flash_cnt = 0;
    step_1(&mut map);
    loop {
      match step_2(&mut map) {
        0 => break,
        n => flash_cnt += n,
      }
    }

    if flash_cnt == 100 {
      break;
    }
  }

  steps
}

fn get_adjacent((y, x): (usize, usize)) -> impl Iterator<Item = (usize, usize)> {
  let satisfy = |n| n >= 0 && n < 10;
  let map_to_adjacent = move |p: (isize, isize)| match p {
    (0, 0) => None,
    (m, n) => {
      let (o, p) = (m + y as isize, n + x as isize);
      if satisfy(o) && satisfy(p) {
        Some((o as usize, p as usize))
      } else {
        None
      }
    }
  };
  (-1..2).cartesian_product(-1..2).filter_map(map_to_adjacent)
}

fn step_1(map: &mut TParsed) {
  map.iter_mut().flatten().for_each(|i| *i += 1);
}

fn step_2(map: &mut TParsed) -> usize {
  let mut flashed = [[0; 10]; 10];

  for (y, ln) in map.iter_mut().enumerate() {
    for (x, n) in ln.iter_mut().enumerate() {
      if *n > 9 {
        *n = 0;
        flashed[y][x] = 1;
      }
    }
  }

  for (y, ln) in map.iter_mut().enumerate() {
    for (x, n) in ln.iter_mut().enumerate() {
      if *n > 0 {
        for (o, p) in get_adjacent((y, x)) {
          *n += flashed[o][p];
        }
      }
    }
  }

  flashed.iter().flatten().sum::<u8>() as usize
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
fn show_parse_11() {
  let input = parse(EXAMPLE_INPUT);
  println!("{:?}", input);
}

#[test]
fn test_example_1_11() {
  let input = parse(EXAMPLE_INPUT);
  assert_eq!(part_1(&input), 1656)
}

#[test]
fn test_example_2_11() {
  let input = parse(EXAMPLE_INPUT);
  assert_eq!(part_2(&input), 195)
}

#[cfg(test)]
const EXAMPLE_INPUT: &str = "\
5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526";

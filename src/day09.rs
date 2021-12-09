use nom::bytes::complete::take;
use nom::character::complete::newline;
use nom::multi::many1;
use nom::{combinator::*, multi::separated_list1, IResult};
use std::collections::HashSet;

type TParsed = Vec<TParsedSub>;
type TParsedSub = Vec<usize>;

pub fn day(input: String) -> (usize, usize) {
  let parsed_input = parse(&input);
  let p1 = part_1(&parsed_input);
  let p2 = part_2(&parsed_input);
  (p1, p2)
}

fn part_1(input: &TParsed) -> usize {
  get_low_points(input)
    .iter()
    .map(|(y, x)| input[*y][*x] + 1)
    .sum()
}

fn part_2(input: &TParsed) -> usize {
  let mut lps: Vec<usize> = get_low_points(input)
    .iter()
    .map(|p| capture_nbs(input, *p, &mut HashSet::new()).len() + 1)
    .collect();
  lps.sort_unstable_by(|l, r| r.cmp(l));
  lps[..3].iter().product()
}

fn get_nbs((y, x): (usize, usize), maxy: usize, maxx: usize) -> HashSet<(usize, usize)> {
  let mut res = HashSet::new();
  if y > 0 {
    res.insert((y - 1, x));
  }
  if x > 0 {
    res.insert((y, x - 1));
  }
  if y < maxy - 1 {
    res.insert((y + 1, x));
  }
  if x < maxx - 1 {
    res.insert((y, x + 1));
  }
  res
}

fn get_low_points(input: &TParsed) -> Vec<(usize, usize)> {
  let mut res = Vec::new();

  for (i, ln) in input.iter().enumerate() {
    for (j, n) in ln.iter().enumerate() {
      let nbs = get_nbs((i, j), input.len(), ln.len());
      if nbs.iter().all(|(y, x)| *n < input[*y][*x]) {
        res.push((i, j));
      }
    }
  }
  res
}

fn capture_nbs(
  input: &TParsed,
  (sy, sx): (usize, usize),
  seen: &mut HashSet<(usize, usize)>,
) -> HashSet<(usize, usize)> {
  seen.insert((sy, sx));

  let g = |(y, x): (usize, usize)| input[y][x];

  let nbs = get_nbs((sy, sx), input.len(), input[0].len())
    .difference(&seen)
    .filter_map(|p| {
      if g((sy, sx)) <= g(*p) && g(*p) != 9 {
        Some(*p)
      } else {
        None
      }
    })
    .collect::<HashSet<(usize, usize)>>();

  nbs
    .iter()
    .map(|p| capture_nbs(input, *p, seen))
    .fold(nbs.clone(), |mut acc, v| {
      acc.extend(v);
      acc
    })
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
fn show_parse_9() {
  let input = parse(EXAMPLE_INPUT);
  println!("{:?}", input);
}

#[test]
fn test_example_1_9() {
  let input = parse(EXAMPLE_INPUT);
  assert_eq!(part_1(&input), 15)
}

#[test]
fn test_example_2_9() {
  let input = parse(EXAMPLE_INPUT);
  assert_eq!(part_2(&input), 1134)
}

#[cfg(test)]
const EXAMPLE_INPUT: &str = "\
2199943210
3987894921
9856789892
8767896789
9899965678";

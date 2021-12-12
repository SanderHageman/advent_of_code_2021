use nom::character::complete::alpha1;
use nom::character::complete::char;
use nom::character::complete::newline;
use nom::sequence::separated_pair;
use nom::{combinator::*, multi::separated_list1, IResult};
use std::collections::HashMap;
use std::collections::HashSet;

type TParsed<'a> = HashMap<&'a str, HashSet<&'a str>>;

pub fn day(input: String) -> (usize, usize) {
  let parsed_input = parse(&input);
  let p1 = part_1(&parsed_input);
  let p2 = part_2(&parsed_input);
  (p1, p2)
}

fn is_small(s: &str) -> bool {
  s.chars().any(|c| c.is_ascii_lowercase())
}

fn part_1(input: &TParsed) -> usize {
  let start = "start";
  let end = "end";

  let mut visited: HashMap<&str, bool> = input
    .iter()
    .filter(|(k, _)| is_small(k))
    .map(|(k, _)| (*k, false))
    .collect();

  fn dfs<'a>(
    u: &'a str,
    v: &'a str,
    current_path: &mut Vec<&'a str>,
    visited: &mut HashMap<&'a str, bool>,
    paths: &mut Vec<Vec<&'a str>>,
    input: &'a TParsed,
  ) {
    if is_small(u) {
      if visited[u] {
        return;
      } else {
        *visited.get_mut(u).expect("small cave not found") = true;
      }
    }
    current_path.push(u);

    if u == v {
      paths.push(current_path.clone());
      *visited.get_mut(u).expect("small cave not found") = false;
      current_path.pop();
      return;
    }

    let nexts = input.get(u).expect("Unable to find next paths");
    for next in nexts {
      dfs(*next, v, current_path, visited, paths, input);
    }

    current_path.pop();

    if is_small(u) {
      *visited.get_mut(u).expect("small cave not found") = false;
    }
  }

  let mut paths = Vec::new();

  dfs(start, end, &mut Vec::new(), &mut visited, &mut paths, input);

  paths.len()
}

fn part_2(input: &TParsed) -> usize {
  let start = "start";
  let end = "end";

  let mut visited: HashMap<&str, bool> = input
    .iter()
    .filter(|(k, _)| is_small(k))
    .map(|(k, _)| (*k, false))
    .collect();

  fn dfs<'a>(
    u: &'a str,
    v: &'a str,
    visited_twice: bool,
    current_path: &mut Vec<&'a str>,
    visited: &mut HashMap<&'a str, bool>,
    paths: &mut Vec<Vec<&'a str>>,
    input: &'a TParsed,
  ) {
    let small = is_small(u);
    let terminator = u == "start" || u == "end";
    if small && visited[u] {
      return;
    }

    current_path.push(u);

    if u == v {
      paths.push(current_path.clone());
      *visited.get_mut(u).expect("small cave not found") = false;
      current_path.pop();
      return;
    }

    let nexts = input.get(u).expect("Unable to find next paths");

    if !terminator && small && !visited_twice {
      for next in nexts {
        dfs(*next, v, true, current_path, visited, paths, input);
      }
    }

    if small {
      *visited.get_mut(u).expect("small cave not found") = true;
    }

    for next in nexts {
      dfs(*next, v, visited_twice, current_path, visited, paths, input);
    }

    current_path.pop();

    if is_small(u) {
      *visited.get_mut(u).expect("small cave not found") = false;
    }
  }

  let mut paths = Vec::new();

  dfs(
    start,
    end,
    false,
    &mut Vec::new(),
    &mut visited,
    &mut paths,
    input,
  );

  paths.sort_unstable();
  paths.dedup();
  paths.len()
}

fn parse<'a>(input: &'a str) -> TParsed {
  match try_parse(input) {
    Ok((_, v)) => make_map(v),
    Err(e) => panic!("error parsing: {:?}", e),
  }
}

fn make_map<'a>(input: Vec<(&'a str, &'a str)>) -> TParsed {
  let mut res = HashMap::new();

  for i in input {
    res.entry(i.0).or_insert(HashSet::new()).insert(i.1);
    res.entry(i.1).or_insert(HashSet::new()).insert(i.0);
  }

  res
}

fn try_parse<'a>(i: &'a str) -> IResult<&'a str, Vec<(&'a str, &'a str)>> {
  all_consuming(separated_list1(newline, parse_line))(i)
}

fn parse_line<'a>(i: &'a str) -> IResult<&'a str, (&'a str, &'a str)> {
  separated_pair(alpha1, char('-'), alpha1)(i)
}

#[test]
fn show_parse_12() {
  let input = parse(EXAMPLE_INPUT);
  println!("{:?}", input);
}

#[test]
fn test_example_1_12() {
  assert_eq!(part_1(&parse(EXAMPLE_INPUT)), 10);
  assert_eq!(part_1(&parse(EXAMPLE_INPUT2)), 19);
  assert_eq!(part_1(&parse(EXAMPLE_INPUT3)), 226);
}

#[test]
fn test_example_2_12() {
  assert_eq!(part_2(&parse(EXAMPLE_INPUT)), 36);
  assert_eq!(part_2(&parse(EXAMPLE_INPUT2)), 103);
  assert_eq!(part_2(&parse(EXAMPLE_INPUT3)), 3509);
}

#[cfg(test)]
const EXAMPLE_INPUT: &str = "\
start-A
start-b
A-c
A-b
b-d
A-end
b-end";

#[cfg(test)]
const EXAMPLE_INPUT2: &str = "\
dc-end
HN-start
start-kj
dc-start
dc-HN
LN-dc
HN-end
kj-sa
kj-HN
kj-dc";

#[cfg(test)]
const EXAMPLE_INPUT3: &str = "\
fs-end
he-DX
fs-he
start-DX
pj-DX
end-zg
zg-sl
zg-pj
pj-he
RW-he
fs-DX
pj-RW
zg-RW
start-pj
he-WI
zg-he
pj-fs
start-RW";

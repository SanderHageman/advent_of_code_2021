use itertools::Itertools;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::digit1;
use nom::character::complete::newline;
use nom::sequence::delimited;
use nom::sequence::separated_pair;
use nom::{combinator::*, multi::separated_list1, IResult};
use std::fmt;

type TParsed = Vec<TParsedSub>;
type TParsedSub = Tree;

pub fn day<'a>(input: String) -> (usize, usize) {
  let parsed_input = parse(&input);
  let p1 = part_1(&parsed_input);
  let p2 = part_2(&parsed_input);
  (p1, p2)
}

fn part_1(input: &TParsed) -> usize {
  let mut res = input[0].to_owned();

  for i in 1..input.len() {
    res = Tree::add(res, input[i].to_owned());
  }

  res.magnitude()
}

fn part_2(input: &TParsed) -> usize {
  input
    .into_iter()
    .tuple_combinations()
    .map(|(l, r)| {
      std::cmp::max(
        Tree::add(l.to_owned(), r.to_owned()).magnitude(),
        Tree::add(r.to_owned(), l.to_owned()).magnitude(),
      )
    })
    .max()
    .unwrap()
}

#[derive(Debug, Clone)]
enum Tree {
  Leaf(usize),
  Node { l: Box<Tree>, r: Box<Tree> },
}

impl Tree {
  fn add(l: Tree, r: Tree) -> Tree {
    let mut tree = Tree::Node {
      l: Box::new(l),
      r: Box::new(r),
    };

    tree.try_explode();
    while tree.try_split() {
      tree.try_explode();
    }

    tree
  }

  fn is_node(&self) -> bool {
    match self {
      Tree::Node { l: _, r: _ } => true,
      _ => false,
    }
  }

  fn is_value_node(&self) -> bool {
    match self {
      Tree::Node { l, r } => !(l.is_node() || r.is_node()),
      _ => false,
    }
  }

  fn try_explode(&mut self) {
    while self.height() > 4 {
      self.explode(0);
    }
  }

  fn try_split(&mut self) -> bool {
    match self {
      Tree::Leaf(_) => false,
      Tree::Node {
        ref mut l,
        ref mut r,
      } => {
        if l.is_node() {
          if l.try_split() {
            return true;
          }
        } else if l.value() > 9 {
          **l = l.split();
          return true;
        }
        if r.is_node() {
          if r.try_split() {
            return true;
          }
        } else if r.value() > 9 {
          **r = r.split();
          return true;
        }
        false
      }
    }
  }

  fn split(&self) -> Tree {
    match self {
      Tree::Leaf(v) => {
        let half = v / 2;
        let l = Box::new(Tree::Leaf(half));
        let r = Box::new(Tree::Leaf(v - half));
        Tree::Node { l: l, r: r }
      }
      _ => panic!("Can't split Node"),
    }
  }

  fn explode(&mut self, depth: u8) -> Option<(usize, usize)> {
    let mut res = (0, 0);
    let (l, r) = match self {
      Tree::Leaf(_) => return None,
      Tree::Node {
        ref mut l,
        ref mut r,
      } => (l, r),
    };

    if depth >= 3 && (l.is_value_node() || r.is_value_node()) {
      if l.is_value_node() {
        let (cl, cr) = l.values();
        r.send_left(cr);
        **l = Tree::Leaf(0);
        res.0 = cl;
      }

      if r.is_value_node() {
        let (cl, cr) = r.values();
        l.send_right(cl);
        **r = Tree::Leaf(0);
        res.1 = cr;
      }
    } else {
      if let Some((cl, cr)) = l.explode(depth + 1) {
        r.send_left(cr);
        res.0 = cl;
      }

      if let Some((cl, cr)) = r.explode(depth + 1) {
        l.send_right(cl);
        res.1 = cr;
      }
    }

    if res.0 > 0 || res.1 > 0 {
      Some(res)
    } else {
      None
    }
  }

  fn send_left(&mut self, val: usize) {
    match self {
      Tree::Leaf(ref mut v) => *v += val,
      Tree::Node { ref mut l, r: _ } => (*l).send_left(val),
    }
  }

  fn send_right(&mut self, val: usize) {
    match self {
      Tree::Leaf(ref mut v) => *v += val,
      Tree::Node { l: _, ref mut r } => (*r).send_right(val),
    }
  }

  fn value(&self) -> usize {
    match self {
      Tree::Leaf(v) => *v,
      Tree::Node { l: _, r: _ } => panic!("can't get node value"),
    }
  }

  fn values(&self) -> (usize, usize) {
    match self {
      Tree::Leaf(_) => panic!("can't get leaf values"),
      Tree::Node { l, r } => (l.value(), r.value()),
    }
  }

  fn magnitude(&self) -> usize {
    match self {
      Tree::Leaf(v) => *v,
      Tree::Node { l, r } => 3 * l.magnitude() + 2 * r.magnitude(),
    }
  }

  fn height(&self) -> usize {
    match self {
      Tree::Leaf(_) => 0,
      Tree::Node { l, r } => 1 + std::cmp::max(l.height(), r.height()),
    }
  }

  fn parse_either<'a>(i: &'a str) -> IResult<&'a str, TParsedSub> {
    alt((Tree::parse_node, Tree::parse_leaf))(i)
  }

  fn parse_node<'a>(i: &'a str) -> IResult<&'a str, TParsedSub> {
    map(
      delimited(
        tag("["),
        separated_pair(Tree::parse_either, tag(","), Tree::parse_either),
        tag("]"),
      ),
      |(l, r)| Tree::Node {
        l: Box::new(l),
        r: Box::new(r),
      },
    )(i)
  }

  fn parse_leaf<'a>(i: &'a str) -> IResult<&'a str, TParsedSub> {
    map(get_dig, |d| Tree::Leaf(d))(i)
  }
}

impl fmt::Display for Tree {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      Tree::Leaf(v) => write!(f, "{}", v),
      Tree::Node { l, r } => write!(f, "[{},{}]", l, r),
    }
  }
}

fn parse<'a>(input: &'a str) -> TParsed {
  match try_parse(input) {
    Ok((_, v)) => v,
    Err(e) => panic!("error parsing: {:?}", e),
  }
}

fn try_parse<'a>(i: &'a str) -> IResult<&'a str, TParsed> {
  all_consuming(separated_list1(newline, Tree::parse_either))(i)
}

fn get_dig<'a, A>(i: &'a str) -> IResult<&'a str, A>
where
  A: std::str::FromStr,
{
  map_res(digit1, |x: &str| x.parse())(i)
}

#[test]
fn show_parse_18() {
  let input = parse(EXAMPLE_INPUT);
  for ln in input {
    println!("{:?}", ln);
  }
}

#[test]
fn test_example_1_18() {
  let input = parse(EXAMPLE_INPUT);
  assert_eq!(part_1(&input), 4140)
}

#[test]
fn test_example_2_18() {
  let input = parse(EXAMPLE_INPUT);
  assert_eq!(part_2(&input), 3993)
}

#[cfg(test)]
const EXAMPLE_INPUT: &str = "\
[[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]
[[[5,[2,8]],4],[5,[[9,9],0]]]
[6,[[[6,2],[5,6]],[[7,6],[4,7]]]]
[[[6,[0,7]],[0,9]],[4,[9,[9,0]]]]
[[[7,[6,4]],[3,[1,3]]],[[[5,5],1],9]]
[[6,[[7,3],[3,2]]],[[[3,8],[5,7]],4]]
[[[[5,4],[7,7]],8],[[8,3],8]]
[[9,3],[[9,9],[6,[4,9]]]]
[[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]
[[[[5,2],5],[8,[3,7]]],[[5,[7,5]],[4,4]]]";

type TParsed = Vec<TParsedSub>;
type TParsedSub = usize;

pub fn day(input: String) -> (usize, usize) {
  let parsed_input = parse(&input);
  let p1 = part_1(&parsed_input);
  let p2 = part_2(&parsed_input);
  (p1, p2)
}

fn part_1(input: &TParsed) -> usize {
  0
}

fn part_2(input: &TParsed) -> usize {
  0
}

fn parse(input: &str) -> TParsed {
  input.lines().map(|s| s.parse().unwrap()).collect()
}

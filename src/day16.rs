use hex::FromHex;
use nom::bits::complete::{tag, take};
use nom::branch::alt;
use nom::multi::count;
use nom::multi::many0;
use nom::sequence::pair;
use nom::sequence::preceded;
use nom::IResult;

type THeader = (usize, usize);
#[derive(Debug)]
enum Packet {
  Lit(THeader, usize),
  Op(THeader, Vec<Packet>),
}

impl Packet {
  fn get_v_sum(&self) -> usize {
    match self {
      Packet::Lit((v, _), _) => *v,
      Packet::Op((v, _), sub) => v + sub.iter().map(|p| p.get_v_sum()).sum::<usize>(),
    }
  }
}

type TParsed = Packet;

type PType<'a> = (&'a [u8], usize);

pub fn day<'a>(input: String) -> (usize, usize) {
  let parsed_input = parse(&input);
  let p1 = part_1(&parsed_input);
  let p2 = part_2(&parsed_input);
  (p1, p2)
}

fn part_1(input: &TParsed) -> usize {
  input.get_v_sum()
}

fn part_2(input: &TParsed) -> usize {
  0
}

fn parse<'a>(input: &'a str) -> TParsed {
  match try_parse((&Vec::from_hex(input).expect("non"), 0)) {
    Ok((_, v)) => v,
    Err(e) => panic!("error parsing: {:?}", e),
  }
}

fn try_parse<'a>(i: PType<'a>) -> IResult<PType<'a>, Packet> {
  parse_packet(i)
}

fn parse_packet<'a>(i: PType<'a>) -> IResult<PType<'a>, Packet> {
  alt((parse_lit, parse_op))(i)
}

fn parse_lit<'a>(i: PType<'a>) -> IResult<PType<'a>, Packet> {
  let (i, v) = take(3usize)(i)?;
  let (i, _) = tag(4, 3usize)(i)?;

  let p_head = preceded(tag(1, 1usize), take(4usize));
  let p_tail = preceded(tag(0, 1usize), take(4usize));
  let (i, (l, r)): (_, (Vec<usize>, usize)) = pair(many0(p_head), p_tail)(i)?;

  let mut t: usize = 0;
  for n in l {
    t <<= 4;
    t |= n;
  }
  t <<= 4;
  t |= r;

  Ok((i, Packet::Lit((v, 4), t)))
}

fn parse_op<'a>(i: PType<'a>) -> IResult<PType<'a>, Packet> {
  let (i, v) = take(3usize)(i)?;
  let (i, t) = take(3usize)(i)?;
  let (i, sub_packets) = alt((parse_op_0, parse_op_1))(i)?;
  Ok((i, Packet::Op((v, t), sub_packets)))
}

fn parse_op_0<'a>(i: PType<'a>) -> IResult<PType<'a>, Vec<Packet>> {
  let (i, _) = tag(0, 1usize)(i)?;
  let (i, l): (_, usize) = take(15usize)(i)?;

  let take_bite = take(8usize);
  let (i, mut new_bytes) = count(take_bite, l / 8)(i)?;
  let rest = l % 8;

  let (i, yeet): (_, u8) = take(rest)(i)?;

  new_bytes.push(yeet << (8 - rest));

  let (_, p) = many0(parse_packet)((&new_bytes, 0)).expect("unable to parse subpackets");

  Ok((i, p))
}

fn parse_op_1<'a>(i: PType<'a>) -> IResult<PType<'a>, Vec<Packet>> {
  let (i, _) = tag(1, 1usize)(i)?;
  let (i, l): (_, usize) = take(11usize)(i)?;
  count(parse_packet, l)(i)
}

#[test]
fn yee_op_1() {
  let a = Vec::from_hex("EE00D40C823060").expect("yeet");
  let input = parse_packet((&a, 0));
  println!("Out: {:?}", input);
}

#[test]
fn yee_op_0() {
  let a = Vec::from_hex("38006F45291200").expect("yeet");
  let input = parse_packet((&a, 0));
  println!("Out: {:?}", input);
}

#[test]
fn show_parse_16() {
  let input = parse(EXAMPLE_INPUT[0].0);
  println!("{:?}", input);
}

#[test]
fn test_example_1_16() {
  for e in EXAMPLE_INPUT {
    let input = parse(e.0);
    assert_eq!(part_1(&input), e.1)
  }
}

#[test]
fn test_example_2_16() {
  for e in EXAMPLE_INPUT {
    let input = parse(e.0);
    assert_eq!(part_1(&input), e.1)
  }
}

#[cfg(test)]
const EXAMPLE_INPUT: [(&str, usize); 4] = [
  ("8A004A801A8002F478", 16),
  ("620080001611562C8802118E34", 12),
  ("C0015000016115A2E0802F182340", 23),
  ("A0016C880162017C3686B18A3D4780", 31),
];

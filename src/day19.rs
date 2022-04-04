use itertools::Itertools;
use nom::bytes::complete::tag;
use nom::bytes::complete::take_till;
use nom::character::complete::digit1;
use nom::character::complete::newline;
use nom::sequence::delimited;
use nom::sequence::pair;
use nom::{combinator::*, multi::separated_list1, IResult};
use std::collections::HashSet;
use std::hash::Hash;
use std::hash::Hasher;
use vek::mat::Mat4;
use vek::vec::Vec3;
use vek::vec::Vec4;

type TParsed = Vec<TParsedSub>;
type TParsedSub = Vec<TVec>;
type TVec = Vec4<TN>;
type TMat = Mat4<TN>;
type TN = f64;

pub fn day<'a>(input: String) -> (usize, usize) {
  let parsed_input = parse(&input);
  let p1 = part_1(&parsed_input);
  let p2 = part_2(&parsed_input);
  (p1, p2)
}

fn get_mat(i: usize) -> TMat {
  let yrot = ((i % 4) * 90) as f64;
  let zrot = (if i < 16 { i / 4 } else { 0 } * 90) as f64;
  let xrot = (if i >= 16 {
    if i < 20 {
      1
    } else {
      3
    }
  } else {
    0
  } * 90) as f64;

  TMat::rotation_x(xrot.to_radians())
    * TMat::rotation_z(zrot.to_radians())
    * TMat::rotation_y(yrot.to_radians())
}

fn get_mats() -> [TMat; 24] {
  let mut res = [TMat::zero(); 24];
  for i in 0..24 {
    res[i] = get_mat(i);
  }
  res
}

fn part_1(input: &TParsed) -> usize {
  println!("Searching for {} scanners...", res.len());

  let mut space = input[0].to_owned();
  let mats = get_mats();

  let mut find = (1..input.len()).collect::<Vec<usize>>();

  while find.len() > 0 {
    let other = &input[find[0]];
    let mut res = vec![];
    'outer: for s in &space {
      for i in 0..24 {
        let mat = mats[i];
        for anchor in other {
          let origin_o = (s - (*anchor * mat)).round();
          let overlap_beacons = other
            .iter()
            .map(|o| (*o * mat).round())
            .filter(|o| space.iter().any(|q| ((q - o) - origin_o).is_approx_zero()))
            .collect::<Vec<_>>();

          if overlap_beacons.len() >= 12 {
            res.extend(
              other
                .iter()
                .map(|o| (*o * mat).round())
                .filter(|o| !overlap_beacons.contains(o))
                .map(|o| origin_o + o),
            );
            break 'outer;
          }
        }
      }
    }
    if res.len() > 0 {
      space.append(&mut res);
      find.remove(0);
    } else {
      assert_ne!(find.len(), 1);
      let last = find[find.len() - 1];
      for i in (1..find.len()).rev() {
        find[i] = find[i - 1];
      }
      find[0] = last;
    }
  }

  space.len()
}

fn part_2(input: &TParsed) -> usize {
  0
}

fn parse<'a>(input: &'a str) -> TParsed {
  match try_parse(input) {
    Ok((_, v)) => v,
    Err(e) => panic!("error parsing: {:?}", e),
  }
}

fn try_parse<'a>(i: &'a str) -> IResult<&'a str, TParsed> {
  all_consuming(separated_list1(pair(newline, newline), parse_scanner))(i)
}

fn parse_scanner<'a>(i: &'a str) -> IResult<&'a str, TParsedSub> {
  let (i, _) = delimited(tag("--- scanner "), digit1, tag(" ---\n"))(i)?;
  separated_list1(newline, parse_vec3)(i)
}

fn parse_vec3<'a>(i: &'a str) -> IResult<&'a str, TVec> {
  map(separated_list1(tag(","), get_dig_n), |v| {
    TVec::new(v[0], v[1], v[2], 0 as TN)
  })(i)
}

fn get_dig_n<'a, A>(i: &'a str) -> IResult<&'a str, A>
where
  A: std::str::FromStr,
{
  map_res(take_till(|c| c == ',' || c == '\n'), |x: &str| x.parse())(i)
}

#[test]
fn show_parse_19() {
  let input = parse(EXAMPLE_INPUT);
  for ln in input {
    println!("{:?}", ln);
  }
}

#[test]
fn test_example_1_19() {
  let input = parse(EXAMPLE_INPUT);
  assert_eq!(part_1(&input), 79)
}

#[test]
fn test_example_2_19() {
  let input = parse(EXAMPLE_INPUT);
  assert_eq!(part_2(&input), 3993)
}

#[cfg(test)]
const EXAMPLE_INPUT: &str = "\
--- scanner 0 ---
404,-588,-901
528,-643,409
-838,591,734
390,-675,-793
-537,-823,-458
-485,-357,347
-345,-311,381
-661,-816,-575
-876,649,763
-618,-824,-621
553,345,-567
474,580,667
-447,-329,318
-584,868,-557
544,-627,-890
564,392,-477
455,729,728
-892,524,684
-689,845,-530
423,-701,434
7,-33,-71
630,319,-379
443,580,662
-789,900,-551
459,-707,401

--- scanner 1 ---
686,422,578
605,423,415
515,917,-361
-336,658,858
95,138,22
-476,619,847
-340,-569,-846
567,-361,727
-460,603,-452
669,-402,600
729,430,532
-500,-761,534
-322,571,750
-466,-666,-811
-429,-592,574
-355,545,-477
703,-491,-529
-328,-685,520
413,935,-424
-391,539,-444
586,-435,557
-364,-763,-893
807,-499,-711
755,-354,-619
553,889,-390

--- scanner 2 ---
649,640,665
682,-795,504
-784,533,-524
-644,584,-595
-588,-843,648
-30,6,44
-674,560,763
500,723,-460
609,671,-379
-555,-800,653
-675,-892,-343
697,-426,-610
578,704,681
493,664,-388
-671,-858,530
-667,343,800
571,-461,-707
-138,-166,112
-889,563,-600
646,-828,498
640,759,510
-630,509,768
-681,-892,-333
673,-379,-804
-742,-814,-386
577,-820,562

--- scanner 3 ---
-589,542,597
605,-692,669
-500,565,-823
-660,373,557
-458,-679,-417
-488,449,543
-626,468,-788
338,-750,-386
528,-832,-391
562,-778,733
-938,-730,414
543,643,-506
-524,371,-870
407,773,750
-104,29,83
378,-903,-323
-778,-728,485
426,699,580
-438,-605,-362
-469,-447,-387
509,732,623
647,635,-688
-868,-804,481
614,-800,639
595,780,-596

--- scanner 4 ---
727,592,562
-293,-554,779
441,611,-461
-714,465,-776
-743,427,-804
-660,-479,-426
832,-632,460
927,-485,-438
408,393,-506
466,436,-512
110,16,151
-258,-428,682
-393,719,612
-211,-452,876
808,-476,-593
-575,615,604
-485,667,467
-680,325,-822
-627,-443,-432
872,-547,-609
833,512,582
807,604,487
839,-516,451
891,-625,532
-652,-548,-490
30,-46,-14";
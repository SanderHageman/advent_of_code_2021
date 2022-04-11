use hashbrown::{HashMap, HashSet};
use itertools::Itertools;
use nom::bytes::complete::tag;
use nom::bytes::complete::take_till;
use nom::character::complete::digit1;
use nom::character::complete::newline;
use nom::sequence::delimited;
use nom::sequence::pair;
use nom::{combinator::*, multi::separated_list1, IResult};
use priority_queue::PriorityQueue;
use std::iter;
use vek::mat::Mat4;
use vek::vec::Vec4;

type TParsed = Vec<TParsedSub>;
type TParsedSub = Vec<TVec>;
type TVec = Vec4<TN>;
type IVec = Vec4<i64>;
type TMat = Mat4<TN>;
type TN = f64;

pub fn day<'a>(input: String) -> (usize, usize) {
  let parsed_input = parse(&input);
  let (p1, sensor_locs) = part_1(&parsed_input);
  let p2 = part_2(&sensor_locs);
  (p1, p2)
}

fn make_abst((l, r): (IVec, IVec)) -> (i64, i64) {
  fn vabs(v: IVec) -> i64 {
    v.iter().map(|x| x.abs()).sum()
  }

  fn vamax(v: IVec) -> i64 {
    v.iter().map(|x| x.abs()).max().unwrap()
  }

  let a = l - r;
  (vabs(a), vamax(a))
}

fn hash_combs(input: &TParsed) -> Vec<HashMap<(i64, i64), (IVec, IVec)>> {
  let i_sensors = input
    .iter()
    .map(|v| v.iter().map(|v| v.as_()).collect::<Vec<_>>())
    .collect::<Vec<_>>();
  let mut yee = vec![HashMap::new(); input.len()];
  for (i, sensor) in i_sensors.iter().enumerate() {
    for t in sensor.iter().cloned().tuple_combinations() {
      yee[i].insert(make_abst(t), t);
    }
  }

  yee
}

fn pp_1(input: &TParsed) -> (usize, Vec<IVec>) {
  // match 12Choose2 = 66
  let yee = hash_combs(input);
  let mut space = HashMap::<_, _>::from_iter(yee[0]);

  let mut find = PriorityQueue::<_, _>::from_iter((1..input.len()).zip(iter::repeat(u16::MAX)));

  // while let Some((input_i, old_prio)) = find.pop() {
  //   let a =

  //   'outer: for s in &space {
  //     for other in &sensors[input_i] {
  //       for test_anchor in other {
  //         let origin_o: IVec = *s - test_anchor;

  //         let beacons = other
  //           .iter()
  //           .map(|o| o + origin_o)
  //           .filter(|o| !space.contains(o));

  //         res.extend(beacons);

  //         if other.len() - res.len() >= 12 {
  //           sensor_locs.push(origin_o);
  //           break 'outer;
  //         }

  //         res.clear();
  //       }
  //     }
  //   }
  //   if res.len() > 0 {
  //     for r in res {
  //       space.insert(r);
  //     }
  //   } else {
  //     find.push(input_i, old_prio - 1);
  //   }
  // }

  (0, vec![])
}

fn part_1(input: &TParsed) -> (usize, Vec<IVec>) {
  let mut space = HashSet::with_capacity(500);
  space.extend(input[0].iter().map(|v| v.as_()));

  let mut find = PriorityQueue::<_, _>::from_iter((1..input.len()).zip(iter::repeat(u16::MAX)));

  let sensors: Vec<Vec<HashSet<IVec>>> = input
    .iter()
    .map(|s| {
      let mats = get_mats();
      (0..24)
        .map(|i| {
          s.iter()
            .map(move |o| (*o * mats[i]).round().as_())
            .collect()
        })
        .collect()
    })
    .collect();

  let mut sensor_locs: Vec<IVec> = vec![IVec::zero()];

  while let Some((input_i, old_prio)) = find.pop() {
    let mut res = HashSet::with_capacity(15);

    'outer: for s in &space {
      for other in &sensors[input_i] {
        for test_anchor in other {
          let origin_o: IVec = *s - test_anchor;

          let beacons = other
            .iter()
            .map(|o| o + origin_o)
            .filter(|o| !space.contains(o));

          res.extend(beacons);

          if other.len() - res.len() >= 12 {
            sensor_locs.push(origin_o);
            break 'outer;
          }

          res.clear();
        }
      }
    }
    if res.len() > 0 {
      for r in res {
        space.insert(r);
      }
    } else {
      find.push(input_i, old_prio - 1);
    }
  }

  (space.len(), sensor_locs)
}

fn part_2(input: &Vec<IVec>) -> usize {
  input
    .iter()
    .tuple_combinations()
    .map(|(l, r)| (l - r).iter().map(|v| v.abs() as usize).sum())
    .max()
    .unwrap()
}

fn get_mats() -> [TMat; 24] {
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

  let mut res = [TMat::zero(); 24];
  for i in 0..24 {
    res[i] = get_mat(i);
  }
  res
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
  assert_eq!(pp_1(&input).0, 79)
}

#[test]
fn test_example_2_19() {
  let input = parse(EXAMPLE_INPUT);
  assert_eq!(part_2(&part_1(&input).1), 3621)
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

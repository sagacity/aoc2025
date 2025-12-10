use std::collections::{BTreeMap, BTreeSet};

fn f(input: &str, num: usize) -> usize {
    let boxes = input.lines().map(|line| {
        let b = line.split(',').map(|v| v.parse::<usize>().unwrap()).collect::<Vec<usize>>();
        (b[0], b[1], b[2])
    }).collect::<Vec<_>>();

    fn calc_dist(a: (usize, usize, usize), b: (usize, usize, usize)) -> usize {
        (
            (a.0 as i64 - b.0 as i64) * (a.0 as i64 - b.0 as i64) +
            (a.1 as i64 - b.1 as i64) * (a.1 as i64 - b.1 as i64) +
            (a.2 as i64 - b.2 as i64) * (a.2 as i64 - b.2 as i64)
        ) as usize
    }

    let mut distances = BTreeMap::new();
    for (x, y, z) in &boxes {
        for (ox, oy, oz) in &boxes {
            let dist = calc_dist((*x, *y, *z), (*ox, *oy, *oz));
            if x != ox || y != oy || z != oz {
                distances.insert(dist, (*x, *y, *z, *ox, *oy, *oz));
            }
        }
    }

    let distances = distances.iter().take(num).collect::<Vec<_>>();
    let mut result: Vec<BTreeSet<(usize, usize, usize)>> = vec![];
    for b in &boxes {
        let mut set = BTreeSet::<(usize, usize, usize)>::new();
        set.insert(*b);
        result.push(set);
    }

    for (_, d) in distances {
        let (x, y, z, ox, oy, oz) = *d;
        //println!("adding: {d:?}");
        //println!("{result:?}");

        let a = result.iter().position(|circuit| circuit.contains(&(x, y, z))).unwrap();
        let b = result.iter().position(|circuit| circuit.contains(&(ox, oy, oz))).unwrap();
        let b_data = result[b].clone();
        result[a].extend(b_data);
        if a != b {
            result.remove(b);
        }
    }

    result.sort_by(|a, b| b.len().cmp(&a.len()));
    /*for b in &result {
        println!("{}", b.len());
    }
    println!("{result:?}");*/

    result[0..3].iter().map(|b| b.len()).product()
}

fn f2(input: &str) -> usize {
    let boxes = input.lines().map(|line| {
        let b = line.split(',').map(|v| v.parse::<usize>().unwrap()).collect::<Vec<usize>>();
        (b[0], b[1], b[2])
    }).collect::<Vec<_>>();

    fn calc_dist(a: (usize, usize, usize), b: (usize, usize, usize)) -> usize {
        (
            (a.0 as i64 - b.0 as i64) * (a.0 as i64 - b.0 as i64) +
                (a.1 as i64 - b.1 as i64) * (a.1 as i64 - b.1 as i64) +
                (a.2 as i64 - b.2 as i64) * (a.2 as i64 - b.2 as i64)
        ) as usize
    }

    let mut distances = BTreeMap::new();
    for (x, y, z) in &boxes {
        for (ox, oy, oz) in &boxes {
            let dist = calc_dist((*x, *y, *z), (*ox, *oy, *oz));
            if x != ox || y != oy || z != oz {
                distances.insert(dist, (*x, *y, *z, *ox, *oy, *oz));
            }
        }
    }

    let mut result: Vec<BTreeSet<(usize, usize, usize)>> = vec![];
    for b in &boxes {
        let mut set = BTreeSet::<(usize, usize, usize)>::new();
        set.insert(*b);
        result.push(set);
    }

    for (_, d) in &distances {
        let (x, y, z, ox, oy, oz) = *d;
        //println!("adding: {d:?}");
        //println!("{result:?}");

        let a = result.iter().position(|circuit| circuit.contains(&(x, y, z))).unwrap();
        let b = result.iter().position(|circuit| circuit.contains(&(ox, oy, oz))).unwrap();
        let b_data = result[b].clone();
        result[a].extend(b_data);
        if a != b {
            result.remove(b);
        }

        if result.len() == 1 {
            return x * ox;
        }
    }

    panic!()
}

fn main() {
    println!("{}", f(include_str!("day8.txt"), 1000));
    println!("{}", f2(include_str!("day8.txt")));
}

#[test]
fn example() {
    let input = r#"162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689"#;
    assert_eq!(f(input, 10), 40);
    assert_eq!(f2(input), 25272);
}
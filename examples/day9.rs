use geo::{Coord, Covers, LineString, Polygon, Rect};

fn f(input: &str) -> usize {
    let coords = input.lines().map(|line| {
        let (l, r) = line.split_at(line.find(',').unwrap());
        let (_, r) = r.split_at(1);
        (l.parse::<isize>().unwrap(), r.parse::<isize>().unwrap())
    }).collect::<Vec<_>>();

    let mut max = 0;
    for a in 0..coords.len() {
        for b in 0..coords.len() {
            if b == a {
                continue;
            }

            let area = ((coords[b].1 - coords[a].1).abs() + 1) * ((coords[b].0 - coords[a].0).abs() + 1);
            if area > max {
                max = area;
            }
        }
    }

    max as usize
}

fn f2(input: &str) -> usize {
    let coords = input.lines().map(|line| {
        let (l, r) = line.split_at(line.find(',').unwrap());
        let (_, r) = r.split_at(1);
        Coord { x: l.parse::<f64>().unwrap(), y: r.parse::<f64>().unwrap() }
    }).collect::<Vec<_>>();

    let polygon = Polygon::new(LineString::new(coords.clone()), vec![]);

    let mut max = 0.0;
    for a in 0..coords.len() {
        for b in 0..coords.len() {
            if b == a {
                continue;
            }

            let area = ((coords[b].y - coords[a].y).abs() + 1.0) * ((coords[b].x - coords[a].x).abs() + 1.0);
            if area > max {
                let rect = Rect::new(coords[a], coords[b]);
                if !polygon.covers(&rect) {
                    continue;
                }

                max = area;
            }
        }
    }

    max as usize
}

fn main() {
    println!("{}", f(include_str!("day9.txt")));
    println!("{}", f2(include_str!("day9.txt")));
}

#[test]
fn example() {
    let input = r#"7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3"#;
    assert_eq!(f(input), 50);
    assert_eq!(f2(input), 24);
}
fn f(input: &str) -> usize {
    let mut cnt = 0;

    let lines = input.lines().collect::<Vec<&str>>();
    let mut iter = lines.iter();

    let mut ranges = vec![];

    loop {
        let line = iter.next().unwrap();
        if line.is_empty() {
            break;
        }

        let (l, r) = line.split_at(line.find('-').unwrap());
        let (_, r) = r.split_at(1);

        let lv = l.parse::<u64>().unwrap();
        let rv = r.parse::<u64>().unwrap();
        ranges.push((lv, rv));
    }

    loop {
        let line = iter.next();
        match line {
            Some(line) => {
                let line = line.parse::<u64>().unwrap();

                for (lr, rr) in &ranges {
                    if line >= *lr && line <= *rr {
                        //println!("{line} is fresh");
                        cnt += 1;
                        break;
                    }
                }
            },
            None => break
        }
    }

    cnt
}

fn f2(input: &str) -> usize {
    let lines = input.lines().collect::<Vec<&str>>();
    let mut iter = lines.iter();

    let mut ranges = vec![];

    loop {
        let line = iter.next().unwrap();
        if line.is_empty() {
            break;
        }

        let (l, r) = line.split_at(line.find('-').unwrap());
        let (_, r) = r.split_at(1);

        let lv = l.parse::<u64>().unwrap();
        let rv = r.parse::<u64>().unwrap();
        ranges.push((lv, rv));
    }

    ranges.sort_by(|a, b| a.0.cmp(&b.0));
    //println!("{ranges:?}");

    // combine ranges and remove ranges that are entirely overlapped
    let mut x = 0;
    while x < ranges.len() - 1 {
        let a = ranges[x];
        let b = ranges[x + 1];
        if a.1 >= b.0 {
            // range is entirely overlapped
            if a.1 >= b.1 {
                ranges.remove(x + 1);
            } else {
                // combine ranges
                ranges[x] = (a.0, b.1);
            }
        } else {
            x += 1;
        }
    }

    ranges.iter()
        .map(|(x, y)| (y - x + 1) as usize)
        .sum()
}

fn main() {
    println!("{}", f(include_str!("day5.txt")));
    println!("{}", f2(include_str!("day5.txt")));
}

#[test]
fn example() {
    let input = r#"3-5
10-14
16-20
12-18

1
5
8
11
17
32"#;
    assert_eq!(f(input), 3);
    assert_eq!(f2(input), 14);
}
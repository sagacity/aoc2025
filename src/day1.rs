fn f(input: &str) -> usize {
    let mut dial = 50i32;
    let mut cnt = 0;
    for line in input.lines() {
        let (dir, val) = line.split_at(1);
        let val = val.parse::<i32>().unwrap();
        match dir {
            "L" => dial = dial - val,
            "R" => dial = dial + val,
            _ => unimplemented!()
        }
        dial = dial.rem_euclid(100);
        //println!("{dial}");

        if dial == 0 {
            cnt += 1;
        }
    }

    cnt
}

fn f2(input: &str) -> usize {
    let mut dial = 50i32;
    let mut cnt = 0i32;
    for line in input.lines() {
        let (dir, val) = line.split_at(1);
        let val = val.parse::<i32>().unwrap();
        match dir {
            "L" => {
                if dial == 0 {
                    dial = 100;
                }
                dial = dial - val;
                cnt += -(dial - 100) / 100;
            },
            "R" => {
                dial = dial + val;
                cnt += dial / 100;
            },
            _ => unimplemented!()
        }

        dial = dial.rem_euclid(100);
    }

    cnt as usize
}

#[test]
fn day1() {
    println!("{}", f(include_str!("day1.txt")));
    println!("{}", f2(include_str!("day1.txt")));
}

#[test]
fn example() {
    let input = r#"L68
L30
R48
L5
R60
L55
L1
L99
R14
L82"#;
    assert_eq!(f(input), 3);
    assert_eq!(f2(input), 6);

    assert_eq!(f2("R1000"), 10);
}
use std::collections::VecDeque;

fn f(input: &str) -> usize {
    let mut total_moves = 0;
    for line in input.lines() {
        let mut line = line.split_whitespace();
        let expected = line.next().unwrap();
        let expected = &expected[1..expected.len() - 1];
        let expected: u16 = expected.chars().enumerate().map(|(idx, c)| {
            if c == '#' { 1 << idx } else { 0 }
        }).sum();
        //println!("{expected}");
        let mut buttons = vec![];
        while let Some(value) = line.next() {
            if value.starts_with('(') {
                let mut val = 0u16;
                for bval in value[1..value.len() - 1].split(',') {
                    val |= 1 << bval.parse::<u16>().unwrap();
                }
                buttons.push(val);
                //println!("{val}");
            }
        }
        let mut depth = 0;

        let mut to_check: VecDeque<(u16, u16, u16)> = VecDeque::new();
        for b in &buttons {
            to_check.push_back((0, *b, 1));
        }

        while !to_check.is_empty() {
            let (mut state, button, actual_depth) = to_check.pop_front().unwrap();
            state ^= button;
            if state == expected {
                depth = actual_depth;
                break;
            } else {
                for b in &buttons {
                    to_check.push_back((state, *b, actual_depth + 1));
                }
            }
        }

        total_moves += depth as usize;
    }

    total_moves
}

fn f2(input: &str) -> usize {
    let mut total_moves = 0;
    for line in input.lines() {
        let mut line = line.split_whitespace();
        let expected = line.next().unwrap();
        let mut buttons = vec![];
        let mut joltage = vec![];
        while let Some(value) = line.next() {
            if value.starts_with('(') {
                let mut val = 0u16;
                for bval in value[1..value.len() - 1].split(',') {
                    val |= 1 << bval.parse::<u16>().unwrap();
                }
                buttons.push(val);
                //println!("{val}");
            }
            if value.starts_with('{') {
                for j in value[1..value.len() - 1].split(',') {
                    joltage.push(j.parse::<usize>().unwrap());
                }
            }
        }
        let mut depth = 0;

        let mut to_check: VecDeque<(Vec<usize>, u16, u16)> = VecDeque::new();
        for b in &buttons {
            to_check.push_back((vec![0; joltage.len()], *b, 1));
        }

        while !to_check.is_empty() {
            let (mut state, mut button, actual_depth) = to_check.pop_front().unwrap();
            //println!("state before: {state:?} // button: {button}");
            let mut cnt = 0;
            while button > 0 {
                //println!("b: {button}");
                if (button & 1) == 1 {
                    //println!("flipping {cnt}");
                    state[cnt] += 1;
                }
                button >>= 1;
                cnt += 1;
            }

            if state.iter().zip(joltage.iter()).any(|(a, b)| a > b) {
                continue;
            }

            //println!("state after: {state:?}");
            //println!("{state:?} vs {joltage:?}: {}", state == joltage);
            if state == joltage {
                //println!("{state:?}, {actual_depth}");
                depth = actual_depth;
                break;
            } else {
                for b in &buttons {
                    to_check.push_back((state.clone(), *b, actual_depth + 1));
                }
            }
        }
        println!("{depth}");

        total_moves += depth as usize;
    }

    total_moves
}

fn main() {
    println!("{}", f(include_str!("day10.txt")));
    println!("{}", f2(include_str!("day10.txt")));
}

#[test]
fn example() {
    let input = r#"[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}"#;
    assert_eq!(f(input), 7);
    assert_eq!(f2(input), 33);
}
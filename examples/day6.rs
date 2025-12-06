fn f(input: &str) -> usize {
    let mut cnt = 0;

    let lines = input.lines().collect::<Vec<_>>();
    let (ops, lines) = lines.split_last().unwrap();
    let mut lines = lines.into_iter().map(|line| line.split_ascii_whitespace()).collect::<Vec<_>>();

    let ops = ops.split_ascii_whitespace();
    for op in ops {
        let data = lines.iter_mut().map(|l| l.next().unwrap().parse::<usize>().unwrap()).collect::<Vec<_>>();
        let result: usize = match op {
            "+" => data.into_iter().sum(),
            "*" => data.into_iter().product(),
            _ => unimplemented!()
        };
        cnt += result;
    }

    cnt
}

fn f2(input: &str) -> usize {
    let mut cnt = 0;

    let lines = input.lines().collect::<Vec<_>>();
    let (ops, lines) = lines.split_last().unwrap();

    let mut x = 0;
    let ops = ops.split_ascii_whitespace();
    for op in ops {
        let mut columns = vec![];

        loop {
            let chars = lines.iter().map(|l| l.chars().nth(x).unwrap_or_else(|| ' ')).collect::<Vec<_>>();
            x += 1;

            if chars.iter().all(|c| c.is_whitespace()) || lines.iter().all(|l| x == l.len()) {
                break;
            } else {
                columns.push(chars.clone());
            }
        }
        let data = columns.into_iter().map(|c| c.iter().collect::<String>().trim().parse::<usize>().unwrap()).collect::<Vec<_>>();

        let result: usize = match op {
            "+" => data.into_iter().sum(),
            "*" => data.into_iter().product(),
            _ => unimplemented!()
        };
        cnt += result;
    }

    cnt
}

fn main() {
    println!("{}", f(include_str!("day6.txt")));
    println!("{}", f2(include_str!("day6.txt")));
}

#[test]
fn example() {
    let input = r#"123 328  51 64
 45 64  387 23
  6 98  215 314
*   +   *   +"#;
    assert_eq!(f(input), 4277556);
    assert_eq!(f2(input), 3263827);
}
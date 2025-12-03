fn f(input: &str, num: usize) -> usize {
    input.lines()
        .map(|line| {
            let mut jolt = String::new();
            let mut pos = 0;
            let mut max_pos = 0;
            let mut left = num;

            while jolt.len() < num {
                let mut max = 0;
                //println!("pos: {pos}, len: {}, left: {left}", line.len());
                while pos <= line.len() - left {
                    //println!("CHECKING pos {pos}");
                    let val = line.chars().nth(pos).unwrap().to_digit(10).unwrap();
                    if val > max {
                        max = val;
                        max_pos = pos;
                    }
                    pos += 1;
                }
                //println!("max: {max}");
                jolt += &max.to_string();
                pos = max_pos + 1;
                left -= 1;
            }

            //jolt += &line[pos..].to_string();

            jolt.parse::<usize>().unwrap()
        })
        .sum()
}

fn main() {
    println!("{}", f(include_str!("day3.txt"), 2));
    println!("{}", f(include_str!("day3.txt"), 12));
}

#[test]
fn example() {
    assert_eq!(f("9876", 2), 98);
    assert_eq!(f("7698", 2), 98);

    let input = r#"987654321111111
811111111111119
234234234234278
818181911112111"#;
    assert_eq!(f(input, 2), 357);

    assert_eq!(f("818181911112111", 12), 888911112111);

    let input = r#"987654321111111
811111111111119
234234234234278
818181911112111"#;
    assert_eq!(f(input, 12), 3121910778619);
}
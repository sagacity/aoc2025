fn f(input: &str) -> u64 {
    let mut result = 0;

    for range in input.split(',') {
        let range = range.trim();
        if range.is_empty() {
            continue;
        }

        let (l, r) = range.split_at(range.find('-').unwrap());
        let (_, r) = r.split_at(1);

        let lv = l.parse::<u64>().unwrap();
        let rv = r.parse::<u64>().unwrap();

        for pv in lv..=rv {
            let p = pv.to_string();
            for subpos in 1..=p.len() {
                let substr = p[0..subpos].to_string();
                let substr = substr.clone() + substr.as_str();
                if p == substr {
                    result += pv;
                    //println!("INVALID: {p}");
                    break;
                }
            }
        }
    }

    result
}

fn f2(input: &str) -> u64 {
    let mut result = 0;

    for range in input.split(',') {
        let range = range.trim();
        if range.is_empty() {
            continue;
        }

        let (l, r) = range.split_at(range.find('-').unwrap());
        let (_, r) = r.split_at(1);

        let lv = l.parse::<u64>().unwrap();
        let rv = r.parse::<u64>().unwrap();

        for pv in lv..=rv {
            let p = pv.to_string();
            // only check half the string because a valid sequence needs to contain twice this substr
            for subpos in 1..=p.len() / 2 {
                let substr = p[0..subpos].to_string();
                let match_count = p.match_indices(&substr).count();
                if p.len() == match_count * substr.len() {
                    result += pv;
                    //println!("INVALID: {p} // {} {} {} {}", substr, p.len(), match_count, substr.len());
                    break;
                }
            }
        }
    }

    result
}

#[test]
fn day2() {
    println!("{}", f(include_str!("day2.txt")));
    println!("{}", f2(include_str!("day2.txt")));
}

#[test]
fn example() {
    let input = r#"11-22,95-115,998-1012,1188511880-1188511890,222220-222224,
1698522-1698528,446443-446449,38593856-38593862,565653-565659,
824824821-824824827,2121212118-2121212124"#;
    assert_eq!(f(input), 1227775554);
    assert_eq!(f2(input), 4174379265);
}
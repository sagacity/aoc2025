use std::collections::{BTreeMap, VecDeque};

fn f(input: &str) -> usize {
    let mut dev_names = BTreeMap::new();
    let mut devs = BTreeMap::new();
    for line in input.lines() {
        let (name, rest) = line.split_once(":").unwrap();
        let outputs = rest[1..].split_whitespace().collect::<Vec<_>>();
        let len = dev_names.len() as u16;
        dev_names.entry(name).or_insert(len);
        for o in &outputs {
            let len = dev_names.len() as u16;
            dev_names.entry(o).or_insert(len);
        }

        devs.insert(dev_names[name], outputs.iter().map(|o| dev_names[o]).collect::<Vec<_>>());
    }

    let mut paths = 0;
    let mut to_check = VecDeque::new();
    to_check.push_back(dev_names["you"]);
    while !to_check.is_empty() {
        let check = to_check.pop_front().unwrap();
        //println!("at: {check}");
        let outputs = devs.get(&check).unwrap();
        if outputs.contains(&dev_names["out"]) {
            assert_eq!(outputs.len(), 1);
            paths += 1;
        } else {
            //println!("`--> going to: {outputs:?}");
            to_check.extend(outputs);
        }
    }

    paths
}

fn visit(devs: &BTreeMap<&str, Vec<&str>>, dev: &str, mut should_visit: Vec<&str>, visited: &mut BTreeMap<Vec<String>, usize>) -> usize {
    if dev == "out" {
        if should_visit.is_empty() {
            1
        } else {
            0
        }
    } else {
        if dev == "dac" || dev == "fft" {
            should_visit.retain(|d| *d != dev);
        }

        let mut to_visit = vec![dev];
        to_visit.extend(should_visit.iter().cloned());
        let to_visit = to_visit.into_iter().map(|t| t.to_string()).collect::<Vec<_>>();
        match visited.get(&to_visit) {
            Some(paths) => *paths,
            None => {
                let paths = devs[dev].iter().map(|o| visit(devs, o, should_visit.clone(), visited)).sum();
                visited.insert(to_visit.clone(), paths);
                paths
            }
        }
    }
}

fn f2(input: &str) -> usize {
    let mut devs = BTreeMap::new();
    for line in input.lines() {
        let (name, rest) = line.split_once(":").unwrap();
        let outputs = rest[1..].split_whitespace().collect::<Vec<_>>();
        devs.insert(name, outputs);
    }

    let should_visit = vec!["dac", "fft"];
    let mut visited = BTreeMap::new();
    visit(&devs, "svr", should_visit, &mut visited)
}

fn main() {
    println!("{}", f(include_str!("day11.txt")));
    println!("{}", f2(include_str!("day11.txt")));
}

#[test]
fn example() {
    let input = r#"aaa: you hhh
you: bbb ccc
bbb: ddd eee
ccc: ddd eee fff
ddd: ggg
eee: out
fff: out
ggg: out
hhh: ccc fff iii
iii: out"#;
    assert_eq!(f(input), 5);
}

#[test]
fn example2() {
    let input = r#"svr: aaa bbb
aaa: fft
fft: ccc
bbb: tty
tty: ccc
ccc: ddd eee
ddd: hub
hub: fff
eee: dac
dac: fff
fff: ggg hhh
ggg: out
hhh: out"#;
    assert_eq!(f2(input), 2);
}
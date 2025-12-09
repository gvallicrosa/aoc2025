use std::collections::{HashMap, HashSet};

enum Cell {
    Start,
    Empty,
    Splitter,
}

fn main() -> Result<(), std::io::Error> {
    let mut data = Vec::new();

    // for line in include_str!("../input_example.txt")
    for line in include_str!("../input.txt")
        .replace("\r\n", "\n")
        .split("\n")
    {
        let row: Vec<_> = line
            .chars()
            .map(|c| match c {
                'S' => Cell::Start,
                '.' => Cell::Empty,
                '^' => Cell::Splitter,
                _ => panic!("invalid cell"),
            })
            .collect();
        data.push(row);
    }

    // part 1
    {
        // init
        let mut beams = HashSet::new();
        for (c, v) in data[0].iter().enumerate() {
            if let Cell::Start = v {
                beams.insert(c);
            }
        }
        // simulate
        let mut splits = 0;
        for row in data.iter().skip(1) {
            let mut next = HashSet::new();
            for c in beams {
                if let Cell::Splitter = row[c] {
                    splits += 1;
                    next.insert(c - 1);
                    next.insert(c + 1);
                } else {
                    next.insert(c);
                }
            }
            beams = next;
        }
        dbg!(splits);
    }

    // part 2
    {
        // init
        let mut beams: HashMap<usize, usize> = HashMap::new();
        for (c, v) in data[0].iter().enumerate() {
            if let Cell::Start = v {
                beams.insert(c, 1); // one beam at column c
            }
        }
        // simulate
        // dbg!(&beams);
        for row in data.iter().skip(1) {
            let mut next = HashMap::new();
            for (c, b) in beams {
                if let Cell::Splitter = row[c] {
                    *next.entry(c - 1).or_insert(0) += b;
                    *next.entry(c + 1).or_insert(0) += b;
                } else {
                    *next.entry(c).or_insert(0) += b;
                }
            }
            // dbg!(&next);
            beams = next;
        }
        dbg!(beams.iter().fold(0, |acc, (_, v)| acc + v));
    }

    Ok(())
}

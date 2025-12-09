use std::collections::VecDeque;

#[derive(PartialEq, Eq)]
enum State {
    Ranges,
    Ingredients,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Range {
    start: u64,
    stop: u64,
}

impl Range {
    fn contains(&self, value: u64) -> bool {
        if value >= self.start && value <= self.stop {
            return true;
        }
        false
    }

    fn count(&self) -> u64 {
        self.stop - self.start + 1 // inclusive
    }

    fn intersect(&self, other: &Self) -> Self {
        Range {
            start: std::cmp::max(self.start, other.start),
            stop: std::cmp::min(self.stop, other.stop),
        }
    }

    fn union(&self, other: &Self) -> Self {
        Range {
            start: std::cmp::min(self.start, other.start),
            stop: std::cmp::max(self.stop, other.stop),
        }
    }
}

fn main() -> Result<(), std::io::Error> {
    let mut ranges = Vec::new();
    let mut ingredients = Vec::new();
    let mut state = State::Ranges;

    // for line in include_str!("../input_example.txt")
    for line in include_str!("../input.txt")
        .replace("\r\n", "\n")
        .split("\n")
    {
        if line.is_empty() {
            state = State::Ingredients;
            continue;
        }

        match state {
            State::Ranges => {
                let mut iter = line.split("-");
                let start = iter.next().unwrap().parse::<u64>().unwrap();
                let stop = iter.next().unwrap().parse::<u64>().unwrap();
                ranges.push(Range { start, stop });
            }
            State::Ingredients => {
                ingredients.push(line.parse::<u64>().unwrap());
            }
        }
    }

    // part 1
    {
        let mut sum = 0;
        for ingredient in ingredients {
            println!("{}", ingredient);
            for range in ranges.iter() {
                if range.contains(ingredient) {
                    sum += 1;
                    println!("  => fresh");
                    break;
                }
            }
        }
        dbg!(sum);
    }

    // part 2
    {
        ranges.sort();
        let mut vd = VecDeque::with_capacity(ranges.len());
        for r in ranges {
            println!("{:?}", &r);
            vd.push_back(r);
        }
        let mut current = vd.pop_front().unwrap();
        let mut sum = current.count();
        while !vd.is_empty() {
            let next = vd.pop_front().unwrap();
            if next.start > current.stop {
                // start not included
                sum += next.count();
                current = next;
            } else {
                // included
                sum += next.count() - current.intersect(&next).count();
                current = current.union(&next);
            }
        }
        dbg!(sum);
    }

    Ok(())
}

struct Range {
    start: u64,
    stop: u64,
}

impl Range {
    fn invalid_ids_sum(&self) -> u64 {
        let mut sum = 0;
        for v in self.start..=self.stop {
            let str = format!("{}", v);
            if str.len() % 2 == 0 {
                let half = str.len() / 2;
                if str[..half] == str[half..] {
                    sum += v;
                    dbg!(v);
                }
            }
        }
        sum
    }

    fn invalid_ids_sum_part2(&self) -> u64 {
        let mut sum = 0;
        for v in self.start..=self.stop {
            let str = format!("{}", v);
            let half = str.len() / 2;
            let groups = (1..=half).filter(|a| str.len() % a == 0);
            for group_size in groups {
                let num = str.len() / group_size;
                let mut invalid = true;
                for n in 0..num {
                    if str[0..group_size] != str[n * group_size..(group_size + n * group_size)] {
                        invalid = false;
                    }
                }
                if invalid {
                    sum += v;
                    dbg!(v);
                    break; // next value
                }
            }
        }
        sum
    }
}

fn main() -> Result<(), std::io::Error> {
    let mut ranges = Vec::new();

    // for line in include_str!("../input_example.txt")
    for line in include_str!("../input.txt")
        .replace("\r\n", "\n")
        .split("\n")
    {
        for r in line.split(",") {
            let mut iter = r.split("-");
            let start = iter.next().unwrap().parse::<u64>().unwrap();
            let stop = iter.next().unwrap().parse::<u64>().unwrap();
            ranges.push(Range { start, stop });
        }
    }

    // part 1
    {
        let mut invalid_sum = 0;
        for r in ranges.iter() {
            invalid_sum += r.invalid_ids_sum();
        }
        dbg!(invalid_sum);
    }

    // part 2
    {
        let mut invalid_sum = 0;
        for r in ranges.iter() {
            invalid_sum += r.invalid_ids_sum_part2();
        }
        dbg!(invalid_sum);
    }

    Ok(())
}

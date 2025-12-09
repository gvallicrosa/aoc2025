struct Bank {
    batteries: Vec<usize>,
}

impl Bank {
    fn biggest_2(&self) -> usize {
        let (index, val) = self
            .batteries
            .iter()
            .enumerate()
            .rev() // to get 1st max
            .max_by(|(_, a), (_, b)| a.cmp(b))
            .unwrap();
        // println!("{:?} ({})", self.batteries, self.batteries.len());
        // println!("  {} val {}", index, val);
        if index + 1 != self.batteries.len() {
            let val2 = self.batteries.iter().skip(index + 1).max().unwrap();
            // println!("  val2 {}", &val2);
            return val * 10 + val2;
        }

        let not_last = &self.batteries[..self.batteries.len() - 1];
        // println!("  {:?} ({})", not_last, not_last.len());
        let val = not_last.iter().max().unwrap();
        val * 10 + self.batteries.last().unwrap()
    }
    fn biggest_12(&self) -> usize {
        // println!(">>>> {:?} ({})", self.batteries, self.batteries.len());
        let mut index = 0;
        let mut sum = 0;
        for i in 0..12 {
            // dbg!(index);
            let remaining = 12 - i - 1; // at least this remaining numbers
            let end = self.batteries.len() - remaining;
            let values = &self.batteries[index..end];
            // println!("{:?}", values);

            let (idx, v) = values
                .iter()
                .enumerate()
                .rev() // to get 1st max
                .max_by(|(_, a), (_, b)| a.cmp(b))
                .unwrap();
            // next
            index += idx + 1;
            sum = sum * 10 + v;
        }
        sum
    }
}

fn main() -> Result<(), std::io::Error> {
    let mut banks = Vec::new();
    // for line in include_str!("../input_example.txt")
    for line in include_str!("../input.txt")
        .replace("\r\n", "\n")
        .split("\n")
    {
        banks.push(Bank {
            batteries: line
                .chars()
                .map(|v| v.to_digit(10).unwrap() as usize)
                .collect(),
        });
    }

    // part 1
    {
        let mut sum = 0;
        for bank in banks.iter() {
            let biggest = bank.biggest_2();
            println!("{}", biggest);
            sum += biggest;
        }
        dbg!(sum);
    }
    // part 2
    {
        let mut sum = 0;
        for bank in banks.iter() {
            let biggest = bank.biggest_12();
            println!("{}", biggest);
            sum += biggest;
        }
        dbg!(sum);
    }
    // only exactly correct "mul(123,456)"
    // dbg!(sum);
    // {
    //     let mut sum = 0;
    //     for v in operations.iter() {
    //         if let Operation::Mul { index: _, value } = v {
    //             sum += value;
    //         }
    //     }
    //     dbg!(sum);
    // }

    // part 2
    // do() and don't() enable and disable mul()
    // dbg!(sum_part2);
    // {
    //     let mut enabled = true;
    //     let mut sum = 0;
    //     operations.sort_by_key(|v| v.index());
    //     // dbg!(operations);
    //     for v in operations.iter() {
    //         match v {
    //             Operation::Do { index: _ } => enabled = true,
    //             Operation::Dont { index: _ } => enabled = false,
    //             Operation::Mul { index: _, value } => {
    //                 if enabled {
    //                     sum += value;
    //                 }
    //             }
    //         }
    //     }
    //     dbg!(sum);
    // }
    Ok(())
}

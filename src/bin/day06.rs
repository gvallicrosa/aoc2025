#[derive(Debug, PartialEq)]
enum Operation {
    Add,
    Multiply,
}

#[derive(Debug)]
enum Cell {
    Empty,
    Value(u64),
    Op(Operation),
}

fn main() -> Result<(), std::io::Error> {
    let mut data = Vec::new();
    let mut operations = Vec::new();
    let mut data_rtl = Vec::new();

    // for line in include_str!("../input_example.txt")
    for line in include_str!("../input.txt")
        .replace("\r\n", "\n")
        .split("\n")
    {
        if line.starts_with('+') || line.starts_with('*') {
            operations = line
                .split_ascii_whitespace()
                .map(|v| match v {
                    "+" => Operation::Add,
                    "*" => Operation::Multiply,
                    _ => panic!("unknown operation"),
                })
                .collect();
        } else {
            // part 1
            let row: Vec<_> = line
                .split_ascii_whitespace()
                .map(|v| v.parse::<u64>().unwrap())
                .collect();
            data.push(row);
        }

        // part 2
        let row: Vec<_> = line
            .chars()
            .map(|v| {
                if v.is_ascii_digit() {
                    Cell::Value(v.to_digit(10).unwrap() as u64)
                } else if v == ' ' {
                    Cell::Empty
                } else if v == '+' {
                    Cell::Op(Operation::Add)
                } else {
                    Cell::Op(Operation::Multiply)
                }
            })
            .collect();
        data_rtl.push(row);
    }

    // part 1
    {
        let mut sum = 0;
        for c in 0..data[0].len() {
            let op = &operations[c];
            let mut values = Vec::with_capacity(data.len());
            for row in data.iter() {
                values.push(row[c]);
            }
            match op {
                Operation::Add => sum += values.iter().sum::<u64>(),
                Operation::Multiply => sum += values.iter().product::<u64>(),
            }
        }
        dbg!(sum);
    }

    // part 2
    {
        let mut sum = 0;
        let mut partial = Vec::new();
        let mut op = &Operation::Add;
        for c in 0..data_rtl[0].len() {
            println!("column {}", c);
            // new operation
            if let Cell::Op(o) = &data_rtl.last().unwrap()[c] {
                op = o;
            }
            // col by col
            let mut all_empty = true;
            let mut value = 0;
            for row in data_rtl.iter() {
                print!("{:?} ", &row[c]);
                if let Cell::Value(v) = row[c] {
                    value = value * 10 + v;
                    all_empty = false;
                }
            }
            println!();
            if all_empty {
                dbg!(&partial);
                sum += match op {
                    Operation::Add => partial.iter().sum::<u64>(),
                    Operation::Multiply => partial.iter().product::<u64>(),
                };
                partial.clear();
            } else {
                partial.push(value);
            }
        }
        // last
        dbg!(&partial);
        sum += match op {
            Operation::Add => partial.iter().sum::<u64>(),
            Operation::Multiply => partial.iter().product::<u64>(),
        };
        dbg!(sum);
    }

    Ok(())
}

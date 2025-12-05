#[derive(Debug)]
enum Direction {
    Left(u16),
    Right(u16),
}

fn main() -> Result<(), std::io::Error> {
    let mut rotations = Vec::new();

    // for line in include_str!("../input_example.txt")
    for line in include_str!("../input.txt")
        .replace("\r\n", "\n")
        .split("\n")
    {
        println!("{}", line);
        let value = line[1..].parse::<u16>().unwrap();
        if line.chars().next().unwrap() == 'R' {
            rotations.push(Direction::Right(value));
        } else {
            rotations.push(Direction::Left(value));
        }
    }

    // solve
    let mut position = 50;
    let mut at_zero = 0;
    let mut passing_zero = 0;
    for rot in rotations {
        println!("position {:2} + {:?}", position, &rot);
        match rot {
            Direction::Right(v) => {
                passing_zero += v / 100;
                let rem = v % 100;
                // part 2
                if (position != 0) && (position + rem) >= 100 {
                    passing_zero += 1;
                }
                // position
                position = (position + v) % 100;
            }
            Direction::Left(v) => {
                passing_zero += v / 100;
                let rem = v % 100;
                // part 2
                if (position != 0) && (position <= rem) {
                    passing_zero += 1;
                }
                // position
                if rem > position {
                    position = 100 - ((v % 100) - position);
                } else {
                    position -= rem;
                }
            }
        }
        // part 1
        if position == 0 {
            at_zero += 1;
        }
    }
    println!("position {}", position);
    dbg!(at_zero);
    dbg!(passing_zero);

    Ok(())
}

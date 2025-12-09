#[derive(PartialEq, Debug)]
struct Point {
    x: i64,
    y: i64,
    z: i64,
}

impl Point {
    fn distance(&self, other: &Point) -> i64 {
        (self.x - other.x) * (self.x - other.x)
            + (self.y - other.y) * (self.y - other.y)
            + (self.z - other.z) * (self.z - other.z)
    }
}

fn main() -> Result<(), std::io::Error> {
    let mut boxes = Vec::new();

    // let iterations = 10;
    // for line in include_str!("../input_example.txt")
    let iterations = 1000;
    for line in include_str!("../input.txt")
        .replace("\r\n", "\n")
        .split("\n")
    {
        let row: Vec<_> = line.split(',').map(|v| v.parse::<i64>().unwrap()).collect();
        boxes.push(Point {
            x: row[0],
            y: row[1],
            z: row[2],
        });
    }

    // part 1
    {
        // pre-compute distances and sort by smallest
        let mut distances = Vec::new();
        for i in 0..boxes.len() {
            for j in i + 1..boxes.len() {
                distances.push((i, j, boxes[i].distance(&boxes[j])));
            }
        }
        distances.sort_by(|(_, _, a_dis), (_, _, b_dis)| a_dis.cmp(b_dis));

        // keep putting cables
        let mut circuits: Vec<Vec<usize>> = Vec::new();
        let mut index = 0;
        for _ in 0..iterations {
            'outer: loop {
                let (box_a, box_b, _) = distances[index];
                for circuit in circuits.iter_mut() {
                    if circuit.contains(&box_a) && circuit.contains(&box_b) {
                        // already in circuit => try next
                        index += 1;
                        continue 'outer;
                    } else if circuit.contains(&box_a) {
                        // partially in circuit
                        circuit.push(box_b);
                        break 'outer;
                    } else if circuit.contains(&box_b) {
                        circuit.push(box_a);
                        break 'outer;
                    }
                }
                // it was not in any circuit => new circuit
                circuits.push(vec![box_a, box_b]);
                break 'outer;
            }
            index += 1;
        }

        let mut sizes: Vec<_> = circuits.iter().map(|v| v.len()).collect();
        sizes.sort();
        dbg!(sizes.iter().rev().take(3).product::<usize>());
    }

    // part 2
    {}

    Ok(())
}

use std::collections::HashSet;

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

    // pre-compute distances and sort by smallest
    let mut distances = Vec::new();
    for i in 0..boxes.len() {
        for j in i + 1..boxes.len() {
            distances.push((i, j, boxes[i].distance(&boxes[j])));
        }
    }
    distances.sort_by(|(_, _, a_dis), (_, _, b_dis)| a_dis.cmp(b_dis));

    // part 1
    {
        // keep putting cables
        let mut circuits: Vec<HashSet<usize>> = Vec::new();
        'outer: for (box_a, box_b, _) in distances.iter().take(iterations) {
            println!("box_a {} box_b {}", box_a, box_b);
            let mut a_in = None;
            let mut b_in = None;
            for (i, circuit) in circuits.iter_mut().enumerate() {
                if circuit.contains(box_a) && circuit.contains(box_b) {
                    // already in same circuit, cannot add cable => try next shortest distance
                    // add cable anyway...
                    continue 'outer;
                } else if circuit.contains(box_a) {
                    // partially in circuit
                    assert!(a_in.is_none());
                    a_in = Some(i);
                } else if circuit.contains(box_b) {
                    // partially in circuit
                    assert!(b_in.is_none());
                    b_in = Some(i);
                }
            }
            // check
            match (a_in, b_in) {
                (None, None) => {
                    // it was not in any circuit => new circuit
                    println!("  >> new circuit");
                    circuits.push(HashSet::from([*box_a, *box_b]));
                }
                (Some(ca), None) => {
                    println!("  >> add {} to circuit {}", box_b, ca);
                    circuits[ca].insert(*box_b);
                }
                (None, Some(cb)) => {
                    println!("  >> add {} to circuit {}", box_a, cb);
                    circuits[cb].insert(*box_a);
                }
                (Some(ca), Some(cb)) => {
                    // remove both circuits from vector
                    let left = std::cmp::min(ca, cb);
                    let right = std::cmp::max(ca, cb);
                    println!("  >> join {} and {}", left, right);
                    let ca = circuits.remove(right);
                    let cb = circuits.remove(left);

                    // join them and add again
                    let union = ca.union(&cb).copied().collect();
                    circuits.push(union);
                }
            }
            println!("  {:?}", circuits);
        }

        let mut sizes: Vec<_> = circuits.iter().map(|v| v.len()).collect();
        sizes.sort();
        dbg!(&sizes);
        dbg!(sizes.iter().rev().take(3).product::<usize>());
    }

    // part 2
    {
        // keep putting cables
        let mut circuits: Vec<HashSet<usize>> = Vec::new();
        'outer: for (box_a, box_b, _) in distances.iter() {
            println!("box_a {} box_b {}", box_a, box_b);
            let mut a_in = None;
            let mut b_in = None;
            for (i, circuit) in circuits.iter_mut().enumerate() {
                if circuit.contains(box_a) && circuit.contains(box_b) {
                    // already in same circuit, cannot add cable => try next shortest distance
                    // add cable anyway...
                    continue 'outer;
                } else if circuit.contains(box_a) {
                    // partially in circuit
                    assert!(a_in.is_none());
                    a_in = Some(i);
                } else if circuit.contains(box_b) {
                    // partially in circuit
                    assert!(b_in.is_none());
                    b_in = Some(i);
                }
            }
            // check
            match (a_in, b_in) {
                (None, None) => {
                    // it was not in any circuit => new circuit
                    println!("  >> new circuit");
                    circuits.push(HashSet::from([*box_a, *box_b]));
                }
                (Some(ca), None) => {
                    println!("  >> add {} to circuit {}", box_b, ca);
                    circuits[ca].insert(*box_b);
                }
                (None, Some(cb)) => {
                    println!("  >> add {} to circuit {}", box_a, cb);
                    circuits[cb].insert(*box_a);
                }
                (Some(ca), Some(cb)) => {
                    // remove both circuits from vector
                    let left = std::cmp::min(ca, cb);
                    let right = std::cmp::max(ca, cb);
                    println!("  >> join {} and {}", left, right);
                    let ca = circuits.remove(right);
                    let cb = circuits.remove(left);

                    // join them and add again
                    let union = ca.union(&cb).copied().collect();
                    circuits.push(union);
                }
            }
            println!("  {:?}", circuits);
            if circuits.len() == 1 && circuits[0].len() == boxes.len() {
                println!("!!! {:?} and {:?}", boxes[*box_a], boxes[*box_b]);
                dbg!(boxes[*box_a].x * boxes[*box_b].x);
                break;
            }
        }

        let mut sizes: Vec<_> = circuits.iter().map(|v| v.len()).collect();
        sizes.sort();
        dbg!(&sizes);
    }

    Ok(())
}

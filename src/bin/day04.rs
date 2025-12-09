#[derive(PartialEq, Eq)]
enum Cell {
    Empty,
    PaperRoll,
}

struct Map {
    data: Vec<Vec<Cell>>,
}

impl Map {
    fn new() -> Self {
        Map { data: Vec::new() }
    }

    fn accessible(&self) -> Vec<(usize, usize)> {
        let mut can_access = Vec::new();
        for r in 0..self.data.len() {
            for c in 0..self.data[r].len() {
                if self.data[r][c] != Cell::PaperRoll {
                    print!(".");
                    continue;
                }

                let mut sum = 0;
                if r > 0 && c > 0 && self.data[r - 1][c - 1] == Cell::PaperRoll {
                    sum += 1; // top-left
                }
                if r > 0 && self.data[r - 1][c] == Cell::PaperRoll {
                    sum += 1; // top
                }
                if r > 0 && c + 1 < self.data[r].len() && self.data[r - 1][c + 1] == Cell::PaperRoll
                {
                    sum += 1; // top-right
                }
                if c > 0 && self.data[r][c - 1] == Cell::PaperRoll {
                    sum += 1; // left
                }
                if c + 1 < self.data[r].len() && self.data[r][c + 1] == Cell::PaperRoll {
                    sum += 1; // right
                }
                if r + 1 < self.data.len() && c > 0 && self.data[r + 1][c - 1] == Cell::PaperRoll {
                    sum += 1; // bottom-left
                }
                if r + 1 < self.data.len() && self.data[r + 1][c] == Cell::PaperRoll {
                    sum += 1; // bottom
                }
                if r + 1 < self.data.len()
                    && c + 1 < self.data[r].len()
                    && self.data[r + 1][c + 1] == Cell::PaperRoll
                {
                    sum += 1; // bottom-right
                }
                if sum < 4 {
                    can_access.push((r, c));
                    print!("x");
                } else {
                    print!("@");
                }
            }
            println!();
        }
        can_access
    }

    fn iterative_remove(&mut self) -> usize {
        let mut sum = 0;
        let mut access = self.accessible();
        while !access.is_empty() {
            // remove
            std::thread::sleep(std::time::Duration::from_millis(50));
            sum += access.len();
            for (r, c) in access {
                self.data[r][c] = Cell::Empty;
            }
            print!("\n---\n\n");
            // check again
            access = self.accessible()
        }
        sum
    }
}

fn main() -> Result<(), std::io::Error> {
    let mut map = Map::new();

    // for line in include_str!("../input_example.txt")
    for line in include_str!("../input.txt")
        .replace("\r\n", "\n")
        .split("\n")
    {
        let row: Vec<_> = line
            .chars()
            .map(|c| match c {
                '.' => Cell::Empty,
                '@' => Cell::PaperRoll,
                _ => panic!("unknown cell"),
            })
            .collect();
        map.data.push(row);
    }

    // part 1
    dbg!(map.accessible().len());
    // part 2
    dbg!(map.iterative_remove());

    Ok(())
}

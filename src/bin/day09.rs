#[derive(Debug)]
struct Tile {
    x: i64,
    y: i64,
}

fn main() -> Result<(), std::io::Error> {
    let mut tiles = Vec::new();

    // for line in include_str!("../input_example.txt")
        for line in include_str!("../input.txt")
        .replace("\r\n", "\n")
        .split("\n")
    {
        let values: Vec<_> = line.split(",").map(|v| v.parse::<i64>().unwrap()).collect();
        tiles.push(Tile {
            x: values[0],
            y: values[1],
        });
    }

    // part 1
    {
        let mut max_area = 0;
        for i in 0..tiles.len() {
            let tile = &tiles[i];
            for other in tiles.iter().skip(i + 1) {
                let dx = (tile.x - other.x).abs() + 1;
                let dy = (tile.y - other.y).abs() + 1;
                let area = dx * dy;
                max_area = max_area.max(area);
            }
        }
        dbg!(max_area);
    }

    // part 2
    {
        // create map
        let mut x_max = 0;
        let mut y_max = 0;
        for tile in tiles.iter() {
            x_max = x_max.max(tile.x as usize);
            y_max = y_max.max(tile.y as usize);
        }
        println!("max {} {}", x_max, y_max);
        let mut map = vec![vec![0u8; y_max + 4]; x_max + 4];

        // paint map borders
        println!("borders");
        for i in 0..tiles.len() {
            let tile = &tiles[i];
            let next = &tiles[(i + 1) % tiles.len()];
            // println!("{:?} {:?}", tile, next);
            if tile.x == next.x {
                for y in tile.y.min(next.y)..=tile.y.max(next.y) {
                    map[tile.x as usize][y as usize] = 1;
                }
            } else {
                assert!(tile.y == next.y);
                for x in tile.x.min(next.x)..=tile.x.max(next.x) {
                    map[x as usize][tile.y as usize] = 1;
                }
            }
        }
        // print_map(&map);

        // paint map interior
        println!("interior");
        let mut queue = std::collections::VecDeque::new();
        queue.push_back(((tiles[0].x + 1) as usize, (tiles[0].y + 1) as usize));
        while !queue.is_empty() {
            // println!("{:?}", &queue);
            let (x, y) = queue.pop_front().unwrap();
            // paint
            map[x][y] = 2;
            // neighbors
            if map[x - 1][y] == 0 {
                queue.push_back((x - 1, y));
            }
            if map[x + 1][y] == 0 {
                queue.push_back((x + 1, y));
            }
            if map[x][y - 1] == 0 {
                queue.push_back((x, y - 1));
            }
            if map[x][y + 1] == 0 {
                queue.push_back((x, y + 1));
            }
        }
        // print_map(&map);

        println!("solve");
        let mut max_area = 0;
        for i in 0..tiles.len() {
            let tile = &tiles[i];
            'outer: for other in tiles.iter().skip(i + 1) {
                let dx = (tile.x - other.x).abs() + 1;
                let dy = (tile.y - other.y).abs() + 1;
                let area = dx * dy;
                if area > max_area {
                    // all ok
                    for x in tile.x.min(other.x)..=tile.x.max(other.x) {
                        for y in tile.y.min(other.y)..=tile.y.max(other.y) {
                            if map[x as usize][y as usize] == 0 {
                                continue 'outer;
                            }
                        }
                    }
                    // update
                    max_area = area;
                }
            }
        }
        dbg!(max_area);
    }

    Ok(())
}

fn print_map(map: &[Vec<u8>]) {
    for row in map.iter() {
        for c in row.iter() {
            print!("{}", c);
        }
        println!();
    }
    println!();
}

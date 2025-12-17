fn main() -> Result<(), std::io::Error> {
    let mut graph = std::collections::HashMap::new();

    // for line in include_str!("../input_example.txt")
        for line in include_str!("../input.txt")
        .replace("\r\n", "\n")
        .split("\n")
    {
        let mut iter = line.split_ascii_whitespace();
        let origin = iter.next().unwrap();
        let origin = origin[..origin.len() - 1].to_owned(); // take out ending ":"

        let others: std::collections::HashSet<_> = iter.map(|v| v.to_owned()).collect();

        graph.insert(origin, others);
    }

    // part 1 - all paths from "you" to "out"
    {
        // let mut queue = std::collections::VecDeque::new();
        // queue.push_back("you");
        // let mut count = 0;
        // while !queue.is_empty() {
        //     let curr = queue.pop_front().unwrap();
        //     for other in graph.get(curr).unwrap() {
        //         if other == "out" {
        //             count += 1;
        //         } else {
        //             queue.push_back(other);
        //         }
        //     }
        // }
        // dbg!(count);
    }

    // part 2 - all paths from "svr" to "out" that visit "dac" and "fft"
    {
        let mut count = 0;
        let mut queue = std::collections::VecDeque::new();
        queue.push_back(("svr", false, false, std::collections::HashSet::new()));
        while !queue.is_empty() {
            // println!("{:?}", &queue);
            let (curr, dac, fft, visited) = queue.pop_front().unwrap();
            let mut vis = visited.clone();
            for other in graph.get(curr).unwrap() {
                if vis.contains(other) {
                    continue;
                }
                vis.insert(other.to_owned());
                match other.as_str() {
                    "out" => {
                        if dac && fft {
                            count += 1;
                        }
                    }
                    "dac" => queue.push_back((other, true, fft, visited.clone())),
                    "fft" => queue.push_back((other, dac, true, visited.clone())),
                    _ => queue.push_back((other, dac, fft, visited.clone())),
                }
            }
        }
        dbg!(count);
    }

    Ok(())
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum Led {
    On,
    Off,
}

impl Led {
    fn toggle(&mut self) {
        if *self == Led::On {
            *self = Led::Off;
        } else {
            *self = Led::On;
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Screen {
    leds: Vec<Led>,
}

impl Screen {
    fn new(size: usize) -> Self {
        Screen {
            leds: vec![Led::Off; size],
        }
    }

    fn from_bools(leds: &[bool]) -> Self {
        Screen {
            leds: leds
                .iter()
                .map(|&v| if v { Led::On } else { Led::Off })
                .collect(),
        }
    }

    fn toggle(&self, button: &[usize]) -> Self {
        let mut leds = self.leds.clone();
        for led in button {
            leds[*led].toggle();
        }
        Self { leds }
    }

    fn is_same(&self, other: &Screen) -> bool {
        for (a, b) in self.leds.iter().zip(other.leds.iter()) {
            if a != b {
                return false;
            }
        }
        true
    }
}

#[derive(Debug, Clone, PartialEq)]
struct Problem {
    screen: Screen,
    screen_goal: Screen,
    buttons: Vec<Vec<usize>>,
    joltage: Vec<usize>,
    joltage_goal: Vec<usize>,
}

impl Problem {
    fn from_str(line: &str) -> Self {
        let mut iter = line.split_whitespace();

        // screen
        let screen = iter.next().unwrap();
        let screen: Vec<_> = screen[1..screen.len() - 1]
            .chars()
            .map(|c| c == '#')
            .collect();

        // buttons and joltage
        let mut buttons = Vec::new();
        let mut joltage = Vec::new();
        for chunk in iter {
            let numbers = chunk[1..chunk.len() - 1]
                .split(",")
                .map(|v| v.parse().unwrap())
                .collect();
            if chunk.starts_with('(') {
                buttons.push(numbers);
            } else {
                joltage = numbers;
            }
        }

        Self {
            screen: Screen::new(screen.len()),
            screen_goal: Screen::from_bools(&screen),
            buttons,
            joltage: vec![0; joltage.len()],
            joltage_goal: joltage,
        }
    }

    fn solve_part1(&self) -> usize {
        if self.screen.is_same(&self.screen_goal) {
            return 0;
        }

        let mut visited = std::collections::HashSet::new();
        visited.insert(self.screen.clone());

        let mut queue = std::collections::VecDeque::new();
        queue.push_back((self.screen.clone(), 0));

        while !queue.is_empty() {
            let (screen, steps) = queue.pop_front().unwrap();
            for b in self.buttons.iter() {
                let new_screen = screen.toggle(b);
                if new_screen.is_same(&self.screen_goal) {
                    return steps + 1;
                } else if !visited.contains(&new_screen) {
                    visited.insert(new_screen.clone());
                    queue.push_back((new_screen, steps + 1));
                }
            }
        }
        0
    }

    fn solve_part2(&self) -> usize {
        if self.joltage == self.joltage_goal {
            return 0;
        }

        let mut visited = std::collections::HashSet::new();
        visited.insert(self.joltage.clone());

        let mut queue = std::collections::VecDeque::new();
        queue.push_back((self.joltage.clone(), 0));

        while !queue.is_empty() {
            let (joltage, steps) = queue.pop_front().unwrap();
            'outer: for b in self.buttons.iter() {
                let mut new_joltage = joltage.clone();
                for v in b.iter() {
                    new_joltage[*v] += 1;
                }
                for (n, goal) in new_joltage.iter().zip(self.joltage_goal.iter()) {
                    if n > goal {
                        continue 'outer;
                    }
                }
                if new_joltage == self.joltage_goal {
                    return steps + 1;
                } else if !visited.contains(&new_joltage) {
                    visited.insert(new_joltage.clone());
                    queue.push_back((new_joltage, steps + 1));
                }
            }
        }
        0
    }
}

fn main() -> Result<(), std::io::Error> {
    let mut problems = Vec::new();

    // for line in include_str!("../input_example.txt")
    for line in include_str!("../input.txt")
        .replace("\r\n", "\n")
        .split("\n")
    {
        problems.push(Problem::from_str(line));
    }
    // dbg!(&problems);

    // part 1
    {
        let mut sum = 0;
        for p in problems.iter() {
            sum += p.solve_part1();
        }
        dbg!(sum);
    }

    // part 2
    {
        let mut sum = 0;
        for p in problems.iter() {
            let v = p.solve_part2();
            // dbg!(&v);
            sum += v;
        }
        dbg!(sum);
    }

    Ok(())
}

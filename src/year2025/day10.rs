/**
--- Day 10: Factory ---

Just across the hall, you find a large factory. Fortunately, the Elves here
have plenty of time to decorate. Unfortunately, it's because the factory
machines are all offline, and none of the Elves can figure out the
initialization procedure.

The Elves do have the manual for the machines, but the section detailing
the initialization procedure was eaten by a Shiba Inu. All that remains of
the manual are some indicator light diagrams, button wiring schematics, and
joltage requirements for each machine.

For example:

    [.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
    [...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
    [.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}

The manual describes one machine per line. Each line contains a single
indicator light diagram in [square brackets], one or more button wiring
schematics in (parentheses), and joltage requirements in {curly braces}.

To start a machine, its indicator lights must match those shown in the
diagram, where . means off and # means on. The machine has the number of
indicator lights shown, but its indicator lights are all initially off.

So, an indicator light diagram like [.##.] means that the machine has four
indicator lights which are initially off and that the goal is to
simultaneously configure the first light to be off, the second light to be
on, the third to be on, and the fourth to be off.

You can toggle the state of indicator lights by pushing any of the listed
buttons. Each button lists which indicator lights it toggles, where 0 means
the first light, 1 means the second light, and so on. When you push a
button, each listed indicator light either turns on (if it was off) or
turns off (if it was on). You have to push each button an integer number of
times; there's no such thing as "0.5 presses" (nor can you push a button a
negative number of times).

So, a button wiring schematic like (0,3,4) means that each time you push
that button, the first, fourth, and fifth indicator lights would all toggle
between on and off. If the indicator lights were [#.....], pushing the
button would change them to be [...##.] instead.

Because none of the machines are running, the joltage requirements are
irrelevant and can be safely ignored.

You can push each button as many times as you like. However, to save on
time, you will need to determine the fewest total presses required to
correctly configure all indicator lights for all machines in your list.

There are a few ways to correctly configure the first machine:

    [.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}

    You could press the first three buttons once each, a total of 3 button
    presses.
    You could press (1,3) once, (2,3) once, and (0,1) twice, a total of 4
    button presses.
    You could press all of the buttons except (1,3) once each, a total of
    5 button presses.

However, the fewest button presses required is 2. One way to do this is by
pressing the last two buttons ((0,2) and (0,1)) once each.

The second machine can be configured with as few as 3 button presses:

    [...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}

One way to achieve this is by pressing the last three buttons ((0,4),
(0,1,2), and (1,2,3,4)) once each.

The third machine has a total of six indicator lights that need to be
configured correctly:

[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}

The fewest presses required to correctly configure it is 2; one way to do
this is by pressing buttons (0,3,4) and (0,1,2,4,5) once each.

So, the fewest button presses required to correctly configure the indicator
lights on all of the machines is 2 + 3 + 2 = 7.

Analyze each machine's indicator light diagram and button wiring
schematics. What is the fewest button presses required to correctly
configure the indicator lights on all of the machines?

--- Part Two ---

All of the machines are starting to come online! Now, it's time to worry
about the joltage requirements.

Each machine needs to be configured to exactly the specified joltage levels
to function properly. Below the buttons on each machine is a big lever that
you can use to switch the buttons from configuring the indicator lights to
increasing the joltage levels. (Ignore the indicator light diagrams.)

The machines each have a set of numeric counters tracking its joltage
levels, one counter per joltage requirement. The counters are all initially
set to zero.

So, joltage requirements like {3,5,4,7} mean that the machine has four
counters which are initially 0 and that the goal is to simultaneously
configure the first counter to be 3, the second counter to be 5, the third
to be 4, and the fourth to be 7.

The button wiring schematics are still relevant: in this new joltage
configuration mode, each button now indicates which counters it affects,
where 0 means the first counter, 1 means the second counter, and so on.
When you push a button, each listed counter is increased by 1.

So, a button wiring schematic like (1,3) means that each time you push that
button, the second and fourth counters would each increase by 1. If the
current joltage levels were {0,1,2,3}, pushing the button would change them
to be {0,2,2,4}.

You can push each button as many times as you like. However, your finger is
getting sore from all the button pushing, and so you will need to determine
the fewest total presses required to correctly configure each machine's
joltage level counters to match the specified joltage requirements.

Consider again the example from before:

    [.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
    [...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
    [.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}

Configuring the first machine's counters requires a minimum of 10 button
presses. One way to do this is by pressing (3) once, (1,3) three times,
(2,3) three times, (0,2) once, and (0,1) twice.

Configuring the second machine's counters requires a minimum of 12 button
presses. One way to do this is by pressing (0,2,3,4) twice, (2,3) five
times, and (0,1,2) five times.

Configuring the third machine's counters requires a minimum of 11 button
presses. One way to do this is by pressing (0,1,2,3,4) five times,
(0,1,2,4,5) five times, and (1,2) once.

So, the fewest button presses required to correctly configure the joltage
level counters on all of the machines is 10 + 12 + 11 = 33.

Analyze each machine's joltage requirements and button wiring schematics.
What is the fewest button presses required to correctly configure the
joltage level counters on all of the machines?
*/
#[cfg(test)]
mod tests {
    use crate::utils::files::read_lines;
    use itertools::Itertools;
    use std::path::Path;

    #[test]
    fn input_example() {
        let input = r#"[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}"#;

        let lines = input.split("\n").into_iter().map(str::to_string);
        let sum = part_one(lines.into_iter());
        assert_eq!(sum, 7);

        let lines = input.split("\n").into_iter().map(str::to_string);
        let sum = part_two(lines.into_iter());
        assert_eq!(sum, 33);
    }

    fn part_one<T>(lines: T) -> usize
    where
        T: Iterator<Item = String>,
    {
        lines.map(|line| solve_part_one(line)).sum::<usize>()
    }

    fn solve_part_one(line: String) -> usize {
        let (mask, button_masks, _) = read_line(line);
        println!("button_masks = {:?}", button_masks);

        let all_mask_count = all_mask(button_masks.len());
        println!("all_mask = {}", all_mask_count);

        let mut min_button = usize::MAX;
        for vec in button_masks.into_iter().powerset() {
            let on = vec.iter().fold(0, |acc, &n| acc ^ n);
            if mask == on {
                println!("m {:?} -> lights = {:?}", vec, on);
                min_button = std::cmp::min(min_button, vec.len());
            }
        }
        println!("min_button = {}", min_button);
        min_button
    }

    fn part_two<T>(lines: T) -> usize
    where
        T: Iterator<Item = String>,
    {
        lines.map(|line| solve_part_two(line)).sum::<usize>()
    }

    fn solve_part_two(line: String) -> usize {
        let (_, button_masks, jolts) = read_line(line);

        let mut problem = Problem::new(OptimizationDirection::Minimize);
        let max = jolts.iter().max().unwrap().clone();

        let variables = (0..button_masks.len())
            .map(|_| problem.add_integer_var(1.0, (0, max)))
            .collect::<Vec<_>>();

        for (i, &n) in jolts.iter().enumerate() {
            problem.add_constraint(
                button_masks
                    .iter()
                    .zip(&variables)
                    .filter(|&(mask, _)| mask & (1 << i) != 0)
                    .fold(LinearExpr::empty(), |mut ex, (_, &var)| {
                        ex.add(var, 1.0);
                        ex
                    }),
                microlp::ComparisonOp::Eq,
                n as f64,
            );
        }
        let solve = problem.solve().unwrap().objective().round() as usize;
        solve
    }

    fn read_line(line: String) -> (u64, Vec<u64>, Vec<i32>) {
        let split: Vec<String> = line.split_whitespace().map(str::to_string).collect();
        let len = split.len();
        println!("{:?}", split);
        let first = split.first().unwrap();
        let mask = read_mask(&first);
        println!("mask = {}", mask);
        let button_masks: Vec<u64> = (1..len - 1)
            .map(|i| split[i].clone())
            .map(|s| read_button_mask(&s))
            .collect();
        let last = split.last().unwrap();
        let jolts = read_jolts(&last);
        (mask, button_masks, jolts)
    }

    fn all_mask(len: usize) -> u64 {
        let mut all_mask: u64 = 0;
        for i in 0..len {
            all_mask |= 1 << i;
        }
        all_mask
    }

    fn read_mask(s: &String) -> u64 {
        let mut result = 0;
        for (pos, char) in s.chars().enumerate() {
            if char == '#' {
                result |= 1 << (pos - 1) as u64;
            }
        }
        result
    }

    fn read_button_mask(s: &String) -> u64 {
        let mut result = 0;
        let x = &s[1..s.len() - 1];
        for c in x.split(',') {
            let n = c.parse::<u8>().unwrap();
            result |= 1 << n as u64;
        }
        result
    }

    fn read_jolts(s: &String) -> Vec<i32> {
        let mut result = Vec::new();
        for c in s[1..s.len() - 1].split(',') {
            result.push(c.parse::<i32>().unwrap());
        }
        result
    }

    #[test]
    fn input_part_1() {
        let path = Path::new("data/day10.txt");

        let lines = read_lines(path);

        let sum = part_one(lines.into_iter());

        assert_eq!(sum, 486);
    }

    use microlp::{LinearExpr, OptimizationDirection, Problem};

    #[test]
    fn input_part_2() {
        let path = Path::new("data/day10.txt");

        let lines = read_lines(path);

        let result = part_two(lines.into_iter());

        assert_eq!(result, 17820);
    }
}

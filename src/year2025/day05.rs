#[derive(Debug, Clone)]
struct Range {
    lower: u64,
    upper: u64,
}

impl Range {
    pub fn new(lower: u64, upper: u64) -> Range {
        Range { lower, upper }
    }

    pub fn intersect(&self, right: &Range) -> bool {
        let lower_max = std::cmp::max(self.lower, right.lower);
        let upper_min = std::cmp::min(self.upper, right.upper);

        lower_max <= upper_min
    }

    pub fn union(&self, other: &Range) -> Range {
        let lower_min = std::cmp::min(self.lower, other.lower);
        let upper_max = std::cmp::max(self.upper, other.upper);

        Range::new(lower_min, upper_max)
    }

    pub fn contains(&self, id: u64) -> bool {
        id >= self.lower && id <= self.upper
    }

    pub fn len(&self) -> u64 {
        self.upper - self.lower + 1
    }
}

/**
--- Day 5: Cafeteria ---

As the forklifts break through the wall, the Elves are delighted to
discover that there was a cafeteria on the other side after all.

You can hear a commotion coming from the kitchen. "At this rate, we won't
have any time left to put the wreaths up in the dining hall!" Resolute in
your quest, you investigate.

"If only we hadn't switched to the new inventory management system right
before Christmas!" another Elf exclaims. You ask what's going on.

The Elves in the kitchen explain the situation: because of their
complicated new inventory management system, they can't figure out which of
their ingredients are fresh and which are spoiled. When you ask how it
works, they give you a copy of their database (your puzzle input).

The database operates on ingredient IDs. It consists of a list of fresh
ingredient ID ranges, a blank line, and a list of available ingredient IDs.
For example:

    3-5
    10-14
    16-20
    12-18

    1
    5
    8
    11
    17
    32

The fresh ID ranges are inclusive: the range 3-5 means that ingredient IDs
3, 4, and 5 are all fresh. The ranges can also overlap; an ingredient ID is
fresh if it is in any range.

The Elves are trying to determine which of the available ingredient IDs are
fresh. In this example, this is done as follows:

    Ingredient ID 1 is spoiled because it does not fall into any range.
    Ingredient ID 5 is fresh because it falls into range 3-5.
    Ingredient ID 8 is spoiled.
    Ingredient ID 11 is fresh because it falls into range 10-14.
    Ingredient ID 17 is fresh because it falls into range 16-20 as well as
    range 12-18.
    Ingredient ID 32 is spoiled.

So, in this example, 3 of the available ingredient IDs are fresh.

Process the database file from the new inventory management system. How
many of the available ingredient IDs are fresh?

--- Part Two ---

The Elves start bringing their spoiled inventory to the trash chute at the
back of the kitchen.

So that they can stop bugging you when they get new inventory, the Elves
would like to know all of the IDs that the fresh ingredient ID ranges
consider to be fresh. An ingredient ID is still considered fresh if it is
in any range.

Now, the second section of the database (the available ingredient IDs) is
irrelevant. Here are the fresh ingredient ID ranges from the above example:

    3-5
    10-14
    16-20
    12-18

The ingredient IDs that these ranges consider to be fresh are 3, 4, 5, 10,
11, 12, 13, 14, 15, 16, 17, 18, 19, and 20. So, in this example, the fresh
ingredient ID ranges consider a total of 14 ingredient IDs to be fresh.

Process the database file again. How many ingredient IDs are considered to
be fresh according to the fresh ingredient ID ranges?
*/
#[cfg(test)]
mod tests {
    use crate::utils::files::read_lines;
    use crate::year2025::day05::Range;
    use regex::Regex;
    use std::collections::VecDeque;
    use std::path::Path;

    #[test]
    fn input_example() {
        let input = r#"3-5
10-14
16-20
12-18

1
5
8
11
17
32"#;

        let lines = input.split("\n").into_iter().map(str::to_string);

        let fresh = cafeteria_part1(lines);

        assert_eq!(fresh, 3);

        let lines = input.split("\n").into_iter().map(str::to_string);

        let fresh = cafeteria_part2(lines);

        assert_eq!(fresh, 14);
    }

    fn cafeteria_part1<T>(lines: T) -> i32
    where
        T: Iterator<Item = String>,
    {
        let regex = Regex::new(r"(\d+)-(\d+)").unwrap();

        let mut ranges: Vec<Range> = Vec::new();
        let mut ids: Vec<u64> = Vec::new();

        for line in lines {
            let capture = regex.captures(&line);
            if capture.is_some() {
                let unwrap = capture.unwrap();
                ranges.push(Range::new(
                    unwrap[1].parse().unwrap(),
                    unwrap[2].parse().unwrap(),
                ));
            } else if !line.is_empty() {
                ids.push(line.parse().unwrap());
            }
        }

        println!("Ranges: {:?}", ranges);
        println!("Ids: {:?}", ids);

        let mut fresh = 0;
        for id in ids {
            if ranges.iter().any(|range| range.contains(id)) {
                println!("Fresh id: {:?}", id);
                fresh += 1;
            }
        }
        fresh
    }

    fn cafeteria_part2<T>(lines: T) -> u64
    where
        T: Iterator<Item = String>,
    {
        let regex = Regex::new(r"(\d+)-(\d+)").unwrap();

        let mut ranges: Vec<Range> = Vec::new();

        for line in lines {
            let capture = regex.captures(&line);
            if capture.is_some() {
                let unwrap = capture.unwrap();
                ranges.push(Range::new(
                    unwrap[1].parse().unwrap(),
                    unwrap[2].parse().unwrap(),
                ));
            } else {
                break;
            }
        }

        println!("Ranges: {:?}", ranges);

        let mut deque: VecDeque<Option<Range>> = VecDeque::new();
        for r in ranges {
            deque.push_back(Some(r));
        }
        loop {
            let size = deque.len();
            let mut found = false;
            for i in 0..size {
                match &deque[i] {
                    None => {}
                    Some(range1) => {
                        let mut new_range = range1.clone();
                        for j in i + 1..size {
                            match &deque[j] {
                                None => {}
                                Some(range2) => {
                                    if new_range.intersect(&range2) {
                                        new_range = new_range.union(&range2);
                                        deque[j] = None;
                                        found = true;
                                    }
                                }
                            }
                            deque[i] = Some(new_range.clone());
                        }
                    }
                }
            }

            if found {
                deque.retain(|s| s.is_some());
            } else {
                break;
            }

            println!("Ranges: {:?}", deque);
        }

        deque
            .iter()
            .map(Option::as_ref)
            .map(Option::unwrap)
            .map(Range::len)
            .sum()
    }

    #[test]
    fn input_part_1() {
        let path = Path::new("data/day05.txt");

        let lines = read_lines(path);

        let fresh = cafeteria_part1(lines.into_iter());

        assert_eq!(fresh, 712);
    }

    #[test]
    fn input_part_2() {
        let path = Path::new("data/day05.txt");

        let lines = read_lines(path);

        let fresh = cafeteria_part2(lines.into_iter());

        assert_eq!(fresh, 332998283036769);
    }
}

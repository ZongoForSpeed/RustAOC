
/**
--- Day 4: Printing Department ---

You ride the escalator down to the printing department. They're clearly
getting ready for Christmas; they have lots of large rolls of paper
everywhere, and there's even a massive printer in the corner (to handle the
really big print jobs).

Decorating here will be easy: they can make their own decorations. What you
really need is a way to get further into the North Pole base while the
elevators are offline.

"Actually, maybe we can help with that," one of the Elves replies when you
ask for help. "We're pretty sure there's a cafeteria on the other side of
the back wall. If we could break through the wall, you'd be able to keep
moving. It's too bad all of our forklifts are so busy moving those big
rolls of paper around."

If you can optimize the work the forklifts are doing, maybe they would have
time to spare to break through the wall.

The rolls of paper (@) are arranged on a large grid; the Elves even have a
helpful diagram (your puzzle input) indicating where everything is located.

For example:

    ..@@.@@@@.
    @@@.@.@.@@
    @@@@@.@.@@
    @.@@@@..@.
    @@.@@@@.@@
    .@@@@@@@.@
    .@.@.@.@@@
    @.@@@.@@@@
    .@@@@@@@@.
    @.@.@@@.@.

The forklifts can only access a roll of paper if there are fewer than four
rolls of paper in the eight adjacent positions. If you can figure out which
rolls of paper the forklifts can access, they'll spend less time looking
and more time breaking down the wall to the cafeteria.

In this example, there are 13 rolls of paper that can be accessed by a
forklift (marked with x):

    ..xx.xx@x.
    x@@.@.@.@@
    @@@@@.x.@@
    @.@@@@..@.
    x@.@@@@.@x
    .@@@@@@@.@
    .@.@.@.@@@
    x.@@@.@@@@
    .@@@@@@@@.
    x.x.@@@.x.

Consider your complete diagram of the paper roll locations. How many rolls
of paper can be accessed by a forklift?

--- Part Two ---

Now, the Elves just need help accessing as much of the paper as they can.

Once a roll of paper can be accessed by a forklift, it can be removed. Once
a roll of paper is removed, the forklifts might be able to access more
rolls of paper, which they might also be able to remove. How many total
rolls of paper could the Elves remove if they keep repeating this process?

Starting with the same example as above, here is one way you could remove
as many rolls of paper as possible, using highlighted @ to indicate that a
roll of paper is about to be removed, and using x to indicate that a roll
of paper was just removed:

    Initial state:
    ..@@.@@@@.
    @@@.@.@.@@
    @@@@@.@.@@
    @.@@@@..@.
    @@.@@@@.@@
    .@@@@@@@.@
    .@.@.@.@@@
    @.@@@.@@@@
    .@@@@@@@@.
    @.@.@@@.@.

    Remove 13 rolls of paper:
    ..xx.xx@x.
    x@@.@.@.@@
    @@@@@.x.@@
    @.@@@@..@.
    x@.@@@@.@x
    .@@@@@@@.@
    .@.@.@.@@@
    x.@@@.@@@@
    .@@@@@@@@.
    x.x.@@@.x.

    Remove 12 rolls of paper:
    .......x..
    .@@.x.x.@x
    x@@@@...@@
    x.@@@@..x.
    .@.@@@@.x.
    .x@@@@@@.x
    .x.@.@.@@@
    ..@@@.@@@@
    .x@@@@@@@.
    ....@@@...

    Remove 7 rolls of paper:
    ..........
    .x@.....x.
    .@@@@...xx
    ..@@@@....
    .x.@@@@...
    ..@@@@@@..
    ...@.@.@@x
    ..@@@.@@@@
    ..x@@@@@@.
    ....@@@...

    Remove 5 rolls of paper:
    ..........
    ..x.......
    .x@@@.....
    ..@@@@....
    ...@@@@...
    ..x@@@@@..
    ...@.@.@@.
    ..x@@.@@@x
    ...@@@@@@.
    ....@@@...

    Remove 2 rolls of paper:
    ..........
    ..........
    ..x@@.....
    ..@@@@....
    ...@@@@...
    ...@@@@@..
    ...@.@.@@.
    ...@@.@@@.
    ...@@@@@x.
    ....@@@...

    Remove 1 roll of paper:
    ..........
    ..........
    ...@@.....
    ..x@@@....
    ...@@@@...
    ...@@@@@..
    ...@.@.@@.
    ...@@.@@@.
    ...@@@@@..
    ....@@@...

    Remove 1 roll of paper:
    ..........
    ..........
    ...x@.....
    ...@@@....
    ...@@@@...
    ...@@@@@..
    ...@.@.@@.
    ...@@.@@@.
    ...@@@@@..
    ....@@@...

    Remove 1 roll of paper:
    ..........
    ..........
    ....x.....
    ...@@@....
    ...@@@@...
    ...@@@@@..
    ...@.@.@@.
    ...@@.@@@.
    ...@@@@@..
    ....@@@...

    Remove 1 roll of paper:
    ..........
    ..........
    ..........
    ...x@@....
    ...@@@@...
    ...@@@@@..
    ...@.@.@@.
    ...@@.@@@.
    ...@@@@@..
    ....@@@...

Stop once no more rolls of paper are accessible by a forklift. In this
example, a total of 43 rolls of paper can be removed.

Start with your original diagram. How many rolls of paper in total can be
removed by the Elves and their forklifts?

*/
#[cfg(test)]
mod tests {
    use crate::utils::files::read_lines;
    use crate::utils::point::{Map, Point2D};
    use std::collections::HashSet;
    use std::path::Path;
    use itertools::iproduct;

    #[test]
    fn input_example() {
        let input = r#"..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@."#;

        let lines = input.split("\n");
        let forklift = part_one(lines.into_iter().map(str::to_string));

        assert_eq!(forklift, 13);

        let lines = input.split("\n");
        let forklift = part_two(lines.into_iter().map(str::to_string));

        assert_eq!(forklift, 43);
    }

    fn part_one<T>(lines: T) -> usize
    where
        T: Iterator<Item = String>,
    {
        let map_vector: Vec<Vec<char>> = lines.map(|l| l.chars().collect()).collect();
        let map = Map::new(map_vector);

        forklift(&map).len()
    }

    fn part_two<T>(lines: T) -> usize
    where
        T: Iterator<Item = String>,
    {
        let map_vector: Vec<Vec<char>> = lines.map(|l| l.chars().collect()).collect();
        let mut map = Map::new(map_vector);

        let mut rolls = 0;
        loop {
            let to_remove = forklift(&map);
            if to_remove.is_empty() {
                break;
            }

            rolls += to_remove.len();

            for x in to_remove {
                map.set(&x, ',');
            }
        }

        rolls
    }

    fn forklift(map: &Map) -> HashSet<Point2D> {
        let mut forklift = HashSet::new();
        for (x, y) in iproduct!(0..map.x_max(), 0..map.y_max()) {
            let p = Point2D::new(x, y);
            if map.get(&p).unwrap() == '@' {
                let count = p
                    .neighbors()
                    .iter()
                    .filter(|p| p.valid(map.x_max(), map.x_max()))
                    .map(|p| map.get(p))
                    .filter(|c| c.unwrap() == '@')
                    .count();

                if count < 4 {
                    forklift.insert(p);
                }
            }
        }
        forklift
    }

    #[test]
    fn input_part_1() {
        let path = Path::new("data/day04.txt");

        let lines = read_lines(path);

        let forklift = part_one(lines.into_iter());

        assert_eq!(forklift, 1419);
    }

    #[test]
    fn input_part_2() {
        let path = Path::new("data/day04.txt");

        let lines = read_lines(path);

        let forklift = part_two(lines.into_iter());

        assert_eq!(forklift, 8739);
    }
}

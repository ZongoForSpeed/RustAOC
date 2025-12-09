/**
--- Day 9: Movie Theater ---

You slide down the firepole in the corner of the playground and land in the
North Pole base movie theater!

The movie theater has a big tile floor with an interesting pattern. Elves
here are redecorating the theater by switching out some of the square tiles
in the big grid they form. Some of the tiles are red; the Elves would like
to find the largest rectangle that uses red tiles for two of its opposite
corners. They even have a list of where the red tiles are located in the
grid (your puzzle input).

For example:

    7,1
    11,1
    11,7
    9,7
    9,5
    2,5
    2,3
    7,3

Showing red tiles as # and other tiles as ., the above arrangement of red
tiles would look like this:

    ..............
    .......#...#..
    ..............
    ..#....#......
    ..............
    ..#......#....
    ..............
    .........#.#..
    ..............

You can choose any two red tiles as the opposite corners of your rectangle;
your goal is to find the largest rectangle possible.

For example, you could make a rectangle (shown as O) with an area of 24
between 2,5 and 9,7:

    ..............
    .......#...#..
    ..............
    ..#....#......
    ..............
    ..OOOOOOOO....
    ..OOOOOOOO....
    ..OOOOOOOO.#..
    ..............

Or, you could make a rectangle with area 35 between 7,1 and 11,7:

    ..............
    .......OOOOO..
    .......OOOOO..
    ..#....OOOOO..
    .......OOOOO..
    ..#....OOOOO..
    .......OOOOO..
    .......OOOOO..
    ..............

You could even make a thin rectangle with an area of only 6 between 7,3 and
2,3:

    ..............
    .......#...#..
    ..............
    ..OOOOOO......
    ..............
    ..#......#....
    ..............
    .........#.#..
    ..............

Ultimately, the largest rectangle you can make in this example has area 50.
One way to do this is between 2,5 and 11,1:

    ..............
    ..OOOOOOOOOO..
    ..OOOOOOOOOO..
    ..OOOOOOOOOO..
    ..OOOOOOOOOO..
    ..OOOOOOOOOO..
    ..............
    .........#.#..
    ..............

Using two red tiles as opposite corners, what is the largest area of any
rectangle you can make?

--- Part Two ---

The Elves just remembered: they can only switch out tiles that are red or
green. So, your rectangle can only include red or green tiles.

In your list, every red tile is connected to the red tile before and after
it by a straight line of green tiles. The list wraps, so the first red tile
is also connected to the last red tile. Tiles that are adjacent in your
list will always be on either the same row or the same column.

Using the same example as before, the tiles marked X would be green:

    ..............
    .......#XXX#..
    .......X...X..
    ..#XXXX#...X..
    ..X........X..
    ..#XXXXXX#.X..
    .........X.X..
    .........#X#..
    ..............

In addition, all of the tiles inside this loop of red and green tiles are
also green. So, in this example, these are the green tiles:

    ..............
    .......#XXX#..
    .......XXXXX..
    ..#XXXX#XXXX..
    ..XXXXXXXXXX..
    ..#XXXXXX#XX..
    .........XXX..
    .........#X#..
    ..............

The remaining tiles are never red nor green.

The rectangle you choose still must have red tiles in opposite corners, but
any other tiles it includes must now be red or green. This significantly
limits your options.

For example, you could make a rectangle out of red and green tiles with an
area of 15 between 7,3 and 11,1:

    ..............
    .......OOOOO..
    .......OOOOO..
    ..#XXXXOOOOO..
    ..XXXXXXXXXX..
    ..#XXXXXX#XX..
    .........XXX..
    .........#X#..
    ..............

Or, you could make a thin rectangle with an area of 3 between 9,7 and 9,5:

    ..............
    .......#XXX#..
    .......XXXXX..
    ..#XXXX#XXXX..
    ..XXXXXXXXXX..
    ..#XXXXXXOXX..
    .........OXX..
    .........OX#..
    ..............

The largest rectangle you can make in this example using only red and green
tiles has area 24. One way to do this is between 9,5 and 2,3:

    ..............
    .......#XXX#..
    .......XXXXX..
    ..OOOOOOOOXX..
    ..OOOOOOOOXX..
    ..OOOOOOOOXX..
    .........XXX..
    .........#X#..
    ..............

Using two red tiles as opposite corners, what is the largest area of any
rectangle you can make using only red and green tiles?
*/
#[cfg(test)]
mod tests {
    use crate::utils::files::read_lines;
    use crate::utils::point::{Point2D, Rectangle};
    use std::path::Path;

    use itertools::Itertools;

    #[test]
    fn input_example() {
        let input = r#"7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3"#;

        let lines = input.split("\n").into_iter().map(str::to_string);

        let a_max = max_area_part1(lines.into_iter());
        assert_eq!(a_max, 50);

        let lines = input.split("\n").into_iter().map(str::to_string);

        let a_max = max_area_part2(lines.into_iter());
        assert_eq!(a_max, 24);
    }

    fn max_area_part1<T>(iter: T) -> i64
    where
        T: Iterator<Item = String>,
    {
        let position = read_input(iter);

        let mut a_max = 0;
        for x in position.iter().combinations(2) {
            let rectangle = Rectangle::from(x[0], x[1]);
            a_max = std::cmp::max(rectangle.area(), a_max);
        }
        a_max
    }

    fn max_area_part2<T>(iter: T) -> i64
    where
        T: Iterator<Item = String>,
    {
        let position = read_input(iter);
        let edges = find_egdes(&position);

        let mut a_max = 0;
        for c in position.iter().combinations(2) {
            let rectangle = Rectangle::from(c[0], c[1]);
            if !edges.iter().any(|e| rectangle.overlaps_with(e)) {
                a_max = std::cmp::max(a_max, rectangle.area());
            }
        }
        a_max
    }

    fn find_egdes(position: &Vec<Point2D>) -> Vec<Rectangle> {
        let mut edges: Vec<Rectangle> = Vec::new();

        let len = position.len();
        for i in 1..len {
            edges.push(Rectangle::from(&position[i - 1], &position[i]));
        }
        edges.push(Rectangle::from(&position[0], &position[len - 1]));
        edges
    }

    fn read_input<T>(iter: T) -> Vec<Point2D>
    where
        T: Iterator<Item = String>,
    {
        let mut boxes: Vec<Point2D> = Vec::new();
        for line in iter {
            let v: Vec<i64> = line
                .split(",")
                .into_iter()
                .map(str::parse::<i64>)
                .map(Result::unwrap)
                .collect();
            boxes.push(Point2D::new(v[0], v[1]));
        }
        boxes
    }

    #[test]
    fn input_part_1() {
        let path = Path::new("data/day09.txt");

        let lines = read_lines(path);
        let a_max = max_area_part1(lines.into_iter());

        assert_eq!(a_max, 4776100539);
    }

    #[test]
    fn input_part_2() {
        let path = Path::new("data/day09.txt");
        let lines = read_lines(path);

        let a_max = max_area_part2(lines.into_iter());
        assert_eq!(a_max, 1476550548);
    }
}

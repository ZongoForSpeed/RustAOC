
/**
--- Day 6: Trash Compactor ---

After helping the Elves in the kitchen, you were taking a break and helping
them re-enact a movie scene when you over-enthusiastically jumped into the
garbage chute!

A brief fall later, you find yourself in a garbage smasher. Unfortunately,
the door's been magnetically sealed.

As you try to find a way out, you are approached by a family of
cephalopods! They're pretty sure they can get the door open, but it will
take some time. While you wait, they're curious if you can help the
youngest cephalopod with her math homework.

Cephalopod math doesn't look that different from normal math. The math
worksheet (your puzzle input) consists of a list of problems; each problem
has a group of numbers that need to be either added (+) or multiplied (*)
together.

However, the problems are arranged a little strangely; they seem to be
presented next to each other in a very long horizontal list. For example:

    123 328  51 64
     45 64  387 23
      6 98  215 314
    *   +   *   +

Each problem's numbers are arranged vertically; at the bottom of the
problem is the symbol for the operation that needs to be performed.
Problems are separated by a full column of only spaces. The left/right
alignment of numbers within each problem can be ignored.

So, this worksheet contains four problems:

    123 * 45 * 6 = 33210
    328 + 64 + 98 = 490
    51 * 387 * 215 = 4243455
    64 + 23 + 314 = 401

To check their work, cephalopod students are given the grand total of
adding together all of the answers to the individual problems. In this
worksheet, the grand total is 33210 + 490 + 4243455 + 401 = 4277556.

Of course, the actual worksheet is much wider. You'll need to make sure to
unroll it completely so that you can read the problems clearly.

Solve the problems on the math worksheet. What is the grand total found by
adding together all of the answers to the individual problems?

--- Part Two ---

The big cephalopods come back to check on how things are going. When they
see that your grand total doesn't match the one expected by the worksheet,
they realize they forgot to explain how to read cephalopod math.

Cephalopod math is written right-to-left in columns. Each number is given
in its own column, with the most significant digit at the top and the least
significant digit at the bottom. (Problems are still separated with a
column consisting only of spaces, and the symbol at the bottom of the
problem is still the operator to use.)

Here's the example worksheet again:

    123 328  51 64
     45 64  387 23
      6 98  215 314
    *   +   *   +

Reading the problems right-to-left one column at a time, the problems are
now quite different:

    The rightmost problem is 4 + 431 + 623 = 1058
    The second problem from the right is 175 * 581 * 32 = 3253600
    The third problem from the right is 8 + 248 + 369 = 625
    Finally, the leftmost problem is 356 * 24 * 1 = 8544

Now, the grand total is 1058 + 3253600 + 625 + 8544 = 3263827.

Solve the problems on the math worksheet again. What is the grand total
found by adding together all of the answers to the individual problems?
*/
#[cfg(test)]
mod tests {
    use crate::utils::files::read_lines;
    use std::collections::VecDeque;
    use std::path::Path;
    use string_builder::Builder;
    #[test]
    fn input_example() {
        let input = r#"123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  "#;

        let lines = input.split("\n").into_iter().map(str::to_string);

        let result = trash_compactor_part_one(lines);

        assert_eq!(result, 4277556);

        let lines = input.split("\n").into_iter().map(str::to_string);

        let result = trash_compactor_part_two(lines);

        assert_eq!(result, 3263827);
    }

    fn trash_compactor_part_two<T>(lines: T) -> u64
    where
        T: Iterator<Item = String>,
    {
        let rev_lines: Vec<String> = lines.map(|s| s.chars().rev().collect::<String>()).collect();

        let len = rev_lines.iter().map(String::len).max().unwrap();
        let mut builder = Builder::default();
        for i in 0..len {
            rev_lines
                .iter()
                .map(|l| l.chars().nth(i))
                .map(Option::unwrap)
                .for_each(|c| builder.append(c));
        }

        let mut queue: VecDeque<u64> = VecDeque::new();
        let mut result: u64 = 0;
        for x in builder
            .string()
            .unwrap()
            .split_whitespace()
            .map(String::from)
        {
            if x.ends_with('*') {
                match x[..x.len() - 1].parse() {
                    Ok(v) => queue.push_back(v),
                    Err(_) => {}
                };
                result += queue.iter().map(|a| *a).reduce(|a, b| a * b).unwrap();
                queue.clear();
            } else if x.ends_with('+') {
                match x[..x.len() - 1].parse() {
                    Ok(v) => queue.push_back(v),
                    Err(_) => {}
                };
                result += queue.iter().map(|a| *a).reduce(|a, b| a + b).unwrap();
                queue.clear();
            } else {
                queue.push_back(x.parse().unwrap());
            }
        }
        result
    }

    fn trash_compactor_part_one<T>(lines: T) -> u64
    where
        T: Iterator<Item = String>,
    {
        let (numbers, operations) = read_input(lines);

        let mut result: u64 = 0;
        for (i, o) in operations.iter().enumerate() {
            match o.as_str() {
                "+" => {
                    result += numbers.iter().map(|n| n[i]).reduce(|a, b| a + b).unwrap();
                }
                "*" => {
                    result += numbers.iter().map(|n| n[i]).reduce(|a, b| a * b).unwrap();
                }
                &_ => todo!(),
            }
        }
        result
    }

    fn read_input<T>(lines: T) -> (Vec<Vec<u64>>, Vec<String>)
    where
        T: Iterator<Item = String>,
    {
        let mut numbers: Vec<Vec<u64>> = Vec::new();
        let mut operations: Vec<String> = Vec::new();
        for line in lines {
            if line.contains("*") || line.contains("+") {
                line.split_whitespace()
                    .into_iter()
                    .map(str::to_string)
                    .for_each(|o| operations.push(o));

                println!("Operations: {:?}", operations)
            } else {
                let x: Vec<u64> = line
                    .split_whitespace()
                    .into_iter()
                    .map(str::parse)
                    .map(Result::unwrap)
                    .collect();
                println!("Values: {:?}", x);
                numbers.push(x);
            }
        }
        (numbers, operations)
    }

    #[test]
    fn input_part_1() {
        let path = Path::new("data/day06.txt");

        let lines = read_lines(path);

        let result = trash_compactor_part_one(lines.into_iter());

        assert_eq!(result, 5667835681547);
    }

    #[test]
    fn input_part_2() {
        let path = Path::new("data/day06.txt");

        let lines = read_lines(path);

        let result = trash_compactor_part_two(lines.into_iter());

        assert_eq!(result, 9434900032651);
    }
}

#[cfg(test)]
mod tests {
    use regex::Regex;
    use std::collections::HashSet;

    fn check_gift_shop(input: &str, mut func: impl FnMut(i64) -> bool) -> i64 {
        let regex = Regex::new(r"(\d+)-(\d+)").unwrap();
        let ids = input
            .split(",")
            .map(|s| {
                let result = regex.captures(s).unwrap();
                let left = result[1].parse::<i64>().unwrap();
                let right = result[2].parse::<i64>().unwrap();
                (left, right)
            })
            .collect::<Vec<_>>();

        let mut invalid_ids = HashSet::new();
        for id in ids {
            //println!("x = ({}, {})", id.0, id.1);
            for i in id.0..id.1 + 1 {
                if func(i) {
                    invalid_ids.insert(i);
                    //println!("invalid = {}", i);
                }
            }
        }
        invalid_ids.iter().sum()
    }

    use std::path::Path;

    fn invalid_part2(id: i64) -> bool {
        let s: String = id.to_string();
        let new_string = s.clone() + &s;
        new_string[1..(2 * s.len() - 1)].contains(&s)
    }

    fn invalid_part1(id: i64) -> bool {
        let s: String = id.to_string();
        let length = s.len();
        length % 2 == 0 && s[..length / 2] == s[length / 2..]
    }

    #[test]
    fn input_example() {
        let input = r#"11-22,95-115,998-1012,1188511880-1188511890,222220-222224,
1698522-1698528,446443-446449,38593856-38593862,565653-565659,
824824821-824824827,2121212118-2121212124"#;

        let invalid_ids = check_gift_shop(input, |s| invalid_part1(s));

        assert_eq!(invalid_ids, 1227775554);

        let invalid_ids = check_gift_shop(input, |s| invalid_part2(s));

        assert_eq!(invalid_ids, 4174379265);
    }

    #[test]
    fn input_part_1() {
        let path = Path::new("data/day02.txt");

        let input = std::fs::read_to_string(path).unwrap();

        let invalid_ids = check_gift_shop(input.as_str(), |s| invalid_part1(s));

        assert_eq!(invalid_ids, 38437576669);
    }

    #[test]
    fn input_part_2() {
        let path = Path::new("data/day02.txt");

        let input = std::fs::read_to_string(path).unwrap();

        let invalid_ids = check_gift_shop(input.as_str(), |s| invalid_part2(s));

        assert_eq!(invalid_ids, 49046150754);
    }
}

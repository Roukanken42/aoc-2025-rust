use advent_of_code::utils::parse_input_by_lines;
use nom::IResult;
use nom::Parser;
use nom::character::complete::one_of;
use nom::multi::many1;

advent_of_code::solution!(3);

pub fn parse_line(input: &str) -> IResult<&str, Vec<u64>> {
    many1(one_of("0123456789").map(|c| c as u64 - '0' as u64)).parse(input)
}

pub fn parse(input: &str) -> IResult<&str, Vec<Vec<u64>>> {
    (parse_input_by_lines(parse_line)).parse(input)
}

fn highest_joltage(input: &[u64]) -> u64 {
    let (l, r) = input
        .into_iter()
        .enumerate()
        .fold((0, 0), |(l, r), (index, &x)| match () {
            _ if x > l && index != input.len() - 1 => (x, 0),
            _ if x > r => (l, x),
            _ => (l, r),
        });

    l * 10 + r
}

pub fn part_one(input: &str) -> Option<u64> {
    let (_, inputs) = parse(input).unwrap();

    Some(inputs.iter().map(|input| highest_joltage(input)).sum())
}

pub fn part_two(input: &str) -> Option<u64> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let input = advent_of_code::template::read_file("examples", DAY);
        let result = parse(&input);

        assert_eq!(
            result,
            Ok((
                "",
                vec![
                    vec![9, 8, 7, 6, 5, 4, 3, 2, 1, 1, 1, 1, 1, 1, 1],
                    vec![8, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 9],
                    vec![2, 3, 4, 2, 3, 4, 2, 3, 4, 2, 3, 4, 2, 7, 8],
                    vec![8, 1, 8, 1, 8, 1, 9, 1, 1, 1, 1, 2, 1, 1, 1],
                ]
            ))
        );
    }

    #[test]
    fn test_highest_joltage() {
        let input = vec![8, 1, 8, 1, 8, 1, 9, 1, 1, 1, 1, 2, 1, 1, 1];
        assert_eq!(highest_joltage(&input), 92);

        let input = vec![1, 2, 3];
        assert_eq!(highest_joltage(&input), 23);
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(357));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}

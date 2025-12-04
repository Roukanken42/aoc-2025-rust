use advent_of_code::utils::location::Access2d;
use advent_of_code::utils::parse_input_by_lines;
use nom::bytes::complete::take_while1;
use nom::{IResult, Parser};

advent_of_code::solution!(4);

pub fn parse(input: &str) -> IResult<&str, Vec<Vec<char>>> {
    let parse_line = take_while1(|c| c == '.' || c == '@').map(|line: &str| line.chars().collect());
    parse_input_by_lines(parse_line).parse(input)
}

pub fn part_one(input: &str) -> Option<usize> {
    let (_, map) = parse(input).unwrap();

    let result = map
        .iter_2d_keys()
        .filter(|loc| {
            map.get_2d(*loc) == Some(&'@')
                && loc
                    .neighbours()
                    .into_iter()
                    .filter(|&neigh| map.get_2d(neigh) == Some(&'@'))
                    .count()
                    < 4
        })
        .count();

    Some(result)
}

pub fn part_two(input: &str) -> Option<usize> {
    let (_, map) = parse(input).unwrap();

    let mut queue = vec![];
    let mut result = 0usize;
    let mut neighborhoods = vec![vec![None::<i32>; map[0].len()]; map.len()];

    for loc in map.iter_2d_keys() {
        if map.get_2d(loc) != Some(&'@') {
            continue;
        }

        let count = loc
            .neighbours()
            .into_iter()
            .filter(|neigh| map.get_2d(*neigh) == Some(&'@'))
            .count();

        neighborhoods.set_2d(loc, Some(count as i32));
        if count < 4 {
            queue.push(loc)
        }
    }

    while let Some(loc) = queue.pop() {
        if neighborhoods.get_2d(loc).unwrap_or(&None).unwrap_or(999) >= 4 {
            continue;
        }

        let neighbours = loc
            .neighbours()
            .into_iter()
            .filter(|neigh| neighborhoods.get_2d(*neigh).unwrap_or(&None) != &None)
            .collect::<Vec<_>>();

        if neighbours.len() < 4 {
            result += 1;
            neighborhoods.set_2d(loc, None);

            for neigh in neighbours {
                let count = neighborhoods.get_2d(neigh).unwrap_or(&None).unwrap_or(999) - 1;
                neighborhoods.set_2d(neigh, Some(count));
                if count < 4 {
                    queue.push(neigh)
                }
            }
        }
    }

    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(43));
    }
}

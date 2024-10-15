advent_of_code::solution!(8);

pub fn part_one(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .map(|line| {
                line.chars()
                    .skip(1)
                    .collect::<Vec<_>>()
                    .windows(2)
                    .fold((2, 0), |(sum, skip), v| {
                        if skip > 0 {
                            (sum, skip - 1)
                        } else {
                            match v[0] {
                                '\\' => match v[1] {
                                    'x' => (sum + 3, 3),
                                    _ => (sum + 1, 1),
                                },
                                _ => (sum, 0),
                            }
                        }
                    })
                    .0
            })
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .map(|line| {
                line.chars().fold(2, |sum, c| {
                    sum + match c {
                        '"' | '\\' => 1,
                        _ => 0,
                    }
                })
            })
            .sum(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[rstest]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(12));
    }

    #[rstest]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(19));
    }
}

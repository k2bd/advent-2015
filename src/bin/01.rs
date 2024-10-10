advent_of_code::solution!(1);

fn bracket_value(c: char) -> i32 {
    match c {
        '(' => 1,
        ')' => -1,
        _ => 0,
    }
}

pub fn part_one(input: &str) -> Option<i32> {
    Some(input.chars().map(bracket_value).sum())
}

pub fn part_two(input: &str) -> Option<usize> {
    input
        .char_indices()
        .map(|(ind, c)| (ind, bracket_value(c)))
        .scan(0, |sum, (ind, v)| {
            *sum += v;
            Some((ind, *sum))
        })
        .find_map(|(ind, sum)| if sum < 0 { Some(ind + 1) } else { None })
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[rstest]
    #[case("(())", 0)]
    #[case("()()", 0)]
    #[case("(((", 3)]
    #[case("(()(()(", 3)]
    #[case("))(((((", 3)]
    #[case("())", -1)]
    #[case("))(", -1)]
    #[case(")))", -3)]
    #[case(")())())", -3)]
    fn test_part_one(#[case] input: &str, #[case] expected: i32) {
        let result = part_one(input);
        assert_eq!(result, Some(expected));
    }

    #[rstest]
    #[case(")", Some(1))]
    #[case("()())", Some(5))]
    #[case("()()", None)]
    #[case("(((", None)]
    fn test_part_two(#[case] input: &str, #[case] expected: Option<usize>) {
        let result = part_two(input);
        assert_eq!(result, expected);
    }
}

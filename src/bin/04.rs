use ::md5;

advent_of_code::solution!(4);

fn advent_coin_miner(input: &str, prefix: &str) -> Option<u32> {
    (1..).find(|num| {
        let mut val = input.trim().to_string();
        val.push_str(&num.to_string());

        let hash = md5::compute(val);
        format!("{:x}", hash).starts_with(prefix)
    })
}

pub fn part_one(input: &str) -> Option<u32> {
    advent_coin_miner(input, "00000")
}

pub fn part_two(input: &str) -> Option<u32> {
    advent_coin_miner(input, "000000")
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[rstest]
    #[case("abcdef", 609043)]
    #[case("pqrstuv", 1048970)]
    fn test_part_one(#[case] input: &str, #[case] expected: u32) {
        let result = part_one(input);
        assert_eq!(result, Some(expected));
    }
}

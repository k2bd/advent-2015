use ::md5;
use md5::Digest;

advent_of_code::solution!(4);

fn advent_coin_miner(input: &str, condition: fn(hash: Digest) -> bool) -> Option<u32> {
    (1..).find(|num| {
        let mut val = input.trim().to_string();
        val.push_str(&num.to_string());

        let hash = md5::compute(val);
        condition(hash)
    })
}

pub fn part_one(input: &str) -> Option<u32> {
    advent_coin_miner(input, |hash| {
        (hash.0[0] == 0u8)
            && (hash.0[1] == 0u8)
            && (hash.0[2] | 0b0000_1111 == 0b0000_1111)
    })
}

pub fn part_two(input: &str) -> Option<u32> {
    advent_coin_miner(input, |hash| {
        (hash.0[0] == 0u8) && (hash.0[1] == 0u8) && (hash.0[2] == 0u8)
    })
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

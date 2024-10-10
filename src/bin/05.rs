advent_of_code::solution!(5);

fn is_nice_p1(val: &str) -> bool {
    let chars = val.chars().collect::<Vec<_>>();

    let vowel_count = chars.iter().filter(|&c| "aeiou".contains(*c)).count();
    let pairs = chars.windows(2).find(|&v| v[0] == v[1]);
    let banned = chars
        .windows(2)
        .find(|&v| ["ab", "cd", "pq", "xy"].contains(&v.iter().collect::<String>().as_str()));

    (vowel_count >= 3) && (pairs.is_some()) && (banned.is_none())
}

fn is_nice_p2(val: &str) -> bool {
    let chars = val.chars().collect::<Vec<_>>();

    let repeat = (0..chars.len() - 3).find(|&index| {
        let pair = [chars[index], chars[index + 1]];
        let matching = chars[index + 2..].windows(2).find(|&v| *v == pair);
        matching.is_some()
    });
    let triple = chars.windows(3).find(|&v| v[0] == v[2]);

    repeat.is_some() && triple.is_some()
}

pub fn part_one(input: &str) -> Option<u32> {
    Some(
        input
            .split_whitespace()
            .filter(|val| is_nice_p1(val))
            .count() as u32,
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(
        input
            .split_whitespace()
            .filter(|val| is_nice_p2(val))
            .count() as u32,
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[rstest]
    #[case("ugknbfddgicrmopn", true)]
    #[case("aaa", true)]
    #[case("jchzalrnumimnmhp", false)]
    #[case("haegwjzuvuyypxyu", false)]
    #[case("dvszwmarrgswjxmb", false)]
    fn test_is_nice_p1(#[case] input: &str, #[case] expected: bool) {
        let result = is_nice_p1(input);
        assert_eq!(result, expected);
    }

    #[rstest]
    #[case("qjhvhtzxzqqjkmpb", true)]
    #[case("aaa", false)]
    #[case("xxyxx", true)]
    #[case("uurcxstgmygtbstg", false)]
    #[case("ieodomkazucvgmuy", false)]
    fn test_is_nice_p2(#[case] input: &str, #[case] expected: bool) {
        let result = is_nice_p2(input);
        assert_eq!(result, expected);
    }
}

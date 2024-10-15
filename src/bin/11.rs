advent_of_code::solution!(11);

const LOWERCASE_LETTERS: [char; 26] = [
    'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's',
    't', 'u', 'v', 'w', 'x', 'y', 'z',
];

struct PasswordGenerator {
    /// Index encoding of password characters
    state: [usize; 8],
}

impl PasswordGenerator {
    fn from_initial_password(password: &str) -> Self {
        Self {
            state: password
                .chars()
                .map(|c| LOWERCASE_LETTERS.iter().position(|&v| v == c).unwrap())
                .collect::<Vec<_>>()
                .try_into()
                .unwrap(),
        }
    }

    fn increase_condition(&self) -> bool {
        self.state
            .windows(3)
            .any(|win| (win[0] + 1 == win[1]) && (win[1] + 1 == win[2]))
    }

    fn valid_character_condition(&self) -> bool {
        self.state
            .iter()
            .all(|&v| (v != 8) && (v != 11) && (v != 14))
    }

    fn triple_double_condition(&self) -> bool {
        self.state
            .windows(2)
            .fold((0, false), |(count, skip), pair| {
                if skip {
                    (count, false)
                } else if pair[0] == pair[1] {
                    (count + 1, true)
                } else {
                    (count, false)
                }
            })
            .0
            >= 2
    }

    fn valid(&self) -> bool {
        self.increase_condition()
            && self.valid_character_condition()
            && self.triple_double_condition()
    }

    fn increment(&mut self) {
        let mut index: i32 = 7;
        let mut complete = false;

        while !complete {
            if index < 0 {
                self.state = [0; 8];
                return;
            }
            self.state[index as usize] = (self.state[index as usize] + 1) % 26;
            if self.state[index as usize] == 0 {
                index -= 1;
            } else {
                complete = true;
            }
        }
    }

    fn as_string(&self) -> String {
        self.state.iter().fold(String::new(), |mut res, &d| {
            res.push(LOWERCASE_LETTERS[d]);
            res
        })
    }
}

impl Iterator for PasswordGenerator {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        self.increment();
        while !self.valid() {
            self.increment();
        }
        Some(self.as_string())
    }
}

pub fn part_one(input: &str) -> Option<String> {
    PasswordGenerator::from_initial_password(input.trim()).next()
}

pub fn part_two(input: &str) -> Option<String> {
    PasswordGenerator::from_initial_password(input.trim()).nth(1)
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[rstest]
    #[case("hijklmmn", false)]
    #[case("abbceffg", false)]
    #[case("abbcegjk", false)]
    #[case("abcdefgh", false)]
    #[case("abcdffaa", true)]
    #[case("ghijklmn", false)]
    #[case("ghjaabcc", true)]
    fn test_valid_password(#[case] password: &str, #[case] expected_valid: bool) {
        assert_eq!(
            PasswordGenerator::from_initial_password(password).valid(),
            expected_valid
        );
    }

    #[rstest]
    #[case("hijklmmn")]
    #[case("abbceffg")]
    #[case("abbcegjk")]
    #[case("abcdefgh")]
    #[case("abcdffaa")]
    #[case("ghijklmn")]
    #[case("ghjaabcc")]
    fn test_password_roundtrip(#[case] password: &str) {
        assert_eq!(
            PasswordGenerator::from_initial_password(password).as_string(),
            password
        );
    }

    #[rstest]
    #[case("aaaaaaaa", "aaaaaaab")]
    #[case("azzzzzzz", "baaaaaaa")]
    #[case("zzzzzzzz", "aaaaaaaa")]
    fn test_increment(#[case] password: &str, #[case] result: &str) {
        let mut pass = PasswordGenerator::from_initial_password(password);
        pass.increment();

        assert_eq!(pass.as_string(), result);
    }

    #[rstest]
    #[case("abcdefgh", "abcdffaa")]
    #[case("ghijklmn", "ghjaabcc")]
    fn test_next_password(#[case] password: &str, #[case] result: &str) {
        let mut pass = PasswordGenerator::from_initial_password(password);

        assert_eq!(pass.next().unwrap(), result);
    }
}

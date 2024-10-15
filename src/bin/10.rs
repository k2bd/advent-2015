advent_of_code::solution!(10);

struct LookAndSay {
    value: String,
}

impl LookAndSay {
    fn new(value: String) -> Self {
        Self { value }
    }
}

impl Iterator for LookAndSay {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        let n = self
            .value
            .chars()
            .fold(Vec::<(char, usize)>::new(), |mut groups, c| {
                match groups.pop() {
                    Some((g, count)) => {
                        if g == c {
                            groups.push((c, count + 1));
                        } else {
                            groups.push((g, count));
                            groups.push((c, 1));
                        }
                    }
                    None => groups.push((c, 1)),
                };
                groups
            })
            .into_iter()
            .fold(String::new(), |mut result, (c, count)| {
                result.push_str(&count.to_string());
                result.push(c);

                result
            });

        self.value = n.clone();
        Some(n)
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    LookAndSay::new(input.to_string()).nth(39).map(|v| v.len())
}

pub fn part_two(input: &str) -> Option<usize> {
    LookAndSay::new(input.to_string()).nth(49).map(|v| v.len())
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[rstest]
    fn test_look_and_say() {
        let mut look = LookAndSay::new("1".to_string());
        assert_eq!(look.next(), Some("11".to_string()));
        assert_eq!(look.next(), Some("21".to_string()));
        assert_eq!(look.next(), Some("1211".to_string()));
        assert_eq!(look.next(), Some("111221".to_string()));
        assert_eq!(look.next(), Some("312211".to_string()));
    }
}

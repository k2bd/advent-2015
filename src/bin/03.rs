use std::collections::HashSet;
use std::ops::{Add, AddAssign};

advent_of_code::solution!(3);

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
struct Coord {
    x: i32,
    y: i32,
}

impl Coord {
    fn origin() -> Self {
        Self { x: 0, y: 0 }
    }
}

impl From<char> for Coord {
    fn from(value: char) -> Self {
        match value {
            '<' => Self { x: -1, y: 0 },
            '>' => Self { x: 1, y: 0 },
            '^' => Self { x: 0, y: 1 },
            'v' => Self { x: 0, y: -1 },
            _ => Self::origin(),
        }
    }
}

impl Add for Coord {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}
impl AddAssign for Coord {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x + other.x,
            y: self.y + other.y,
        };
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    Some(
        input
            .chars()
            .map(Coord::from)
            .scan(Coord::origin(), |santa_pos, instruction| {
                *santa_pos += instruction;
                Some(*santa_pos)
            })
            .chain([Coord::origin()])
            .collect::<HashSet<_>>()
            .len() as u32,
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(
        input
            .char_indices()
            .map(|(i, c)| (i, Coord::from(c)))
            .scan(
                (Coord::origin(), Coord::origin()),
                |(santa_pos, robo_santa_pos), (index, instruction)| {
                    match index % 2 {
                        0 => *santa_pos += instruction,
                        1 => *robo_santa_pos += instruction,
                        _ => panic!("Impossible remainder"),
                    };

                    Some([*santa_pos, *robo_santa_pos])
                },
            )
            .chain([[Coord::origin(), Coord::origin()]])
            .flatten()
            .collect::<HashSet<_>>()
            .len() as u32,
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[rstest]
    #[case(">", 2)]
    #[case("^>v<", 4)]
    #[case("^v^v^v^v^v", 2)]
    fn test_part_one(#[case] input: &str, #[case] expected: u32) {
        let result = part_one(input);
        assert_eq!(result, Some(expected));
    }

    #[rstest]
    #[case(">", 2)]
    #[case("^v", 3)]
    #[case("^>v<", 3)]
    #[case("^v^v^v^v^v", 11)]
    fn test_part_two(#[case] input: &str, #[case] expected: u32) {
        let result = part_two(input);
        assert_eq!(result, Some(expected));
    }
}

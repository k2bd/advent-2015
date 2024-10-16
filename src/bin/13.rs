use std::collections::{HashMap, HashSet};

use itertools::Itertools;

advent_of_code::solution!(13);

struct HappinessMatrix {
    neighbor_happiness: HashMap<(String, String), i32>,
}

impl HappinessMatrix {
    fn from_input(input: &str) -> Self {
        Self {
            neighbor_happiness: input.lines().fold(HashMap::new(), |mut map, line| {
                let (first_part, next_to_part) = line
                    .split_once(" happiness units by sitting next to ")
                    .unwrap();

                let (who, change_part) = first_part.split_once(" would ").unwrap();

                let (change, by_part) = change_part.split_once(" ").unwrap();

                let mut by = by_part.parse::<i32>().unwrap();
                if change == "lose" {
                    by *= -1;
                }

                let next_to = next_to_part.strip_suffix(".").unwrap();

                map.insert((who.to_string(), next_to.to_string()), by);

                map
            }),
        }
    }

    fn add_myself(&mut self) {
        self.all_guests().into_iter().for_each(|guest| {
            self.neighbor_happiness
                .insert(("~~~ME~~~".to_string(), guest.clone()), 0);
            self.neighbor_happiness
                .insert((guest.clone(), "~~~ME~~~".to_string()), 0);
        });
    }

    fn all_guests(&self) -> HashSet<String> {
        self.neighbor_happiness
            .keys()
            .fold(HashSet::new(), |mut set, (guest, _)| {
                set.insert(guest.clone());
                set
            })
    }

    fn evaluate_total_happiness(&self, arrangement: Vec<String>) -> i32 {
        let len = arrangement.len();

        (0..len)
            .map(|index| {
                let me = arrangement[index].clone();
                let left_neighbor =
                    arrangement[(index as i32 - 1).rem_euclid(len as i32) as usize].clone();
                let right_neighbor = arrangement[(index + 1).rem_euclid(len)].clone();

                self.neighbor_happiness[&(me.clone(), left_neighbor)]
                    + self.neighbor_happiness[&(me.clone(), right_neighbor)]
            })
            .sum()
    }
}

pub fn part_one(input: &str) -> Option<i32> {
    let happiness_matrix = HappinessMatrix::from_input(input);
    happiness_matrix
        .all_guests()
        .into_iter()
        .permutations(happiness_matrix.all_guests().len())
        .map(|p| happiness_matrix.evaluate_total_happiness(p))
        .max()
}

pub fn part_two(input: &str) -> Option<i32> {
    let mut happiness_matrix = HappinessMatrix::from_input(input);
    happiness_matrix.add_myself();
    happiness_matrix
        .all_guests()
        .into_iter()
        .permutations(happiness_matrix.all_guests().len())
        .map(|p| happiness_matrix.evaluate_total_happiness(p))
        .max()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(330));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert!(result.is_some());
    }
}

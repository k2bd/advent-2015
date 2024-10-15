use itertools::Itertools;
use std::collections::{HashMap, HashSet};

advent_of_code::solution!(9);

struct Destinations {
    distances: HashMap<(String, String), u32>,
}

impl Destinations {
    fn new() -> Self {
        Self {
            distances: HashMap::new(),
        }
    }

    fn from_input(input: &str) -> Self {
        let mut result = Self::new();
        input.lines().for_each(|line| {
            let (instruction_part, distance_part) = line.split_once(" = ").unwrap();
            let (source, dest) = instruction_part.split_once(" to ").unwrap();

            result.add_distance(
                source.to_string(),
                dest.to_string(),
                distance_part.parse().unwrap(),
            );
        });

        result
    }

    fn add_distance(&mut self, loc_1: String, loc_2: String, distance: u32) {
        self.distances
            .insert((loc_1.clone(), loc_2.clone()), distance);
        self.distances
            .insert((loc_2.clone(), loc_1.clone()), distance);
    }

    fn all_destinations(&self) -> HashSet<String> {
        let mut result = HashSet::new();
        self.distances.keys().for_each(|(loc, _)| {
            result.insert(loc.clone());
        });
        result
    }

    fn permutation_distances(&self) -> impl Iterator<Item = u32> + '_ {
        let dests = self.all_destinations();
        let len = dests.len();
        dests.into_iter().permutations(len).map(|perm| {
            perm.windows(2)
                .map(|loc| {
                    self.distances
                        .get(&(loc[0].clone(), loc[1].clone()))
                        .unwrap()
                })
                .sum()
        })
    }

    fn salesman_distance(&self) -> Option<u32> {
        self.permutation_distances().min()
    }

    fn terrible_salesman_distance(&self) -> Option<u32> {
        self.permutation_distances().max()
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    Destinations::from_input(input).salesman_distance()
}

pub fn part_two(input: &str) -> Option<u32> {
    Destinations::from_input(input).terrible_salesman_distance()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(605));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(982));
    }
}

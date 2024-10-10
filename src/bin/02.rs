advent_of_code::solution!(2);

#[derive(Debug, PartialEq)]
struct BoxDims {
    w: u32,
    h: u32,
    l: u32,
}

impl BoxDims {
    fn from_input(dims: &str) -> Self {
        let mut parsed = dims.split("x").map(|v| str::parse(v).unwrap());
        Self {
            w: parsed.next().unwrap(),
            h: parsed.next().unwrap(),
            l: parsed.next().unwrap(),
        }
    }

    fn required_wrapping(&self) -> u32 {
        let sides = [self.w * self.h, self.h * self.l, self.l * self.w];
        2 * sides.iter().sum::<u32>() + sides.iter().min().unwrap()
    }

    fn required_ribbon(&self) -> u32 {
        let mut sides = [self.w, self.h, self.l];
        sides.sort();
        let mut sorted_sides = sides.iter();
        let side_a = sorted_sides.next().unwrap();
        let side_b = sorted_sides.next().unwrap();

        2 * (side_a + side_b) + sides.iter().product::<u32>()
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    Some(
        input
            .split_whitespace()
            .map(|i| BoxDims::from_input(i).required_wrapping())
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(
        input
            .split_whitespace()
            .map(|i| BoxDims::from_input(i).required_ribbon())
            .sum(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[rstest]
    #[case("2x3x4", BoxDims { w: 2, h: 3, l: 4})]
    #[case("1x1x10", BoxDims { w: 1, h: 1, l: 10})]
    fn test_box_dims(#[case] input: &str, #[case] expected: BoxDims) {
        assert_eq!(BoxDims::from_input(input), expected);
    }

    #[rstest]
    #[case("2x3x4", 58)]
    #[case("1x1x10", 43)]
    fn test_required_wrapping(#[case] input: &str, #[case] expected: u32) {
        assert_eq!(BoxDims::from_input(input).required_wrapping(), expected);
    }
    #[rstest]
    #[case("2x3x4", 34)]
    #[case("1x1x10", 14)]
    fn test_required_ribbon(#[case] input: &str, #[case] expected: u32) {
        assert_eq!(BoxDims::from_input(input).required_ribbon(), expected);
    }
}

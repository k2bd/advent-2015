advent_of_code::solution!(7);

use std::collections::HashMap;

#[derive(Debug)]
enum ComponentInput {
    Signal(u16),
    Component(String),
}

#[derive(Debug)]
enum CircuitComponent {
    Direct(ComponentInput),
    AndGate(ComponentInput, ComponentInput),
    OrGate(ComponentInput, ComponentInput),
    LeftShifter(ComponentInput, usize),
    RightShifter(ComponentInput, usize),
    Not(ComponentInput),
}

type ResCache = HashMap<String, Option<u16>>;

fn get_component_input_value(
    value: &ComponentInput,
    board: &CircuitBoard,
    resolution_cache: &mut ResCache,
) -> Option<u16> {
    match value {
        ComponentInput::Signal(value) => Some(*value),
        ComponentInput::Component(key) => board.get_signal(key, resolution_cache),
    }
}

impl CircuitComponent {
    fn resolve(&self, board: &CircuitBoard, resolution_cache: &mut ResCache) -> u16 {
        match self {
            CircuitComponent::Direct(value) => {
                get_component_input_value(value, board, resolution_cache).unwrap_or_default()
            }
            CircuitComponent::AndGate(input_l, input_r) => {
                match (
                    get_component_input_value(input_l, board, resolution_cache),
                    get_component_input_value(input_r, board, resolution_cache),
                ) {
                    (Some(l), Some(r)) => l & r,
                    _ => 0,
                }
            }
            CircuitComponent::OrGate(input_l, input_r) => {
                match (
                    get_component_input_value(input_l, board, resolution_cache),
                    get_component_input_value(input_r, board, resolution_cache),
                ) {
                    (Some(l), Some(r)) => l | r,
                    _ => 0,
                }
            }
            CircuitComponent::LeftShifter(input, by) => {
                match get_component_input_value(input, board, resolution_cache) {
                    Some(i) => i << by,
                    _ => 0,
                }
            }
            CircuitComponent::RightShifter(input, by) => {
                match get_component_input_value(input, board, resolution_cache) {
                    Some(i) => i >> by,
                    _ => 0,
                }
            }
            CircuitComponent::Not(input) => {
                match get_component_input_value(input, board, resolution_cache) {
                    Some(i) => !i,
                    _ => 0,
                }
            }
        }
    }
}

struct CircuitBoard {
    components: HashMap<String, CircuitComponent>,
}

impl CircuitBoard {
    fn get_signal(&self, wire_id: &String, resolution_cache: &mut ResCache) -> Option<u16> {
        match resolution_cache.get(wire_id).cloned() {
            Some(val) => val,
            None => {
                let res = self
                    .components
                    .get(wire_id)
                    .map(|c| c.resolve(self, resolution_cache));
                resolution_cache.insert(wire_id.to_string(), res);
                res
            }
        }
    }

    fn from_instructions(instructions: &str) -> Self {
        Self {
            components: HashMap::from_iter(instructions.lines().map(parse_instruction)),
        }
    }
}

fn parse_source(soure_str: &str) -> ComponentInput {
    match soure_str.parse::<u16>() {
        Ok(value) => ComponentInput::Signal(value),
        Err(_) => ComponentInput::Component(soure_str.to_string()),
    }
}

fn parse_component(component_str: &str) -> CircuitComponent {
    if component_str.contains(" AND ") {
        let (input_l, input_r) = component_str.split_once(" AND ").unwrap();
        CircuitComponent::AndGate(parse_source(input_l), parse_source(input_r))
    } else if component_str.contains(" OR ") {
        let (input_l, input_r) = component_str.split_once(" OR ").unwrap();
        CircuitComponent::OrGate(parse_source(input_l), parse_source(input_r))
    } else if component_str.contains(" LSHIFT ") {
        let (input, by_str) = component_str.split_once(" LSHIFT ").unwrap();
        let by = by_str.parse::<usize>().unwrap();
        CircuitComponent::LeftShifter(parse_source(input), by)
    } else if component_str.contains(" RSHIFT ") {
        let (input, by_str) = component_str.split_once(" RSHIFT ").unwrap();
        let by = by_str.parse::<usize>().unwrap();
        CircuitComponent::RightShifter(parse_source(input), by)
    } else if component_str.contains("NOT ") {
        let input = component_str.strip_prefix("NOT ").unwrap();
        CircuitComponent::Not(parse_source(input))
    } else {
        CircuitComponent::Direct(parse_source(component_str))
    }
}

fn parse_instruction(instruction: &str) -> (String, CircuitComponent) {
    let (instruction_part, target) = instruction.split_once(" -> ").unwrap();

    (target.to_string(), parse_component(instruction_part))
}

pub fn part_one(input: &str) -> Option<u16> {
    let mut cache = HashMap::<String, Option<u16>>::new();
    CircuitBoard::from_instructions(input).get_signal(&"a".to_string(), &mut cache)
}

pub fn part_two(input: &str) -> Option<u16> {
    let mut board = CircuitBoard::from_instructions(input);
    let mut cache = HashMap::<String, Option<u16>>::new();

    let original_a = board.get_signal(&"a".to_string(), &mut cache).unwrap();
    board.components.insert(
        "b".to_string(),
        CircuitComponent::Direct(ComponentInput::Signal(original_a)),
    );

    cache.clear();
    board.get_signal(&"a".to_string(), &mut cache)
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[rstest]
    #[case("d", Some(72))]
    #[case("e", Some(507))]
    #[case("f", Some(492))]
    #[case("g", Some(114))]
    #[case("h", Some(65412))]
    #[case("i", Some(65079))]
    #[case("x", Some(123))]
    #[case("y", Some(456))]
    fn test_from_instructions(#[case] wire_id: String, #[case] expected_signal: Option<u16>) {
        let instructions = "123 -> x\n456 -> y\nx AND y -> d\nx OR y -> e\nx LSHIFT 2 -> f\ny RSHIFT 2 -> g\nNOT x -> h\nNOT y -> i";
        let board = CircuitBoard::from_instructions(instructions);

        let mut cache = HashMap::<String, Option<u16>>::new();

        assert_eq!(board.get_signal(&wire_id, &mut cache), expected_signal);
    }
}

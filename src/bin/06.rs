advent_of_code::solution!(6);

struct Lights {
    lights: [[u32; 1000]; 1000],
}

impl Lights {
    fn new() -> Self {
        Self {
            lights: [[0; 1000]; 1000],
        }
    }

    fn apply_instruction_p1(&mut self, instruction: Instruction) {
        (instruction.from.0..=instruction.to.0).for_each(|i| {
            (instruction.from.1..=instruction.to.1).for_each(|j| match instruction.kind {
                InstructionKind::TurnOff => self.lights[i][j] = 0,
                InstructionKind::TurnOn => self.lights[i][j] = 1,
                InstructionKind::Toggle => {
                    self.lights[i][j] = if self.lights[i][j] == 0 { 1 } else { 0 }
                }
            });
        });
    }

    fn apply_instruction_p2(&mut self, instruction: Instruction) {
        (instruction.from.0..=instruction.to.0).for_each(|i| {
            (instruction.from.1..=instruction.to.1).for_each(|j| match instruction.kind {
                InstructionKind::TurnOff => {
                    self.lights[i][j] = 0.max((self.lights[i][j] as i64) - 1) as u32
                }
                InstructionKind::TurnOn => self.lights[i][j] += 1,
                InstructionKind::Toggle => self.lights[i][j] += 2,
            });
        });
    }

    fn count_on(&self) -> usize {
        self.lights
            .as_flattened()
            .iter()
            .filter(|&&l| l > 0)
            .count()
    }

    fn total_brightness(&self) -> u32 {
        self.lights.as_flattened().iter().sum()
    }
}

enum InstructionKind {
    TurnOn,
    TurnOff,
    Toggle,
}

struct Instruction {
    kind: InstructionKind,
    from: (usize, usize),
    to: (usize, usize),
}

impl<'a> From<&'a str> for Instruction {
    fn from(value: &'a str) -> Self {
        let kind: InstructionKind;

        let range_part: &'a str;

        if value.starts_with("turn on") {
            kind = InstructionKind::TurnOn;
            range_part = value.strip_prefix("turn on ").unwrap();
        } else if value.starts_with("turn off") {
            kind = InstructionKind::TurnOff;
            range_part = value.strip_prefix("turn off ").unwrap();
        } else {
            kind = InstructionKind::Toggle;
            range_part = value.strip_prefix("toggle ").unwrap();
        }

        let mut i = range_part.split(" through ");
        let from_part = i.next().unwrap().split_once(",").unwrap();
        let from: (usize, usize) = (
            str::parse(from_part.0).unwrap(),
            str::parse(from_part.1).unwrap(),
        );
        let to_part = i.next().unwrap().split_once(",").unwrap();
        let to: (usize, usize) = (
            str::parse(to_part.0).unwrap(),
            str::parse(to_part.1).unwrap(),
        );

        Self { kind, from, to }
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let mut lights = Lights::new();
    input.lines().for_each(|line| {
        lights.apply_instruction_p1(Instruction::from(line));
    });
    Some(lights.count_on())
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut lights = Lights::new();
    input.lines().for_each(|line| {
        lights.apply_instruction_p2(Instruction::from(line));
    });
    Some(lights.total_brightness())
}

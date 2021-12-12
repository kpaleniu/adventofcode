enum InstructionSet {
    Forward(i64),
    Up(i64),
    Down(i64),
}

fn parse(input: &str) -> Vec<InstructionSet> {
    let mut instructions = vec![];
    for line in input.lines() {
        let mut s = line.split_whitespace();
        let instruction = match s.next() {
            Some("forward") => InstructionSet::Forward(s.next().unwrap().parse().unwrap()),
            Some("up") => InstructionSet::Up(s.next().unwrap().parse().unwrap()),
            Some("down") => InstructionSet::Down(s.next().unwrap().parse().unwrap()),
            _ => panic!("unknown instruction"),
        };
        instructions.push(instruction);
    }
    instructions
}

pub fn compute(input: &str) -> (i64, i64) {
    let (mut position, mut depth) = (0, 0);
    for instruction in parse(input) {
        match instruction {
            InstructionSet::Forward(val) => position += val,
            InstructionSet::Up(val) => depth -= val,
            InstructionSet::Down(val) => depth += val,
        }
    }
    (position, depth)
}

pub fn part2(input: &str) -> (i64, i64, i64) {
    let (mut position, mut depth, mut aim) = (0, 0, 0);
    for instruction in parse(input) {
        match instruction {
            InstructionSet::Forward(val) => {
                position += val;
                depth += aim * val
            }
            InstructionSet::Up(val) => aim -= val,
            InstructionSet::Down(val) => aim += val,
        }
    }
    (position, depth, aim)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_example() {
        let input = "forward 5
down 5
forward 8
up 3
down 8
forward 2";
        let (position, depth) = compute(input);
        assert_eq!(position, 15);
        assert_eq!(depth, 10);
    }

    #[test]
    fn test_part2() {
        let input = "forward 5
down 5
forward 8
up 3
down 8
forward 2";
        let (position, depth, _) = part2(input);
        assert_eq!(position, 15);
        assert_eq!(depth, 60);
    }
}

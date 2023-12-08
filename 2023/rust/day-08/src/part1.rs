use std::collections::BTreeMap;
use crate::custom_error::AocError;

#[tracing::instrument]
pub fn process(
    _input: &str,
) -> miette::Result<u64, AocError> {
    let mut lines = _input.lines();
    let instructions = lines.next().expect("Should have line of instructions");
    lines.next();
    let nodes = lines.fold(BTreeMap::new(), |mut map, line| {
        let name = &line[0..=2];
        let left = &line[7..=9];
        let right = &line[12..=14];
        map.insert(name, Node { left, right });
        map
    });
    let steps = count_steps(instructions, &nodes);
    Ok(steps)
}

fn count_steps(instructions: &str, nodes: &BTreeMap<&str, Node>) -> u64 {
    let mut current_node = "AAA";
    let mut steps = 0;
    loop {
        for d in instructions.chars() {
            if current_node == "ZZZ" { return steps; }
            steps += 1;
            let node = nodes.get(current_node).expect("Should exist");
            match d {
                'L' => current_node = node.left,
                'R' => current_node = node.right,
                _ => { panic!("Unknown instruction") }
            }
        };
    }
}

struct Node<'a> {
    left: &'a str,
    right: &'a str,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)";
        assert_eq!(2, process(input)?);
        Ok(())
    }

    #[test]
    fn test_process_repeat() -> miette::Result<()> {
        let input = "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";
        assert_eq!(6, process(input)?);
        Ok(())
    }
}

use std::collections::BTreeMap;
use crate::custom_error::AocError;
use rayon::prelude::*;

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

    let start_nodes: Vec<_> = nodes.keys().filter(|k| k.chars().last().unwrap() == 'A').copied().collect();

    let steps = start_nodes.into_par_iter().map(|node|count_steps(node, instructions, &nodes)).collect::<Vec<_>>().into_iter().fold(1,|lcm, steps|{
        num_integer::lcm(lcm,steps)
    });

    Ok(steps)
}

fn count_steps(start: &str, instructions: &str, nodes: &BTreeMap<&str, Node>) -> u64 {
    let mut current_node = start;
    let mut steps = 0;
    loop {
        for d in instructions.chars() {
            if current_node.chars().last().unwrap() == 'Z' { return steps; }
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
        let input = "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)";
        assert_eq!(6, process(input)?);
        Ok(())
    }
}

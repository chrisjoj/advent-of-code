use crate::custom_error::AocError;

#[tracing::instrument]
pub fn process(
    _input: &str,
) -> miette::Result<u64, AocError> {
    const DEFAULT: Pipe = Pipe::Ground;
    let mut grid: [[(Pipe, i64); 5]; 5] = [[(DEFAULT, -1); 5]; 5];
    let mut grid_row: usize = 0;
    let mut start_loc: (usize, usize) = (0, 0);
    _input.lines().for_each(|line| {
        let mut index: usize = 0;
        line.chars().for_each(|char| {
            let pipe = match char {
                '.' => Pipe::Ground,
                '|' => Pipe::Vertical,
                '-' => Pipe::Horizontal,
                'L' => Pipe::NEBend,
                'J' => Pipe::NWBend,
                '7' => Pipe::SWBend,
                'F' => Pipe::SEBend,
                'S' => Pipe::Start,
                _ => panic!("{}", format!("Unknown input {}", char))
            };
            grid[grid_row][index] = (pipe, -1);
            if pipe == Pipe::Start {
                start_loc = (grid_row, index);
            }
            index += 1;
        });
        grid_row += 1;
    });

    let start = find_first_nodes(start_loc, grid);
    let result = next_node(start, grid);

    Ok(result)
}

fn next_node(start: ((Pipe, i64), (Pipe, i64)), grid: [[(Pipe, i64); 5]; 5]) -> u64 {
    let mut a = grid
    [start.0.]
    [];
    let mut b = grid
    []
    [];
}

fn find_first_nodes(start: (usize, usize), grid: [[(Pipe, i64); 5]; 5]) -> (usize,usize) {
    let mut n = vec!();

    //north
    if grid[start.0 - 1][start.1].0 == Pipe::Vertical || grid[start.0 - 1][start.1].0 == Pipe::SWBend || grid[start.0 - 1][start.1].0 == Pipe::SEBend {
        n.push((start.0-1,start.1));
    }
    //south
    if grid[start.0 + 1][start.1].0 == Pipe::Vertical || grid[start.0 + 1][start.1].0 == Pipe::SWBend || grid[start.0 + 1][start.1].0 == Pipe::SEBend {
        n.push((start.0+1,start.1));
    }
    //east
    if grid[start.0][start.1 + 1].0 == Pipe::Horizontal || grid[start.0][start.1 + 1].0 == Pipe::SEBend || grid[start.0][start.1 + 1].0 == Pipe::NEBend {
        n.push((start.0,start.1 + 1));
    }
    //west
    if grid[start.0][start.1 - 1].0 == Pipe::Horizontal || grid[start.0][start.1 - 1].0 == Pipe::SWBend || grid[start.0][start.1 - 1].0 == Pipe::NWBend {
        n.push((start.0,start.1 - 1));
    }
    (*n.get(0).unwrap(), *n.get(1).unwrap())
}

#[derive(Debug, PartialEq, Copy, Clone)]
enum Pipe {
    Vertical,
    Horizontal,
    NEBend,
    NWBend,
    SWBend,
    SEBend,
    Ground,
    Start,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "-L|F7
7S-7|
L|7||
-L-J|
L|-JF";
        assert_eq!(4, process(input)?);
        Ok(())
    }

    #[test]
    fn test_process_2() -> miette::Result<()> {
        let input = "7-F7-
.FJ|7
SJLL7
|F--J
LJ.LJ";
        assert_eq!(8, process(input)?);
        Ok(())
    }
}

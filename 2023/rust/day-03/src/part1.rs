use std::collections::BTreeMap;
use itertools::Itertools;
use crate::custom_error::AocError;

#[derive(Debug, Eq, PartialEq)]
struct Number {
    line: i32,
    start: i32,
    number: i32,
}

#[tracing::instrument]
pub fn process(
    _input: &str,
) -> miette::Result<i32, AocError> {
    const RADIX: i32 = 10;
    const SYMBOLS: [char; 11] = ['#', '$', '*', '/', '=', '-', '+', '%', '@', '&', '.'];
    let mut symbols: BTreeMap<(i32,i32), i32> = BTreeMap::new();
    let mut numbers: Vec<Number> = vec![];
    let mut line_index = 0;

    let mut digits: Vec<char> = vec!();
    _input.lines().for_each(|line| {
        digits.clear();
        let mut index = 0;
        let chars = line.chars().collect_vec();

        while index < line.len()  {
            let char = chars[index];
            match char {
                '.' => {}
                '#' | '$' | '*' | '/' | '=' | '-' | '+' | '%' | '@' | '&' => {
                    symbols.insert((line_index, index as i32), index as i32);
                }
                _ => {
                    digits.push(char);
                    if index == line.len() - 1 || SYMBOLS.contains(&chars[index + 1]) {
                        // end of number so store it in numbers
                        // println!("Adding number: {:?}", digits);
                        let number = digits
                            .iter()
                            .map(|c| c.to_digit(RADIX as u32))
                            .try_fold(0, |ans, i| i.map(|i| ans * RADIX + (i as i32))).unwrap();
                        numbers.push(Number { line: line_index, start: (index + 1 - digits.len()) as i32, number });
                        digits.clear();
                    }
                }
            }
            index += 1;
        }
        line_index += 1;
    });

    // println!("Symbols: {:?}", symbols);
    // println!("Numbers: {:?}", numbers);

    let result: i32 = numbers.iter().map(|number| {
        let line_no = number.line;
        let length = number.number.to_string().len() as i32;
        let start = number.start;

        let found = (line_no - 1..line_no + 2).any(|line| {
            (start - 1..start + 1+length).any(|index| {
                match index {
                    i if i < 0 || line < 0 => { false }
                    _ => {
                        // println!("Trying to find: {} {} {:?}", line, index, number);
                        symbols.get(&((line), (index))) == Some(&(index))
                    }
                }
            })
        });
        match found {
            true => {
                // println!("Found symbol for number: {:?}", number.number);
                number.number
            }
            false => {
                // println!("No symbol found for number: {:?}", number.number);
                0
                }
        }
    }).sum();

                Ok(result)
            }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";
        assert_eq!(4361, process(input)?);
        Ok(())
    }
}

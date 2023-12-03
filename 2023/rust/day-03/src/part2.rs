use std::collections::BTreeMap;
use itertools::Itertools;
use crate::custom_error::AocError;

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
struct Number {
    line: i32,
    start: i32,
    number: i32,
}

#[tracing::instrument]
pub fn process(
    _input: &str,
) -> miette::Result<i32, AocError> {
    const SYMBOLS: [char; 11] = ['#', '$', '*', '/', '=', '-', '+', '%', '@', '&', '.'];
    let mut symbols: BTreeMap<(i32, i32), i32> = BTreeMap::new();
    let mut numbers: BTreeMap<i32, Vec<Number>> = BTreeMap::new();
    let mut line_index = 0;
    let mut digits: Vec<char> = vec!();

    _input.lines().for_each(|line| {
        let mut index: usize = 0;
        let chars = line.chars().collect_vec();
        while index < line.len() {
            let char = chars[index];
            match char {
                '#' | '$' | '/' | '=' | '-' | '+' | '%' | '@' | '&' | '.' => {}
                '*' => {
                    symbols.insert((line_index, index as i32), index as i32);
                }
                _ => {
                    digits.push(char);
                    if index == line.len() - 1 || SYMBOLS.contains(&chars[index + 1]) {
                        let number = String::from_iter(&digits).parse::<i32>().unwrap();
                        let number = Number { line: line_index, start: (index + 1 - digits.len()) as i32, number };
                        match numbers.get_mut(&line_index) {
                            None => { numbers.insert(line_index, vec![number]); }
                            Some(v) => { v.push(number); }
                        }
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
    let mut count = 0;
    let result: i32 = symbols.keys().map(|symbol| {
        let line = symbol.0;
        let valid_lines = numbers.get(&line).unwrap();
        let valid_lines1 = numbers.get(&(&line + 1)).unwrap();
        let valid_lines2 = numbers.get(&(&line - 1)).unwrap();
        count += 1;
        find_gear_ratios(*symbol, &valid_lines.iter().copied().chain(valid_lines1.iter().copied().chain(valid_lines2.iter().copied())).collect_vec())
    }
    ).sum();
    Ok(result)
}

fn find_gear_ratios(symbol: (i32, i32), numbers: &[Number]) -> i32 {
    let line_no = symbol.0;
    let start = symbol.1;

    let matched_numbers = numbers.iter().filter_map(|number| {
        let number_length = number.number.to_string().len() as i32;
        let lower_line = std::cmp::max(number.line - 1, 0);
        let upper_line = number.line + 1;
        let number_start = std::cmp::max(number.start - 1, 0);
        let number_end = number.start + number_length;

        if (lower_line..upper_line + 1).contains(&line_no) && (number_start..number_end + 1).contains(&start) {
            // println!("{} {} = {}, {} {} = {}", lower_line, upper_line, line_no, number_start, number_end, start);
            // println!("Found number: {} with symbol: {:?}", number.number, symbol);
            Some(number.number)
        } else {
            // println!("{} {} = {}, {} {} = {}", lower_line, upper_line, line_no, number_start, number_end, start);
            // println!("Skipped number: {} with symbol: {:?}", number.number, symbol);
            None
        }
    }).collect_vec();

    match matched_numbers.len() {
        2 => {
            // println!("Matched: {:?}", matched_numbers);
            matched_numbers.iter().product()
        }
        _ => 0
    }
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
        assert_eq!(467835, process(input)?);
        Ok(())
    }
}

use crate::custom_error::AocError;

#[tracing::instrument]
pub fn process(
    _input: &str,
) -> miette::Result<i64, AocError> {
    let result = _input.lines().map(|line|{
        let sequence = line.split_ascii_whitespace().map(|element|{
            element.parse::<i64>().expect("Should be a number")
        }).collect::<Vec<_>>();
        
        next_value(sequence)
    }).sum::<i64>();
    Ok(result)
}

fn next_value(seq: Vec<i64>) -> i64 {
    let mut diffs = vec!();
    for i in 1..seq.len() {
        let diff = seq.get(i).expect("Should have a value") - seq.get(i-1).expect("Should have a lower index to subtract");
        diffs.push(diff);
    }
    if diffs.iter().all(|x| *x== 0i64) {
        *seq.first().expect("Should have a last value")
    } else {
        *seq.first().unwrap() - next_value(diffs)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";
        assert_eq!(2, process(input)?);
        Ok(())
    }
}

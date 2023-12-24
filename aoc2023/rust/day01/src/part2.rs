use crate::part1::extract_trebuchet_calibration;


/// Process the problem for this day and stage.
pub fn process(input: &str) -> u32 {
    extract_trebuchet_calibration(input).sum()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        let input = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen

";
        let expected: u32 = 281;
        assert_eq!(process(input), expected);
    }
}

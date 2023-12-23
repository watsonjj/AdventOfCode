fn main() {
    let input = include_str!("../../input.txt");
    let result = process(input);
    println!("{}", result);
}


fn extract_calibration_value(calibration_line: &str) -> u32 {
    // Extract the digits from the line of text
    let mut s = String::from("");
    for c in calibration_line.chars() {
        match c.to_digit(10) {
            Some(_) => { s.push(c); }
            None => {}
        }
    }

    if s.len() == 0 { return 0; }

    // Combine the first and last digit
    let calibration_str = format!(
        "{}{}",
        s.chars().next().expect(&format!("First value failed for value: {}", s)),
        s.chars().last().expect(&format!("Last value failed for value: {}", s)),
    );

    // Convert into integer
    calibration_str.parse::<u32>().expect("Parsing into u32 failed")
}

fn process(input: &str) -> u32 {
    input.split('\n').map(|l| extract_calibration_value(l)).sum()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        let input = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";
        let expected: u32 = 142;
        assert_eq!(process(input), expected);
    }

        #[test]
    fn test_process_empty_line() {
        let input = "1abc2
pqr3stu8vwx

a1b2c3d4e5f
treb7uchet";
        let expected: u32 = 142;
        assert_eq!(process(input), expected);
    }
}


use regex::Regex;


/// Extract a digit from the text following the regex pattern.
/// Returns "0" if not match found.
fn extract_digit_str(re: Regex, text: &str) -> &str {
    match re.captures(text) {
        Some(caps) => { caps.get(1).unwrap().as_str() }
        None => { "0" }
    }
}

/// Extract the trebuchet calibration value for a single line
/// in the calibration document.
fn extract_trebuchet_calibration_value(calibration_line: &str) -> u32 {
    // Define patterns
    let re = r"\d";
    let re_0 = Regex::new(&format!(r"({re})")).unwrap();
    let re_1 = Regex::new(&format!(r".*({re}).*?$")).unwrap();

    // Extract matches
    let digit_0 = extract_digit_str(re_0, calibration_line);
    let digit_1 = extract_digit_str(re_1, calibration_line);

    // Combine the first and last digit
    let calibration_str = format!("{}{}", digit_0, digit_1);

    // Convert into integer
    let calibration_value = calibration_str
        .parse::<u32>()
        .expect("Parsing into u32 failed");
    return calibration_value;
}

/// Prepare an Iterator containing the trebuchet calibration values extracted
/// from the document.
fn extract_trebuchet_calibration<'a>(
    calibration_document: &'a str
) -> impl Iterator<Item = u32> + 'a {
    calibration_document
        .split_terminator('\n')
        .map(|l| extract_trebuchet_calibration_value(l))
}

/// Process the problem for this day and stage.
fn process(input: &str) -> u32 {
    extract_trebuchet_calibration(input).sum()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        let input = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet

";
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

fn main() {
    let input = include_str!("../../input.txt");
    let result = process(input);
    println!("{}", result);
}
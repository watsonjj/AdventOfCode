use regex::Regex;


/// Extract a digit from the text following the regex pattern.
/// Returns "0" if not match found.
fn extract_digit_str(re: Regex, text: &str) -> &str {
    match re.captures(text) {
        Some(caps) => caps.get(1).unwrap().as_str(),
        None => "0"
    }
}

/// Convert a digit from its string form into an integer.
fn convert_digit_str_to_int(digit_str: &str) -> u32{
    match digit_str {
        "1" | "one" => 1,
        "2" | "two" => 2,
        "3" | "three" => 3,
        "4" | "four" => 4,
        "5" | "five" => 5,
        "6" | "six" => 6,
        "7" | "seven" => 7,
        "8" | "eight" => 8,
        "9" | "nine" => 9,
        _ => {
            println!("Unexpected digit: {}", digit_str);
            0
        }
    }
}


/// Extract the trebuchet calibration value for a single line
/// in the calibration document.
fn extract_trebuchet_calibration_value(calibration_line: &str) -> u32 {
    // Define patterns
    let re = r"\d|one|two|three|four|five|six|seven|eight|nine";
    let re_0 = Regex::new(&format!(r"({re})")).unwrap();
    let re_1 = Regex::new(&format!(r".*({re}).*?$")).unwrap();

    // Extract matches
    let digit_str_0 = extract_digit_str(re_0, calibration_line);
    let digit_str_1 = extract_digit_str(re_1, calibration_line);

    // Convert into integers
    let digit_value_0 = convert_digit_str_to_int(digit_str_0);
    let digit_value_1 = convert_digit_str_to_int(digit_str_1);
    return digit_value_0 * 10 + digit_value_1;
}

/// Prepare an Iterator containing the trebuchet calibration values extracted
/// from the document.
pub fn extract_trebuchet_calibration<'a>(
    calibration_document: &'a str
) -> impl Iterator<Item = u32> + 'a {
    calibration_document
        .split_terminator('\n')
        .map(|l| extract_trebuchet_calibration_value(l))
}

/// Process the problem for this day and stage.
pub fn process(input: &str) -> u32 {
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

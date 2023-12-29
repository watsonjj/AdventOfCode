use std::collections::HashSet;
use once_cell::sync::Lazy;
use regex::Regex;

#[derive(Debug)]
pub struct Scratchcard {
    winning_numbers: HashSet<u32>,
    scratch_numbers: HashSet<u32>,
}

impl Scratchcard {
    pub fn from_line(line: &str) -> Scratchcard {
        let (l0, l1) = line.split_once(':').unwrap();
        let (l2, l3) = l1.split_once('|').unwrap();

        // Extract the game ID
        static RE_ID: Lazy<Regex> = Lazy::new(|| Regex::new(r"(\d+)").unwrap());
        let _id = RE_ID
            .captures(l0)
            .unwrap()[1]
            .parse::<u32>()
            .unwrap();

        // Extract winning numbers
        let winning_numbers: HashSet<u32> = HashSet::from_iter(l2
            .split_whitespace()
            .map(|x| x.parse::<u32>().unwrap()));

        // Extract scratch numbers
        let scratch_numbers: HashSet<u32> = HashSet::from_iter(l3
            .split_whitespace()
            .map(|x| x.parse::<u32>().unwrap()));

        Scratchcard{winning_numbers, scratch_numbers}
    }

    pub fn wins(&self) -> usize {
        self.winning_numbers
            .intersection(&self.scratch_numbers)
            .count()
    }

    pub fn score(&self) -> u32 {
        let wins = self.wins() as u32;
        match wins {
            0 => 0,
            _ => 2u32.pow(wins-1)
        }
    }
}

/// Process the problem for this day and stage.
pub fn process(input: &str) -> u32 {
    input.lines()
        .map(|l| Scratchcard::from_line(l))
        .map(|x| x.score())
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
        let expected: u32 = 13;
        assert_eq!(process(input), expected);
    }
}

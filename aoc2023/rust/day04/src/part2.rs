use crate::part1::Scratchcard;

/// Process the problem for this day and stage.
pub fn process(input: &str) -> u32 {
    let wins: Vec<usize> = input.lines()
    .map(|l| Scratchcard::from_line(l))
    .map(|x| x.wins()).collect();

    let size = wins.len();
    let mut n_cards= vec![1u32; size];
    for (i, &w) in wins.iter().enumerate() {
        for j in 0..w {
            n_cards[i + j + 1] += n_cards[i];
        }
    }

    n_cards.iter().sum()
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
        let expected: u32 = 30;
        assert_eq!(process(input), expected);
    }
}

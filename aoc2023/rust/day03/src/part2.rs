use crate::part1::{Schematic, EnginePart};

/// Process the problem for this day and stage.
pub fn process(input: &str) -> u32 {
    let schematic = Schematic::from_document(input);

    // Obtain the gears
    let gears = schematic.symbols
        .iter()
        .filter(|e| e.character == '*');

    // Calculate the gear ratios
    let gear_ratio = gears.filter_map(|g| {
        let parts: Vec<&EnginePart> = g.bordering_parts(schematic.parts.iter()).collect();
        return if parts.len() == 2 {
            Some(parts[0].number() * parts[1].number())
        } else { None }
    });

    return gear_ratio.sum();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
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
        let expected: u32 = 467835;
        assert_eq!(process(input), expected);
    }
}

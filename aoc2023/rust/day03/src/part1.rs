#[derive(Debug)]
pub struct Coordinates {
    x: u32,
    y: u32,
}

impl Coordinates {
    pub fn is_bordering(&self, other: &Coordinates) -> bool {
        return (self.x.abs_diff(other.x) <= 1) && (self.y.abs_diff(other.y) <= 1)
    }
}


#[derive(Debug)]
pub struct SchematicElement {
    pub coordinates: Coordinates,
    pub character: char,
}

impl SchematicElement {
    pub fn bordering_parts<'a, I>(&'a self, parts: I) -> impl Iterator<Item = &EnginePart> + 'a
    where I: Iterator<Item = &'a EnginePart> + 'a{
        parts.filter(|e| e.borders_a_symbol(self))
    }
}

#[derive(Debug)]
pub struct EnginePart {
    elements: Vec<SchematicElement>,
}

impl EnginePart {
    /// Create a bare EnginePart.
    fn new() -> EnginePart {
        EnginePart{ elements: Vec::new()}
    }

    /// Add a schematic element to the engine part.
    fn push(&mut self, element: SchematicElement) {
        self.elements.push(element);
    }

    /// Parse the engine part number from the characters.
    pub fn number(&self) -> u32 {
        let number_str: String = self.elements
            .iter()
            .map(|e| e.character)
            .collect();
        number_str.parse::<u32>().unwrap()
    }

    /// True if the engine part borders the given symbol.
    fn borders_a_symbol(&self, symbol: &SchematicElement) -> bool {
        self.elements.iter()
            .any(|e| symbol.coordinates.is_bordering(&e.coordinates))
    }

    /// True if the engine part borders any of the given symbols.
    fn borders_any_symbol<'a, I>(&self, mut symbols: I) -> bool
    where I: Iterator<Item = &'a SchematicElement> {
        symbols.any(|s| self.borders_a_symbol(s))
    }
}

#[derive(Debug)]
pub struct Schematic {
    pub symbols: Vec<SchematicElement>,
    pub parts: Vec<EnginePart>,
}

impl Schematic {
    ///Build the schematic from the document.
    pub fn from_document(document: &str) -> Schematic {
        let mut parts = Vec::new();
        let mut part = EnginePart::new();
        let mut symbols = Vec::new();

        let mut x: u32 = 0;
        let mut y: u32 = 0;
        for character in document.chars() {
            let coordinates = Coordinates{x, y};
            let element = SchematicElement{ coordinates, character};
            match character {
                '\n' => {
                    x = 0; y += 1;
                    if !part.elements.is_empty() {parts.push(part);part = EnginePart::new();}
                    continue;
                }
                x if x.is_digit(10) => {part.push(element);}
                '.' => {
                    if !part.elements.is_empty() {parts.push(part);part = EnginePart::new();}
                }
                _ => {
                    symbols.push(element);
                    if !part.elements.is_empty() {parts.push(part);part = EnginePart::new();}
                }
            }
            x += 1;
        }
        Schematic{symbols, parts}
    }

    /// Engine parts which are adjacent to symbols.
    pub fn valid_parts<'a>(&'a self) -> impl Iterator<Item = &EnginePart> + 'a {
        self.parts
            .iter()
            .filter(|e| e.borders_any_symbol(self.symbols.iter()))
    }

    /// The numbers of the engine parts which are adjacent to symbols.
    pub fn valid_part_numbers<'a>(&'a self) -> impl Iterator<Item = u32> + 'a {
        self.valid_parts().map(|e| e.number())
    }
}


/// Process the problem for this day and stage.
pub fn process(input: &str) -> u32 {
    let schematic = Schematic::from_document(input);
    schematic.valid_part_numbers().sum()
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
        let expected: u32 = 4361;
        assert_eq!(process(input), expected);
    }
}

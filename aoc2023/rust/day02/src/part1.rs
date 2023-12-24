use {
    once_cell::sync::Lazy,
    regex::Regex,
};

const MAX_RED: u32 = 12;
const MAX_GREEN: u32 = 13;
const MAX_BLUE: u32 = 14;

#[derive(Debug)]
pub struct CubeDraws {
    r: u32,
    g: u32,
    b: u32,
}

impl CubeDraws {
    pub fn power(&self) -> u32 {
        self.r * self.g * self.b
    }
}

#[derive(Debug)]
pub struct GameRound {
    game_id: u32,
    draws: Vec<CubeDraws>,
}

impl GameRound {
    /// Create an instance from a line of the input.
    pub fn from_line(line: &str) -> GameRound {
        let (game_str, sets_str) = line.split_once(":").unwrap();

        // Extract the game ID
        static RE_ID: Lazy<Regex> = Lazy::new(|| Regex::new(
            r"Game (\d+)"
        ).unwrap());
        let game_id = RE_ID
            .captures(game_str)
            .unwrap()[1]
            .parse::<u32>()
            .unwrap();

        // Extract the results from each game set
        let mut draws = Vec::new();
        let set_str = sets_str.split(";");
        static RE_SET: Lazy<Regex> = Lazy::new(|| Regex::new(
            r"(\d+) (green|blue|red)"
        ).unwrap());
        for set in set_str {
            let mut r = 0;
            let mut g = 0;
            let mut b = 0;
            let matches = RE_SET
                .captures_iter(set)
                .map(|c| c.extract());
            for (_, [count_str, colour]) in matches {
                let count = count_str.parse::<u32>().unwrap();
                match colour {
                    "red" => { r += count; }
                    "green" => { g += count; }
                    "blue" => { b += count; }
                    _ => { panic!("Unexpected colour: {}", colour) }
                }
            }
            draws.push(CubeDraws {r, g, b});
        }
        GameRound{game_id, draws}
    }

    /// True if the cube draws are within the colour maximums.
    pub fn is_valid(&self) -> bool {
        for draw in &self.draws {
            if draw.r > MAX_RED { return false; }
            if draw.g > MAX_GREEN { return false; }
            if draw.b > MAX_BLUE { return false; }
        }
        return true;
    }

    /// Obtain a CubeDraws struct describing the minimum number of cubes for
    /// each colour needed for the game draws to have taken place.
    pub fn cubes_minima(&self) -> CubeDraws {
        CubeDraws{
            r: self.draws.iter().map(|d| d.r).max().unwrap(),
            b: self.draws.iter().map(|d| d.b).max().unwrap(),
            g: self.draws.iter().map(|d| d.g).max().unwrap(),
        }
    }
}

pub fn parse<'a>(
    input: &'a str
) -> impl Iterator<Item = GameRound> + 'a {
    input.lines().map(|l| GameRound::from_line(l))
}

/// Process the problem for this day and stage.
pub fn process(input: &str) -> u32 {
    parse(input).map(|r| if r.is_valid() {r.game_id} else {0}).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
        let expected: u32 = 8;
        assert_eq!(process(input), expected);
    }
}

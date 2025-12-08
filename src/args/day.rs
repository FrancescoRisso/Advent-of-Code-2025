use std::fmt::Display;

#[derive(Debug, Clone)]
pub enum Day {
    // One,
    // Two,
    // Three,
    // Four,
    // Five,
    // Six,
    // Seven,
    Eight,
    Nine,
    Ten,
    Eleven,
    Twelve,
}

impl Display for Day {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let disp = match self {
            // Day::One => "1",
            // Day::Two => "2",
            // Day::Three => "3",
            // Day::Four => "4",
            // Day::Five => "5",
            // Day::Six => "6",
            // Day::Seven => "7",
            Day::Eight => "8",
            Day::Nine => "9",
            Day::Ten => "10",
            Day::Eleven => "11",
            Day::Twelve => "12",
        };

        write!(f, "{}", disp)
    }
}

impl From<String> for Day {
    fn from(value: String) -> Self {
        match value.trim() {
            // "1" | "01" => Day::One,
            // "2" | "02" => Day::Two,
            // "3" | "03" => Day::Three,
            // "4" | "04" => Day::Four,
            // "5" | "05" => Day::Five,
            // "6" | "06" => Day::Six,
            // "7" | "07" => Day::Seven,
            "8" | "08" => Day::Eight,
            "9" | "09" => Day::Nine,
            "10" => Day::Ten,
            "11" => Day::Eleven,
            "12" => Day::Twelve,
            _ => panic!("Invalid day"),
        }
    }
}

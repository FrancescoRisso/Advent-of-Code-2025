use std::fmt::Display;

#[derive(Debug, Clone)]
pub enum Exercise {
    One,
    Two,
}

impl Display for Exercise {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let disp = match self {
            Exercise::One => "1",
            Exercise::Two => "2",
        };

        write!(f, "{}", disp)
    }
}

impl From<String> for Exercise {
    fn from(value: String) -> Self {
        match value.trim() {
            "1" | "01" => Exercise::One,
            "2" | "02" => Exercise::Two,
            _ => panic!("Invalid exercise"),
        }
    }
}

use crate::args::day::Day;

#[derive(Debug, Clone)]
pub enum InputFile {
    Example,
    Real,
    Test,
}

impl From<String> for InputFile {
    fn from(value: String) -> Self {
        match value.trim().to_lowercase().as_str() {
            "example" => InputFile::Example,
            "real" => InputFile::Real,
            "test" => InputFile::Test,
            _ => panic!("Invalid file"),
        }
    }
}

impl InputFile {
    pub fn path(&self, day: &Day) -> String {
        let folder = match self {
            InputFile::Example => "example",
            InputFile::Real => "real",
            InputFile::Test => "test",
        };

        let day_str: String = match day.to_string().len() {
            0 => unreachable!(),
            1 => format!("0{}", day),
            2 => day.to_string(),
            _ => unreachable!(),
        };

        format!("./inputs/{}/day_{}.txt", folder, day_str)
    }
}

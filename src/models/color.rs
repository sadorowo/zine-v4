use std::fmt::{Display, Formatter};
use std::str::FromStr;

#[derive(Debug)]
pub struct Color(pub u32);

#[derive(Debug)]
pub struct ColorError;

impl Display for ColorError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("invalid color")
    }
}

impl std::error::Error for ColorError {}

impl Display for Color {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let color = format!("{:06x}", self.0);
        f.write_str(&color)
    }
}

impl FromStr for Color {
    type Err = ColorError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let color = match s {
            "red" => 0xFF0000,
            "green" => 0x00FF00,
            "blue" => 0x0000FF,
            "yellow" => 0xFFFF00,
            "cyan" => 0x00FFFF,
            "magenta" => 0xFF00FF,
            "white" => 0xFFFFFF,
            "black" => 0x000000,
            "gray" => 0x808080,
            "orange" => 0xFFA500,
            "purple" => 0x800080,
            "brown" => 0xA52A2A,
            "maroon" => 0x800000,
            "lime" => 0x00FF00,
            "olive" => 0x808000,
            "navy" => 0x000080,
            "teal" => 0x008080,
            "silver" => 0xC0C0C0,
            "crimson" => 0xDC143C,
            "coral" => 0xFF7F50,
            "indigo" => 0x4B0082,
            "turquoise" => 0x40E0D0,
            "pink" => 0xFFC0CB,
            "lavender" => 0xE6E6FA,
            "beige" => 0xF5F5DC,
            _ => match u32::from_str_radix(s.trim_start_matches(&['#', '0', 'x']), 16) {
                Ok(color) => color,
                Err(_) => return Err(ColorError),
            },
        };

        Ok(Color(color))
    }
}
use color_eyre::eyre::{eyre, Result};
use regex::Regex;

#[derive(Default, Debug, Clone, Copy)]
pub struct Color {
    r: u8,
    g: u8,
    b: u8,
}

impl Color {
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Color { r, g, b }
    }

    pub fn rgba(&self) -> [u8; 4] {
        [self.r, self.g, self.b, 0xff]
    }

    pub fn add(&self, addend: Color) -> Self {
        Color {
            r: self.r + addend.r,
            g: self.g + addend.g,
            b: self.b + addend.b,
        }
    }

    pub fn multiply(&self, multiplier: Color) -> Self {
        Color {
            r: ((self.r * multiplier.r) / 0xff),
            g: ((self.g * multiplier.g) / 0xff),
            b: ((self.b * multiplier.b) / 0xff),
        }
    }

    pub fn scale(&self, factor: f64) -> Result<Self> {
        if factor < 0. {
            Err(eyre!("Can't scale color values by negative amount"))
        } else {
            Ok(Color {
                r: (self.r as f64 * factor) as u8,
                g: (self.g as f64 * factor) as u8,
                b: (self.b as f64 * factor) as u8,
            })
        }
    }

    // Parse hex colors like #fff, #abc123
    pub fn parse(color: impl Into<String>) -> Result<Self> {
        let color: String = color.into().replace(' ', "");

        match color.chars().count() {
            6 => {
                let six_digit_regex = Regex::new(r"#([\da-f]{2})([\da-f]{2})([\da-f]{2})/i")?;
                return if let Some((_, [r, g, b])) =
                    six_digit_regex.captures(&color).map(|c| c.extract())
                {
                    let (r, g, b) = (r.parse()?, g.parse()?, b.parse()?);

                    Ok(Color { r, g, b })
                } else {
                    Err(eyre!(r#"Error parsing color from string: "{color}""#))
                };
            }
            3 => {
                let three_digit_regex = Regex::new(r"#([\da-f])([\da-f])([\da-f])")?;
                return if let Some((_, [r, g, b])) =
                    three_digit_regex.captures(&color).map(|c| c.extract())
                {
                    let (r, g, b) = (r.parse()?, g.parse()?, b.parse()?);

                    Ok(Color { r, g, b })
                } else {
                    Err(eyre!(r#"Error parsing color from string: "{color}""#))
                };
            }
            _ => {
                if color.starts_with("rgb(") && color.ends_with(')') && color.len() == 10 {
                    let colors: Box<[&str]> = color[3..color.len() - 1].split(',').collect();

                    let (r, g, b) = (colors[0].parse()?, colors[1].parse()?, colors[2].parse()?);

                    Ok(Color { r, g, b })
                } else {
                    Err(eyre!(r#"Error parsing color from string: "{color}""#))
                }
            }
        }
    }
}

impl TryFrom<String> for Color {
    type Error = color_eyre::Report;

    fn try_from(value: String) -> Result<Self> {
        Color::parse(value)
    }
}

impl TryFrom<&str> for Color {
    type Error = color_eyre::Report;

    fn try_from(value: &str) -> Result<Self> {
        Color::parse(value)
    }
}

impl From<(u8, u8, u8)> for Color {
    fn from((r, g, b): (u8, u8, u8)) -> Self {
        Color::new(r, g, b)
    }
}

impl From<&Color> for Color {
    fn from(color: &Color) -> Self {
        color.clone()
    }
}

impl From<Color> for sdl2::pixels::Color {
    fn from(color: Color) -> Self {
        sdl2::pixels::Color::RGB(color.r, color.g, color.b)
    }
}

impl From<&Color> for sdl2::pixels::Color {
    fn from(color: &Color) -> Self {
        sdl2::pixels::Color::RGB(color.r, color.g, color.b)
    }
}

pub const WHITE: Color = Color {
    r: 255,
    g: 255,
    b: 255,
};

pub const BLACK: Color = Color { r: 0, g: 0, b: 0 };

pub const GREY: Color = Color {
    r: 127,
    g: 127,
    b: 127,
};

pub const RED: Color = Color { r: 255, g: 0, b: 0 };

pub const GREEN: Color = Color { r: 0, g: 255, b: 0 };

pub const BLUE: Color = Color { r: 0, g: 0, b: 255 };

pub const YELLOW: Color = Color {
    r: 255,
    g: 255,
    b: 0,
};

pub const MAGENTA: Color = Color {
    r: 255,
    g: 0,
    b: 255,
};

pub const CYAN: Color = Color {
    r: 0,
    g: 255,
    b: 255,
};

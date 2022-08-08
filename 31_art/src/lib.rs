pub use self::kinds::{Color, PrimaryColor, SecondaryColor};
pub use self::utils::mix;

/// # Kinds
///
/// Contains Kind of colors
pub mod kinds {
    /// # Color
    ///
    /// Gathering trait
    pub trait Color {
        fn to_str(&self) -> &str;
    }

    /// # Primary color enum
    #[derive(Debug)]
    pub enum PrimaryColor {
        Red,
        Yellow,
        Blue,
    }
    impl Color for PrimaryColor {
        fn to_str(&self) -> &str {
            match self {
                PrimaryColor::Red => "Red",
                PrimaryColor::Yellow => "Yellow",
                PrimaryColor::Blue => "Blue",
            }
        }
    }

    /// # Secondary color enum
    #[derive(Debug)]
    pub enum SecondaryColor {
        Orange,
        Purple,
        Green,
    }
    impl Color for SecondaryColor {
        fn to_str(&self) -> &str {
            match self {
                SecondaryColor::Orange => "Orange",
                SecondaryColor::Purple => "Purple",
                SecondaryColor::Green => "Green",
            }
        }
    }
}
/// # Utils
///
/// Contains method to use colors
pub mod utils {
    use crate::kinds::{Color, PrimaryColor, SecondaryColor};
    /// # Mix
    ///
    /// Mix color together
    ///
    pub fn mix(color_left: PrimaryColor, color_right: PrimaryColor) -> Box<dyn Color> {
        match color_left {
            PrimaryColor::Red => match color_right {
                PrimaryColor::Yellow => Box::new(SecondaryColor::Orange),
                PrimaryColor::Blue => Box::new(SecondaryColor::Purple),
                _ => Box::new(color_left),
            },
            PrimaryColor::Yellow => match color_right {
                PrimaryColor::Red => Box::new(SecondaryColor::Orange),
                PrimaryColor::Blue => Box::new(SecondaryColor::Green),
                _ => Box::new(color_left),
            },
            PrimaryColor::Blue => match color_right {
                PrimaryColor::Red => Box::new(SecondaryColor::Purple),
                PrimaryColor::Yellow => Box::new(SecondaryColor::Green),
                _ => Box::new(color_left),
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        use kinds::{PrimaryColor, SecondaryColor};
        use utils;

        assert_eq!(
            utils::mix(PrimaryColor::Red, PrimaryColor::Blue).to_str(),
            SecondaryColor::Purple.to_str()
        );
        assert_eq!(
            utils::mix(PrimaryColor::Blue, PrimaryColor::Blue).to_str(),
            PrimaryColor::Red.to_str(),
        );
    }
}

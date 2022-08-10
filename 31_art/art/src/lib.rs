pub use self::kinds::{NamedColor, PrimaryColor, SecondaryColor};
pub use self::utils::mix;

/// # Kinds
///
/// Contains Kind of colors
pub mod kinds {
    pub use named_color::NamedColor;
    use named_color_derive::NamedColor;

    /// # Primary color enum
    #[derive(Debug, NamedColor)]
    pub enum PrimaryColor {
        Red,
        Yellow,
        Blue,
    }

    /// # Secondary color enum
    #[derive(Debug, NamedColor)]
    pub enum SecondaryColor {
        Orange,
        Purple,
        Brown,
        Green,
    }
}
/// # Utils
///
/// Contains method to use colors
pub mod utils {
    use crate::kinds::{NamedColor, PrimaryColor, SecondaryColor};
    /// # Mix
    ///
    /// Mix color together
    ///
    pub fn mix(color_left: PrimaryColor, color_right: PrimaryColor) -> Box<dyn NamedColor> {
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
            utils::mix(PrimaryColor::Red, PrimaryColor::Blue).name(),
            SecondaryColor::Purple.name()
        );
        assert_eq!(
            utils::mix(PrimaryColor::Blue, PrimaryColor::Blue).name(),
            PrimaryColor::Blue.name(),
        );
        assert_ne!(
            utils::mix(PrimaryColor::Blue, PrimaryColor::Blue).name(),
            PrimaryColor::Red.name(),
        );
    }
}

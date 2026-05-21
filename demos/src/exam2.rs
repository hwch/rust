#[derive(Debug, PartialEq, Clone, Copy)]
pub enum ShirtColor {
    Red,
    Blue,
}

pub struct Inventory {
    pub shirts: Vec<ShirtColor>,
}

impl Inventory {
    pub fn giveaway(&self, color: Option<ShirtColor>) -> ShirtColor {
        color.unwrap_or_else(|| self.most_stocked())
    }

    fn most_stocked(&self) -> ShirtColor {
        let mut red_num = 0;
        let mut blue_num = 0;

        for color in &self.shirts {
            match color {
                ShirtColor::Red => red_num += 1,
                ShirtColor::Blue => blue_num += 1,
            }
        }

        if red_num > blue_num {
            ShirtColor::Red
        } else {
            ShirtColor::Blue
        }
    }
}

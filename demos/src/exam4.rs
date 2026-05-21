#[derive(Debug, PartialEq)]
pub struct Shoe {
    pub size: u32,
    pub style: String,
}

pub fn shoes_in_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    shoes.into_iter().filter(|s| s.size == shoe_size).collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_shoes_in_size() {
        let v1 = vec![
            Shoe {
                size: 10,
                style: String::from("a"),
            },
            Shoe {
                size: 13,
                style: String::from("b"),
            },
            Shoe {
                size: 10,
                style: String::from("c"),
            },
        ];
        let v2 = shoes_in_size(v1, 10);
        assert_eq!(
            v2,
            vec![
                Shoe {
                    size: 10,
                    style: String::from("a"),
                },
                Shoe {
                    size: 10,
                    style: String::from("c"),
                },
            ]
        )
    }
}

#[derive(Debug, PartialEq)]
struct Shoe {
    size: u32,
    style: String,
}

impl Shoe {
    fn new(size: u32, style: String) -> Shoe {
        Shoe { size, style }
    }
}

fn shoes_in_size(shoes: Vec<Shoe>, size: u32) -> Vec<Shoe> {
    shoes.into_iter().filter(|shoe| shoe.size == size).collect()
}

fn shoes_in_size_slice<'a>(shoes: &'a Vec<Shoe>, size: u32) -> Vec<&'a Shoe> {
    shoes.iter().filter(|shoe| shoe.size == size).collect()
}

mod tests {
    use super::*;
    #[test]
    fn test_0() {
        let list = vec![1, 2, 3];
        let mut list_iter = list.iter();
        assert_eq!(list_iter.next(), Some(&1));
        assert_eq!(list_iter.next(), Some(&2));
        assert_eq!(list_iter.next(), Some(&3));
        assert_eq!(list_iter.next(), None);
        let mut list_iter = list.iter();
        assert_eq!(list_iter.next(), Some(&1));
        let sum: u32 = list_iter.sum();
        assert_eq!(sum, 5);
        let v2: Vec<_> = list.iter().map(|x| x + 1).collect();
        assert_eq!(v2, vec![2, 3, 4]);
    }

    #[test]
    fn test_shoes() {
        let shoes = vec![
            Shoe::new(32, "basket".to_string()),
            Shoe::new(40, "mocassin".to_string()),
            Shoe::new(38, "basket".to_string()),
            Shoe::new(39, "basket".to_string()),
        ];
        let shoe_38: Vec<Shoe> = shoes_in_size(shoes, 38);
        assert_eq!(shoe_38, vec![Shoe::new(38, "basket".to_string())]);
    }

    #[test]
    fn test_shoes_slice() {
        let shoes = vec![
            Shoe::new(40, "basket".to_string()),
            Shoe::new(40, "mocassin".to_string()),
            Shoe::new(38, "basket".to_string()),
            Shoe::new(39, "basket".to_string()),
        ];
        let shoe_38: Vec<&Shoe> = shoes_in_size_slice(&shoes, 40);
        assert_eq!(
            shoe_38,
            vec![
                &Shoe::new(40, "basket".to_string()),
                &Shoe::new(40, "mocassin".to_string())
            ]
        );
    }
}

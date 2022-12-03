fn main() {
    let mut inventory = Inventory::new();
    inventory.push_stock(ShirtColor::Red);
    inventory.push_stock(ShirtColor::Red);
    inventory.push_stock(ShirtColor::Red);
    inventory.push_stock(ShirtColor::Blue);
    inventory.push_stock(ShirtColor::Blue);
    inventory.push_stock(ShirtColor::Blue);
    inventory.push_stock(ShirtColor::Blue);

    let user_pref = Some(ShirtColor::Red);
    let giveaway = inventory.giveaway(user_pref);
    println!("giveaway : {:?}", giveaway);

    let user_pref = None;
    let giveaway = inventory.giveaway(user_pref);
    println!("giveaway : {:?}", giveaway);
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum ShirtColor {
    Red,
    Blue,
}

struct Inventory {
    shirts: Vec<ShirtColor>,
}

impl Inventory {
    fn new() -> Self {
        Self { shirts: vec![] }
    }

    fn push_stock(&mut self, color: ShirtColor) {
        self.shirts.push(color);
    }

    fn giveaway(&mut self, user_preference: Option<ShirtColor>) -> ShirtColor {
        let result = user_preference.unwrap_or_else(|| self.most_stocked());
        let idx = self.shirts.iter().position(|s| *s == result).unwrap();
        self.shirts.remove(idx);
        result
    }

    fn most_stocked(&self) -> ShirtColor {
        let mut num_red = 0;
        let mut num_blue = 0;

        for color in &self.shirts {
            match color {
                ShirtColor::Red => num_red += 1,
                ShirtColor::Blue => num_blue += 1,
            }
        }

        if num_red > num_blue {
            ShirtColor::Red
        } else {
            ShirtColor::Blue
        }
    }
}

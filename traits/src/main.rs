use traits::*;

fn main() {
    let apple = Fruit { weight_in_kg: 1.0 };

    println!("this apple gives {} units of strength", apple.gives());

    let steak = Meat {
        weight_in_kg: 1.0,
        fat_content: 1.0,
    };

    let mut player1 = Player {
        name: String::from("player1"),
        strength: 1.0,
        score: 0,
        money: 0,
        weapons: vec![String::from("knife")],
    };
    println!("Before eating {:?}", player1);
    player1.eat(apple);
    println!("After eating an apple\n{}", player1);
    player1.eat(steak);
    println!("After eating a steak\n{}", player1);
}

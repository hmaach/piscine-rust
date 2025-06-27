use borrow_box::*;

fn main() {
    let mut game = GameSession::new(0, String::from("Joao"), String::from("Susana"), 5);
    println!("{:?}", game.read_winner());
    // output : ("Same score! tied", 0)

    game.update_score(String::from("Joao"));
    game.update_score(String::from("Joao"));
    game.update_score(String::from("Susana"));
    game.update_score(String::from("Susana"));
    println!("{:?}", game.read_winner());
    // output : ("Same score! tied", 2)

    game.update_score(String::from("Joao"));
    // this one will not count because it already 5 games played, the nb_games
    game.update_score(String::from("Susana"));

    println!("{:?}", game.read_winner());
    // output : ("Joao", 3)

    println!("{:?}", game.delete());
    // output : "game deleted: id -> 0"

    // game.read_winner();
    // this will give error
    // because the game was dropped, no longer exists on the heap
}

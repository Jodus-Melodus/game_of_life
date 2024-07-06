use game::Game;

mod game;

fn main() {
    let game = Game::new(30);

    game.run(100, 0);
}

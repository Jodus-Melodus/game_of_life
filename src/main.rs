use std::{process::Command, thread, time::Duration};

use game::Game;

mod game;

fn clear_console() {
    if cfg!(windows) {
        let _ = Command::new("cmd").arg("/c").arg("cls").status();
    } else {
        let _ = Command::new("sh").arg("-c").arg("clear").status();
    }
}

fn main() {
    let board = vec![
        vec![0, 0, 0, 0, 0, 0],
        vec![0, 0, 0, 1, 1, 0],
        vec![0, 0, 0, 1, 1, 0],
        vec![0, 1, 1, 0, 0, 0],
        vec![0, 1, 1, 0, 0, 0],
        vec![1, 0, 0, 0, 0, 0],
    ];

    let mut game = Game::from_vec_matrix(board);

    loop {
        clear_console();
        game.show();
        game = game.step();
        thread::sleep(Duration::from_millis(500));
    }
}

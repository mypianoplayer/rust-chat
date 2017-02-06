extern crate env_logger;

use std::thread;

mod game1;
mod game2;

fn main() {
    env_logger::init().unwrap();

    thread::spawn(|| {
        let mut game = game1::Game::new("0.0.0.0:10000");
        game.start();
    });

    let mut game = game2::Game::new("0.0.0.0:10001");
    game.start();

}

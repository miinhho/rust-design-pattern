use crate::{magic_maze::MagicMaze, ordinary_maze::OrdinaryMaze};

mod game;
mod magic_maze;
mod ordinary_maze;

fn main() {
    let ordinary_maze = OrdinaryMaze::new();
    game::run(ordinary_maze);

    let magic_maze = MagicMaze::new();
    game::run(magic_maze);
}

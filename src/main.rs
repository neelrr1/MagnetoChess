mod engine;

use anyhow::{bail, Result};
use chess::{Board, ChessMove, Game};
use dialoguer::Input;

fn main() -> Result<()> {
    println!("Hello, world! Let's play chess!");
    play_game()?;

    Ok(())
}

fn play_game() -> Result<()> {
    let mut game = Game::new();
    let mut move_count = 0;
    let mut engine_turn = false;

    while game.result().is_none() && move_count < 200 {
        if engine_turn {
            let engine_move = engine::get_move(&game.current_position())?;
            game.make_move(engine_move);
        } else {
            let player_move = get_player_move(&game.current_position())?;
            game.make_move(player_move);
        }
        engine_turn = !engine_turn;
        move_count += 1;
        println!("Move {}: {}", move_count, game.current_position());
    }

    println!("The result of the game was: {:?}", game.result());
    println!("Final Position: {}", game.current_position());
    println!("Took {} moves", move_count);
    Ok(())
}

fn get_player_move(board: &Board) -> Result<ChessMove> {
    let input: String = Input::new()
        .with_prompt("Please enter a move in SAN format")
        .validate_with(|input: &String| -> Result<(), &str> {
            if ChessMove::from_san(board, input).is_err() {
                Err("Please enter a legal move!")
            } else {
                Ok(())
            }
        })
        .interact_text()?;

    match ChessMove::from_san(board, &input) {
        Ok(player_move) => Ok(player_move),
        Err(_) => bail!("Error while getting the player's move"),
    }
}

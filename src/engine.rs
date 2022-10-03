use anyhow::Context;
use anyhow::Result;
use chess::Board;
use chess::ChessMove;
use chess::MoveGen;
use rand::seq::IteratorRandom;

pub fn get_move(board: &Board) -> Result<ChessMove> {
    let mut rng = rand::thread_rng();
    let moves = MoveGen::new_legal(board);

    moves.choose(&mut rng).context("No legal moves!")
}

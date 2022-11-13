use rand::Rng;

use crate::board::Board;

mod board;
mod field;

const MINE_COUNT: usize = 25;

fn main() {
    let mut board = Board::new(MINE_COUNT, 16, 32);
    let mut rng = rand::thread_rng();
    let mut placed_mines: usize = 0;

    while placed_mines < board.mine_count {
        let x = rng.gen_range(0..board.x_size);
        let y = rng.gen_range(0..board.y_size);

        let possible_mine = &mut board.fields[x as usize][y as usize];
        if possible_mine.mine { continue; }

        possible_mine.mine = true;
        possible_mine.value = 9;
        placed_mines += 1;

        for xd in -1..=1 {
            for yd in -1..=1 {
                let xx = x + xd;
                let yy = y + yd;
                if xx < 0 || xx >= board.x_size || yy < 0 || yy >= board.y_size || (yd == 0 && xd == 0) {
                    continue;
                }

                let x2 = &mut board.fields[xx as usize][yy as usize];
                if x2.mine { continue; }

                x2.value += 1;
            }
        }
    }

    for x in board.fields {
        for field in x {
            print!("{:?}", field.value);
        }
        println!()
    }
}
use rand::Rng;

fn main() {
    let board_size: i32 = 16;

    let mut rng = rand::thread_rng();
    let mut board = vec![vec![0i32; board_size as usize]; board_size as usize];

    for _ in 0..25 {
        let x = rng.gen_range(0..board_size);
        let y = rng.gen_range(0..board_size);

        board[x as usize][y as usize] = 9;
        for xd in -1..=1 {
            for yd in -1..=1 {
                let xx = x + xd;
                let yy = y + yd;
                if yy < 0 || yy >= board_size || xx < 0 || xx >= board_size || (yd == 0 && xd == 0) {
                    continue;
                }

                let x2 = &mut board[xx as usize][yy as usize];
                if *x2 >= 9 { continue; }

                *x2 += 1;
            }
        }
    }

    for x in board {
        println!("{:?}", x);
    }
}

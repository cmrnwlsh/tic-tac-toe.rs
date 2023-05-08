extern crate core;
use core::fmt;

#[derive(Clone, Copy, Eq, PartialEq)]
enum Player {
    P1,
    P2,
    None
}

enum State {
    Stalemate,
    Winner(Player),
    InProgress
}

struct Board {
    grid: [[Player; 3]; 3],
    status: State,
    player_current: Player
}

impl fmt::Display for Player {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Player::P1 => write!(f, "O"),
            Player::P2 => write!(f, "X"),
            Player::None => write!(f, ".")
        }
    }
}

impl Board {
    pub fn new() -> Board {
        Board { grid: [[Player::None; 3]; 3],
            player_current: Player::P1,
            status: State::InProgress }
    }

    const WIN_CONDITIONS: [[[usize; 2]; 3]; 8] = [
    [[0, 0], [1, 1], [2, 2]],
    [[0, 2], [1, 1], [2, 0]],
    [[0, 0], [0, 1], [0, 2]],
    [[1, 0], [1, 1], [1, 2]],
    [[2, 0], [2, 1], [2, 2]],
    [[0, 0], [1, 0], [2, 0]],
    [[0, 1], [1, 1], [2, 1]],
    [[0, 2], [1, 2], [2, 2]]
    ];

    pub fn is_won(&self) -> bool {
        Self::WIN_CONDITIONS.iter().any(|condition|
            condition.iter().all(|coord| {
                let [x, y] = coord;
                self.grid[*x][*y] == self.player_current
            }))
    }

    pub fn stalemate(&self) -> bool {
        self.grid.iter().flatten().all(|&coords| coords != Player::None)
    }

    pub fn play_move(&mut self, [x, y]: [usize; 2]) {
        self.grid[x][y] = self.player_current;
        if self.is_won() {
            self.status = State::Winner(self.player_current);
        }
        else if self.stalemate() {
            self.status = State::Stalemate;
        }
        else {
            self.status = State::InProgress;
        }
        match self.player_current {
            Player::P1 => self.player_current = Player::P2,
            Player::P2 => self.player_current = Player::P1,
            Player::None => {}
        };
    }

    pub fn display_board(&self) {
        self.grid.iter().for_each(|x| {{x
            .iter().for_each(|y| print!("{y}"))}; println!();})
    }
}

fn main() {
    let mut game = Board::new();
    loop {
        let mut line = String::new();
        println!("Enter coordinates (Row major, space separated):");
        std::io::stdin().read_line(&mut line).unwrap();
        let mut iter = line.split_whitespace();
        if let (Some(x), Some(y)) = (iter.next(), iter.next()) {
            game.play_move([x.parse().unwrap(), y.parse().unwrap()]);
        }
        game.display_board();
        match game.status {
            State::Stalemate => {println!("stalemate"); break}
            State::Winner(player) => {println!("{player} wins"); break}
            State::InProgress => {}
        }
    }
}

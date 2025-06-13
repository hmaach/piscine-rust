// pub fn tic_tac_toe(table: [[char; 3]; 3]) -> String {

// }

// pub fn diagonals(player: char, table: [[char; 3]; 3]) -> bool {
// }

pub fn horizontal(player: char, table: [[char; 3]; 3]) -> bool {
    for row in table {
        let mut count = 0;
        for col in row {
            if col == player {
                count += 1;
            }
        }
        if count == 3 {
            return true;
        }
    }
    false
}

// pub fn vertical(player: char, table: [[char; 3]; 3]) -> bool {
// }

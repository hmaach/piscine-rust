pub fn tic_tac_toe(table: [[char; 3]; 3]) -> String {
    if diagonals('O', table) || horizontal('O', table) || vertical('O', table) {
        return "player O won".to_owned();
    }
    if diagonals('X', table) || horizontal('X', table) || vertical('X', table) {
        return "player X won".to_owned();
    }
    "tie".to_owned()
}

pub fn diagonals(player: char, table: [[char; 3]; 3]) -> bool {
    if table[0][0] == player && table[1][1] == player && table[2][2] == player {
        return true;
    }

    if table[0][2] == player && table[1][1] == player && table[2][0] == player {
        return true;
    }

    false
}

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

pub fn vertical(player: char, table: [[char; 3]; 3]) -> bool {
    let mut col = 0;
    
    while col < 3 {
        let mut row = 0;
        let mut count = 0;
        while row < 3 {
            if table[row][col] == player {
                count += 1;
            }
            row += 1;
        }

        if count == 3 {
            return true;
        }
        col += 1;
    }
    false
}

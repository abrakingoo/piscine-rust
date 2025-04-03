pub fn tic_tac_toe(table: [[char; 3]; 3]) -> String {
    // Check for player O or X win
    if horizontal('O', table) || vertical('O', table) || diagonals('O', table) {
        return "player O won".to_string();
    }
    if horizontal('X', table) || vertical('X', table) || diagonals('X', table) {
        return "player X won".to_string();
    }

    return "tie".to_string()
}


pub fn diagonals(player: char, table: [[char; 3]; 3]) -> bool {
    // Check both diagonals
    (table[0][0] == player && table[1][1] == player && table[2][2] == player) ||
    (table[0][2] == player && table[1][1] == player && table[2][0] == player)
}

pub fn horizontal(player: char, table: [[char; 3]; 3]) -> bool {
    // Check all rows
    table.iter().any(|row| row.iter().all(|&cell| cell == player))
}

pub fn vertical(player: char, table: [[char; 3]; 3]) -> bool {
    // Check all columns
    (0..3).any(|col| table.iter().all(|row| row[col] == player))
}

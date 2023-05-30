pub fn tic_tac_toe(table: Vec<Vec<&str>>) -> String {
    const P1: &str = "X";
    const P2: &str = "O";
    const DRAW_MSG: &str = "tie";
    if diagonals(P1, &table) || horizontal(P1, &table) || vertical(P1, &table) {
        return format!("player {} won", P1).to_string();
    } else if diagonals(P2, &table) || horizontal(P2, &table) || vertical(P2, &table) {
        return format!("player {} won", P2).to_string();
    } else {
        return DRAW_MSG.to_string();
    }
}

pub fn diagonals(player: &str, table: &Vec<Vec<&str>>) -> bool {
    let mut i_diag: usize = 0;
    if table.iter().all(|line| {i_diag+=1; return line[i_diag-1] == player;}) {return true;}
    i_diag = 3;
    if table.iter().all(|line| {i_diag-=1; return line[i_diag] == player;}) {return true;}
    false
}

pub fn horizontal(player: &str, table: &Vec<Vec<&str>>) -> bool {
    table.iter().any(|line| line.iter().all(|case| case == &player))
}

pub fn vertical(player: &str, table: &Vec<Vec<&str>>) -> bool {
    for cl in 0..=2 {
        if table.iter().all(|ln| ln[cl] == player) {return true;}
    }
    false
}
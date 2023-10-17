mod grid;

mod players;

//::{Player, IdPlayer, self}; 
use self::players::*;

pub struct GameData {
    pub grid: Vec<Vec<char>>,
    pub players: [Player; 2],
    pub current_player: usize,
    pub game_over: bool
}

impl GameData {
    pub fn new() -> GameData {
        // On demande les noms des joueurs 
        let (player1_name, player2_name) = set_player_names();

        let grid = grid::create_grid(6, 7);
        let player1 = Player::new(&player1_name, IdPlayer::Player1, "X");
        let player2 = Player::new(&player2_name, IdPlayer::Player2, "O");
        
        GameData {
            grid,
            players: [player1, player2],
            current_player: 0,
            game_over: false
        }
    }

    pub fn get_current_player_name(&self) -> &str {
        &self.players[self.current_player].name
    }

    pub fn get_player_names(&self) -> (String, String) {
        (self.players[0].name.to_string(), self.players[1].name.to_string())
    }

    pub fn display(&self) {
        grid::display_grid(&self.grid);
    }

    pub fn make_move(&mut self, column: usize) -> Result<(), &str> {
       
        if column >= self.grid[0].len() {
            self.display();
            return Err("La colonne n'est pas valide, veuillez en choisir une autre");
        }

        // Ancienne version de la boucle
        // let mut row = None;
        // for r in (0..self.grid.len()).rev() {
        //     if self.grid[r][column] == ' ' {
        //         row = Some(r);
        //         break;
        //     }
        // }

        let row = self.grid.iter_mut().rev().find(|r| r[column] == ' ');

        if let Some(row) = row {

            // Placez le jeton du joueur actuel dans la grille
            let current_player = &self.players[self.current_player];
            row[column] = current_player.symbol.chars().next().unwrap();
        
            // Actualise le joueur courant
            self.current_player = 1 - self.current_player;

            Ok(())
        }
        else {
            Err("La colonne est pleine, veuillez en choisir une autre")
        }
    }

    pub fn play_game(&mut self) {
        
        // Détermine le joueur courant
        let current_player = &self.players[self.current_player];
        
        // Affiche la grille vide
        self.display();
    
        println!("C'est à {} de jouer ({}).", current_player.name, current_player.symbol);
    
        let mut valid_move = false;
        while !valid_move {
            // Demande au joueur courant de choisir la colonne
            let column = players::get_column_choice();
    
            // Essayez de placer une pièce sur la grille
            match self.make_move(column) {
                Ok(_) => {
                    valid_move = true;
                    // Effacement de la grille de jeu pour actualiser le terminal
                    clearscreen::clear().expect("Échec de l'effacement de l'écran !");
                }
                Err(err) => {
                    println!("Erreur : {}", err);
                }
            }
        }
    }

    pub fn is_game_over(&self) -> bool {
        let symbol_chars: Vec<char> = self.players.iter().map(|player| player.symbol.chars().next().unwrap()).collect();
        for row in &self.grid {
            if let Some(symbol) = row.iter().find(|&&cell| cell != ' ') {
                if row.windows(4).any(|window| window.iter().all(|&cell| cell == *symbol)) {
                    return true;
                }
            }
        }
    
        for col in 0..self.grid[0].len() {
            for i in 0..self.grid.len() - 3 {
                if (0..4).all(|j| {
                    let cell = self.grid[i + j][col];
                    symbol_chars.contains(&cell)
                }) {
                    return true;
                }
            }
        }
    
        for row in 0..self.grid.len() - 3 {
            for col in 0..self.grid[0].len() - 3 {
                if (0..4).all(|i| {
                    (0..4).all(|j| {
                        let cell1 = self.grid[row + i + j][col + i + j];
                        let cell2 = self.grid[row + i + j][col + 3 - i - j];
                        symbol_chars.contains(&cell1) || symbol_chars.contains(&cell2)
                    })
                }) {
                    return true;
                }
            }
        }
    
        false
    }    

    pub fn is_game_draw(&self) -> bool {
        for row in &self.grid {
            for cell in row {
                if *cell == ' ' {
                    // Il y a une case vide, le jeu n'est pas terminé
                    return false;
                }
            }
        }
        
        true
    }

    pub fn timeout(&mut self) {
        self.game_over = true;
    }

}

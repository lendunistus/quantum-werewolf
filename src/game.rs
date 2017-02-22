use std::collections::HashSet;

use player::Player;

/// The possible errors returned by `Game::new`.
#[derive(Debug)]
pub enum NewGameError {
    /// There are less than three players.
    NotEnoughPlayers,
    /// Multiple players have the same name.
    NameCollision(String)
}

/// This represents the state of a game.
#[derive(Debug)]
pub struct Game {
    //TODO
}

impl Game {
    /// Creates a new game from a list of players.
    ///
    /// # Errors
    ///
    /// Will return an error if no game can be created with the given player list. See `NewGameError` for details.
    pub fn new(players: Vec<Box<Player>>) -> Result<Game, NewGameError> {
        if players.len() < 3 {
            return Err(NewGameError::NotEnoughPlayers);
        }
        for ((i1, p1), (i2, p2)) in iproduct!(players.iter().enumerate(), players.iter().enumerate()) {
            if i1 != i2 && p1.name() == p2.name() {
                return Err(NewGameError::NameCollision(p1.name().to_owned()));
            }
        }
        Ok(Game {})
    }

    /// Runs the entire game and returns the names of the winners.
    pub fn run(self) -> HashSet<String> {
        unimplemented!() //TODO game loop
    }
}
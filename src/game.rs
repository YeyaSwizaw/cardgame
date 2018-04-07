use server::Game;

pub struct TestGame;

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Player {
    pub name: String
}

impl Player {
    pub fn new(name: String) -> Self {
        Player {
            name
        }
    }
}

impl Game for TestGame {
    type Player = Player;
}

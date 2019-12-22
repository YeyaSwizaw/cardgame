#[macro_use] extern crate serde_derive;
extern crate serde_json;
extern crate serde;
extern crate server;

use server::GameServer;
use crate::game::TestGame;

mod game;

fn main() {
    let _server = GameServer::<TestGame>::spawn("127.0.0.1:8080").unwrap();
    loop {}
}

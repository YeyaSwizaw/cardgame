#[macro_use] extern crate serde_derive;
extern crate serde_json;
extern crate serde;
extern crate client;
extern crate server;

use std::{
    thread,
    io::{self, Write},
    sync::{Arc, Mutex},
};

use client::{
    event::{Event, EventKind},
    GameClient,
};

use game::{TestGame, Player};

mod game;

fn handle_event(client: &Arc<Mutex<GameClient<TestGame>>>, event: Event<TestGame>) {
    match event.event {
        EventKind::ChatMessage(msg) => println!("{}: {}", client.lock().unwrap().player(event.from).map(|player| player.name).unwrap_or_else(|| "???".to_string()), msg),
        EventKind::Connection(addr) => println!("Connection from: {}", addr),
        _ => (),
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let ip = args[1].clone();
    let port = args[2].clone();

    let client = GameClient::spawn([ip, port].join(":")).unwrap();
    let thread_client = client.clone();

    thread::spawn(move || {
        let client = thread_client;

        loop {
            let events: Vec<_> = {
                let lock = client.lock().unwrap();
                let iter = lock.events();
                iter.collect()
            };

            for event in events {
                handle_event(&client, event);
            }
        }
    });

    print!("Enter Name:");
    io::stdout().flush().expect("Could not flush stdout");

    let name = {
        let mut input = String::new();
        io::stdin().read_line(&mut input).map(|_| input).unwrap()
    };

    client.lock().unwrap().update(Player::new(name));

    loop {
        let mut input = String::new();
        if io::stdin().read_line(&mut input).is_ok() {
            client.lock().unwrap().chat(input);
        }
    }
}

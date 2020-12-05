use std::io::{self, BufRead};
use serde::{Serialize, Deserialize};

// ----- INPUT -----

#[derive(Serialize, Deserialize)]
struct Planet {
    name: String,
    x: f64,
    y: f64,
    owner: Option<usize>,
    ship_count: usize,
}

#[derive(Serialize, Deserialize)]
struct Expedition {
    id: usize,
    origin: String,
    destination: String,
    turns_remaining: usize,
    owner: usize,
    ship_count: usize,
}

#[derive(Serialize, Deserialize)]
struct GameState {
    planets: Vec<Planet>,
    expeditions: Vec<Expedition>,
}

// ----- OUTPUT -----

#[derive(Serialize, Deserialize)]
struct Move {
    origin: String,
    destination: String,
    ship_count: usize
}

#[derive(Serialize, Deserialize)]
struct Turn {
    moves: Vec<Move>,
}

fn next_move(state: &GameState) -> Turn {
    let my_planets: Vec<&Planet> = state.planets
                                        .iter()
                                        .filter(|x| x.owner.unwrap_or(0) == 1)
                                        .collect();
    let other_planets: Vec<&Planet> = state.planets
                                        .iter()
                                        .filter(|x| x.owner.unwrap_or(0) != 1)
                                        .collect();

    if my_planets.len() == 0 || other_planets.len() == 0 {
        Turn { moves: vec![] }
    } else {
        let planet = my_planets
                        .iter()
                        .max_by_key(|x| x.ship_count)
                        .expect("Unreachable statement");
        let dest = other_planets
                        .iter()
                        .max_by_key(|x| x.ship_count)
                        .expect("Unreachable statement");
        Turn {
            moves: vec![Move {
                origin: planet.name.to_string(),
                destination: dest.name.to_string(),
                ship_count: planet.ship_count - 1
            }]
        }
    }
}

fn main() {
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let state: GameState = serde_json::from_str(line.expect("Could not deserialize").as_str()).unwrap();
        let turn: Turn = next_move(&state);
        println!("{}", serde_json::to_string(&turn).expect("Could not serialize"));
    }
}

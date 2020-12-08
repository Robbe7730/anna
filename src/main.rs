use std::io::{self, BufRead};
use serde::{Serialize, Deserialize};
use itertools::iproduct;
use std::collections::HashSet;

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

// ----- HELPER FUNCTIONS -----

fn distance_between_planets(planet1: &Planet, planet2: &Planet) -> f64 {
    let dx = planet1.x - planet2.x;
    let dy = planet1.y - planet2.y;
    (dx * dx + dy * dy).sqrt()
}

fn simulate_arrivals(planet: &Planet, gamestate: &GameState) -> (usize, usize) {
    let mut relevant_expeditions = gamestate.expeditions
                                        .iter()
                                        .filter(|x| x.destination == planet.name)
                                        .collect::<Vec<&Expedition>>();
    relevant_expeditions.sort_by_key(|x| x.turns_remaining);
    let mut owner = planet.owner.unwrap_or(0);
    let mut ship_count = planet.ship_count;
    let mut last_simulated_turn = 0;
    for expedition in relevant_expeditions {
        // Account for growth
        if owner != 0 {
            ship_count += expedition.turns_remaining - last_simulated_turn;
        }
        last_simulated_turn = expedition.turns_remaining;

        if expedition.owner == owner {
            ship_count += expedition.ship_count;
        } else {
            if ship_count < expedition.ship_count {
                owner = expedition.owner;
                ship_count = expedition.ship_count - ship_count;
            } else if ship_count == expedition.ship_count {
                owner = 0;
                ship_count = 0;
            } else {
                ship_count -= expedition.ship_count;
            }
        }
    }
    (owner, ship_count)
}

fn score(source: &Planet, dest: &Planet, gamestate: &GameState) -> (usize, usize) {
    let (owner, ship_count) = simulate_arrivals(dest, gamestate);
    if (ship_count+1) >= source.ship_count || owner == 1 {
        (0, 0)
    } else {
        (
            ship_count+1,
            distance_between_planets(source, dest).ceil() as usize * (ship_count+1)
        )
    }
}

// ----- NEXT MOVE -----

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
        let mut moves = vec![];

        let mut best_move = iproduct!(my_planets.iter(), other_planets.iter())
            .map(|(s,d)| (s, d, score(s, d, state)))
            .filter(|(_,_,(_,sc))| *sc != 0)
            .min_by_key(|x| (*x).2);

        let mut used_planets: HashSet<String> = HashSet::new();

        while let Some((source, dest, (ship_count, _score))) = best_move {
            moves.push(Move {
                origin: source.name.to_string(),
                destination: dest.name.to_string(),
                ship_count: ship_count,
            });

            used_planets.insert(source.name.to_string());

            best_move = iproduct!(
                my_planets.iter().filter(|x| !used_planets.contains(&x.name)),
                other_planets.iter()
            ).map(|(s,d)| (s, d, score(s, d, state)))
             .filter(|(_,_,(_, sc))| *sc != 0)
             .min_by_key(|x| (*x).2);
        }
        Turn { moves: moves }
    }
}

// ----- MAIN -----

fn main() {
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let state: GameState = serde_json::from_str(line.expect("Could not deserialize").as_str()).unwrap();
        let turn: Turn = next_move(&state);
        println!("{}", serde_json::to_string(&turn).expect("Could not serialize"));
    }
}

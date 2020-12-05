use std::io::{self, BufRead};
use serde::{Serialize, Deserialize};
use itertools::iproduct;

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

fn distance_between_planets(planet1: &Planet, planet2: &Planet) -> f64 {
    let dx = planet1.x - planet2.x;
    let dy = planet1.y - planet2.y;
    (dx * dx + dy * dy).sqrt()
}

fn incoming_ship_diff(planet: &Planet, gamestate: &GameState) -> isize {
    gamestate.expeditions
        .iter()
        .filter(|x| x.destination == planet.name)
        .map(|x| {
            if let Some(planet_owner) = planet.owner {
                if x.owner == planet_owner {
                    1
                } else {
                    -1
                }
            } else {
                -1
            }
        })
        .sum()
}

fn ship_count_on_arrival(source: &Planet, dest: &Planet, gamestate: &GameState) -> isize {
    let distance = distance_between_planets(source, dest).ceil() as usize;
    let expeditions_diff: isize = incoming_ship_diff(dest, gamestate);

    if dest.owner.is_none() {
        dest.ship_count as isize + expeditions_diff
    } else {
        (dest.ship_count + distance) as isize + expeditions_diff
    }
}

fn score(source: &Planet, dest: &Planet, gamestate: &GameState) -> usize {
    let ship_count = ship_count_on_arrival(source, dest, gamestate);
    if ship_count < (source.ship_count as isize + incoming_ship_diff(source, gamestate)) {
        0
    } else {
        distance_between_planets(source, dest).ceil() as usize * ship_count as usize
    }
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
        let mut moves = vec![];

        // TODO: more moves
        let best_move = iproduct!(my_planets, other_planets)
                                .map(|(s,d)| (s, d, score(s, d, state)))
                                .filter(|(_,_,sc)| *sc != 0)
                                .min_by_key(|x| (*x).2);

        if let Some((source, dest, score)) = best_move {
            eprintln!("found move with score {}", score);
            moves.push(Move {
                origin: source.name.to_string(),
                destination: dest.name.to_string(),
                ship_count: ship_count_on_arrival(source, dest, state) as usize + 1
            });
        }
        Turn { moves: moves }
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

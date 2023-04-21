#[macro_use]
extern crate rocket;
use rocket::http::Status;
use rocket::serde::{json::Json, Serialize};
use rocket::State;
use std::sync::atomic::{AtomicUsize, Ordering};

pub mod war;

struct Scoring {
    player_1: AtomicUsize,
    player_2: AtomicUsize,
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
struct ScoringJSON {
    player_1: usize,
    player_2: usize,
}

#[get("/start")]
fn index(scoring: &State<Scoring>) -> &'static str {
    let res = war::play();
    match res {
        "Player 1" => {
            scoring.player_1.fetch_add(1, Ordering::Relaxed);
            "Player 1"
        }
        "Player 2" => {
            scoring.player_2.fetch_add(1, Ordering::Relaxed);
            "Player 2"
        }
        _ => "Draw!",
    }
}

#[get("/scores")]
fn scores(scoring: &State<Scoring>) -> Json<ScoringJSON> {
    Json(ScoringJSON {
        player_1: scoring.player_1.load(Ordering::Relaxed),
        player_2: scoring.player_2.load(Ordering::Relaxed),
    })
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index, scores])
        .manage(Scoring {
            player_1: AtomicUsize::new(0),
            player_2: AtomicUsize::new(0),
        })
}

#[macro_use]
extern crate rocket;
use rocket::serde::{json::Json, Serialize};

use rocket_db_pools::sqlx;
use rocket_db_pools::{Connection, Database};

pub mod war;

#[derive(Database)]
#[database("postgres_score")]
struct ScoringDB(sqlx::PgPool);

#[allow(dead_code)]
struct ScoringRow {
    player: String,
    score: i64,
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
struct ScoringJSON {
    player_1: i64,
    player_2: i64,
}

#[get("/start")]
async fn index(mut db: Connection<ScoringDB>) -> &'static str {
    let res = war::play();
    match res {
        "Player 1" => {
            sqlx::query!("update scoring set score = score+1 where player = 'Player1'")
                .execute(&mut *db)
                .await
                .expect("Could not update player score in DB");
            "Player 1"
        }
        "Player 2" => {
            sqlx::query!("update scoring set score = score+1 where player = 'Player2'")
                .execute(&mut *db)
                .await
                .expect("Could not update player score in DB");
            "Player 2"
        }
        _ => "Draw!",
    }
}

#[get("/scores")]
async fn scores_db(mut db: Connection<ScoringDB>) -> Json<ScoringJSON> {
    // Retrieve score from DB
    let mut score: Vec<ScoringRow> = sqlx::query_as!(ScoringRow, "select * from scoring")
        .fetch_all(&mut *db)
        .await
        .expect("Unable to query scoring table");

    // If the table is empty, create the correct rows with score 0
    if score.is_empty() {
        sqlx::query!("insert into scoring values ('Player1', 0)")
            .execute(&mut *db)
            .await
            .expect("Could not update player score in DB");
        sqlx::query!("insert into scoring values ('Player2', 0)")
            .execute(&mut *db)
            .await
            .expect("Could not update player score in DB");
        score = vec![
            ScoringRow {
                player: String::from("Player1"),
                score: 0,
            },
            ScoringRow {
                player: String::from("Player2"),
                score: 0,
            },
        ];
    }

    Json(ScoringJSON {
        player_1: score[0].score,
        player_2: score[1].score,
    })
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index, scores_db])
        .attach(ScoringDB::init())
}

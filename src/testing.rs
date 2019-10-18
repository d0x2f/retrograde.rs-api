extern crate parking_lot;

use super::schema::{board, participant};
use super::{embedded_migrations, guards, rocket};
use parking_lot::Mutex;
use rocket::config::{Config, Environment, Value};
use rocket::local::Client;
use std::collections::HashMap;
use super::models::{Board, NewBoard, Rank, NewRank};
use rocket::http::ContentType;

static DB_LOCK: Mutex<()> = Mutex::new(());

use diesel::pg::PgConnection;
use diesel::prelude::*;

fn test_cleanup(db: &PgConnection) {
  embedded_migrations::run(db).expect("database migrations");

  diesel::delete(board::table)
    .execute(db)
    .expect("database cleanup (board)");
  diesel::delete(participant::table)
    .execute(db)
    .expect("database cleanup (participant)");
}

pub fn run_test<F>(test: F)
where
  F: FnOnce(Client, &PgConnection),
{
  let mut database_config = HashMap::new();
  let mut databases = HashMap::new();

  database_config.insert(
    "url",
    Value::from("postgres://postgres:postgres@127.0.0.1/retrograde"),
  );
  database_config.insert("pool_size", Value::from(2));
  databases.insert("postgres", Value::from(database_config));

  let config = Config::build(Environment::Development)
    .address("0.0.0.0")
    .port(80)
    .secret_key("apnUQicUZ8QRDN1+rlIGdvhfdabLCTg4aGd0MHFQIPQ=")
    .extra("databases", databases)
    .finalize()
    .unwrap();

  let rocket = rocket(config);
  let db = guards::DatabaseConnection::get_one(&rocket).expect("database connection");
  let client = Client::new(rocket).expect("valid rocket instance");

  let _lock = DB_LOCK.lock();
  test_cleanup(&db);
  test(client, &db);
}

/// Create a board
pub fn create_board(client: &Client, board: &NewBoard ) -> Board {
  let mut response = client
    .post("/boards")
    .header(ContentType::JSON)
    .body(serde_json::to_string(board).unwrap())
    .dispatch();
  serde_json::from_str(response.body_string().unwrap().as_str()).unwrap()
}

/// Create a rank
pub fn create_rank(client: &Client, rank: &NewRank) -> Rank {
  let mut response = client
    .post(format!("/boards/{}/ranks", rank.board_id))
    .header(ContentType::JSON)
    .body(serde_json::to_string(rank).unwrap())
    .dispatch();
  serde_json::from_str(response.body_string().unwrap().as_str()).unwrap()
}
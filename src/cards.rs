use super::models::*;
use super::persistence;
use super::guards::BoardOwner;
use super::guards::DatabaseConnection;
use super::guards::ParticipantId;
use super::guards::RankInBoard;
use super::guards::CardInRank;
use log::error;
use rocket::http::Status;
use rocket_contrib::json::{Json, JsonValue};
use diesel::result::Error;

#[post("/boards/<board_id>/ranks/<rank_id>/cards", data = "<post_card>")]
pub fn post_card(
    _participant_id: ParticipantId,
    _board_owner: BoardOwner,
    _rank_in_board: RankInBoard,
    postgres: DatabaseConnection,
    board_id: String,
    rank_id: String,
    post_card: Json<PostCard>,
) -> Result<JsonValue, Status> {
    // Check that voting is open for the board
    let cards_open = match persistence::cards_open(&postgres, &board_id) {
        Ok(b) => Ok(b),
        Err(Error::NotFound) => Err(Status::NotFound),
        Err(_) =>  Err(Status::InternalServerError)
    }?;

    if !cards_open {
        return Err(Status::Forbidden);
    }

    let new_card = NewCard {
        id: None,
        name: post_card.name,
        description: post_card.description,
        rank_id: &rank_id,
    };

    persistence::put_card(&postgres, new_card)
        .map(|card| json!(card))
        .map_err(|error| {
            error!("{}", error.to_string());
            Status::InternalServerError
        })
}

#[get("/boards/<_board_id>/ranks/<rank_id>/cards")]
pub fn get_cards(
    _participant_id: ParticipantId,
    _rank_in_board: RankInBoard,
    postgres: DatabaseConnection,
    _board_id: String,
    rank_id: String,
) -> Result<JsonValue, Status> {
    persistence::get_cards(&postgres, &rank_id)
        .map(|cards| json!(cards))
        .map_err(|error| {
            error!("{}", error.to_string());
            Status::InternalServerError
        })
}

#[get("/boards/<_board_id>/ranks/<_rank_id>/cards/<card_id>")]
pub fn get_card(
    _participant_id: ParticipantId,
    _rank_in_board: RankInBoard,
    _card_in_rank: CardInRank,
    postgres: DatabaseConnection,
    _board_id: String,
    _rank_id: String,
    card_id: String,
) -> Result<JsonValue, Status> {
    let card = persistence::get_card(&postgres, &card_id).map_err(|error| {
        error!("{}", error.to_string());
        Status::InternalServerError
    })?;
    Ok(json!(card))
}

#[patch("/boards/<_board_id>/ranks/<_rank_id>/cards/<card_id>", data = "<update_card>")]
#[allow(clippy::too_many_arguments)]
pub fn patch_card(
    _participant_id: ParticipantId,
    _board_owner: BoardOwner,
    _rank_in_board: RankInBoard,
    _card_in_rank: CardInRank,
    postgres: DatabaseConnection,
    _board_id: String,
    _rank_id: String,
    card_id: String,
    update_card: Json<UpdateCard>,
) -> Result<JsonValue, Status> {
    persistence::patch_card(&postgres, &card_id, &update_card)
        .map(|card| json!(card))
        .map_err(|error| {
            error!("{}", error.to_string());
            Status::InternalServerError
        })
}

#[delete("/boards/<_board_id>/ranks/<_rank_id>/cards/<card_id>")]
#[allow(clippy::too_many_arguments)]
pub fn delete_card(
    _participant_id: ParticipantId,
    _board_owner: BoardOwner,
    _rank_in_board: RankInBoard,
    _card_in_rank: CardInRank,
    postgres: DatabaseConnection,
    _board_id: String,
    _rank_id: String,
    card_id: String,
) -> Result<(), Status> {
    persistence::delete_card(&postgres, &card_id)
        .map(|_| ())
        .map_err(|error| {
            error!("{}", error.to_string());
            Status::InternalServerError
        })
}

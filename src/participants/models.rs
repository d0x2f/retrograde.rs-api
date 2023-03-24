use crate::config::Config;
use crate::error;
use actix_identity::Identity;
use actix_web::dev::Payload;
use actix_web::web::Data;
use actix_web::{FromRequest, HttpRequest};
use firestore::{FirestoreDb, FirestoreTimestamp};
use futures::future::Future;
use serde::{Deserialize, Serialize};
use std::pin::Pin;

#[derive(Deserialize, Serialize, Clone)]
pub struct Participant {
  pub id: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NewParticipant {
  pub created_at: FirestoreTimestamp,
}

#[derive(Deserialize)]
pub struct ParticipantInFirestore {
  pub _firestore_id: String,
  pub _firestore_created: FirestoreTimestamp,
  pub boards: Option<Vec<String>>,
}

impl From<ParticipantInFirestore> for Participant {
  fn from(participant: ParticipantInFirestore) -> Self {
    Participant {
      id: participant._firestore_id,
    }
  }
}

impl FromRequest for Participant {
  type Error = error::Error;
  type Future = Pin<Box<dyn Future<Output = Result<Self, Self::Error>>>>;

  fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
    let req = req.clone();
    Box::pin(async move {
      let firestore = req.app_data::<Data<FirestoreDb>>().unwrap();
      let config = req.app_data::<Data<Config>>().unwrap();
      super::new(
        config,
        firestore,
        Identity::from_request(&req, &mut Payload::None),
        req.clone(),
      )
      .await
    })
  }
}

use super::draft::Draft;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct UserId(Uuid);

impl UserId {
    fn new() -> Self {
        Self(Uuid::new_v4())
    }
}

#[derive(Debug, Serialize)]
pub struct User {
    pub id: UserId,
    pub draft: Draft,
}

impl User {
    fn new(id: UserId, draft: Draft) -> Self {
        Self {
            id: id,
            draft: draft,
        }
    }
}

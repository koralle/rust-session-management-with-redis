use anyhow::Result;
use serde::Deserialize;
use uuid::Uuid;

use super::draft::Draft;

#[derive(Debug, Deserialize)]
pub struct UserId(Uuid);

impl UserId {
    fn new() -> Self {
        Self(Uuid::new_v4())
    }
}

#[derive(Debug)]
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

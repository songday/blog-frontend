use std::vec::Vec;

use chrono::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Default, Deserialize, Serialize)]
pub(crate) struct User {
    pub email: String,
    pub password: String,
    pub access_token: String,
}

#[derive(Deserialize, Serialize)]
pub(crate) struct Post {
    pub id: String,
    pub title: String,
    pub content: String,
    pub tags: Vec<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}
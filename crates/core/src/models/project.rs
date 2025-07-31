use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct Project {
    pub id: Uuid,
    pub name: String,
    pub description: String,
    pub created_at: DateTime<Utc>,
}

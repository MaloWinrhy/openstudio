use chrono::{DateTime, Utc};
use uuid::Uuid;

use crate::models::project_status::{ProjectStatus, Visibility};

#[derive(Debug, Clone)]
pub struct Project {
    pub id: Uuid,
    pub name: String,
    pub description: String,
    pub created_at: DateTime<Utc>,
    pub visibility: Visibility,
    pub status: ProjectStatus,
}
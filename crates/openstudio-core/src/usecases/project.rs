use crate::models::{project::Project, project_status::{ProjectStatus, Visibility}};
use chrono::Utc;
use uuid::Uuid;

pub fn create_project(name: &str, description: &str) -> Project {
    Project {
        id: Uuid::new_v4(),
        name: name.to_string(),
        description: description.to_string(),
        created_at: Utc::now(),
        visibility: Visibility::Private,
        status: ProjectStatus::Draft,
    }
}

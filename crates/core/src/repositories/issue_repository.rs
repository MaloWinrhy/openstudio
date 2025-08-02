use crate::models::issue::Issue;
use uuid::Uuid;

pub trait IssueRepository: Send + Sync {
    fn list_by_project(&self, project_id: Uuid) -> Result<Vec<Issue>, String>;
    fn get_by_id(&self, id: Uuid) -> Result<Option<Issue>, String>;
    fn save(&self, issue: Issue) -> Result<(), String>;
    fn update(&self, issue: Issue) -> Result<bool, String>;
    fn delete(&self, id: Uuid) -> Result<bool, String>;
}

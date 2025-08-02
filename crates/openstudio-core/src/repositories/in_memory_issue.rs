use crate::models::issue::Issue;
use crate::repositories::issue_repository::IssueRepository;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use uuid::Uuid;

pub struct InMemoryIssueRepo {
    issues: Arc<Mutex<HashMap<Uuid, Issue>>>,
}

impl InMemoryIssueRepo {
    pub fn new() -> Self {
        Self {
            issues: Arc::new(Mutex::new(HashMap::new())),
        }
    }
}

impl IssueRepository for InMemoryIssueRepo {
    fn list_by_project(&self, project_id: Uuid) -> Result<Vec<Issue>, String> {
        let issues = self.issues.lock().unwrap();
        Ok(issues.values().filter(|i| i.project_id == project_id).cloned().collect())
    }
    fn get_by_id(&self, id: Uuid) -> Result<Option<Issue>, String> {
        let issues = self.issues.lock().unwrap();
        Ok(issues.get(&id).cloned())
    }
    fn save(&self, issue: Issue) -> Result<(), String> {
        let mut issues = self.issues.lock().unwrap();
        issues.insert(issue.id, issue);
        Ok(())
    }
    fn update(&self, issue: Issue) -> Result<bool, String> {
        let mut issues = self.issues.lock().unwrap();
        if issues.contains_key(&issue.id) {
            issues.insert(issue.id, issue);
            Ok(true)
        } else {
            Ok(false)
        }
    }
    fn delete(&self, id: Uuid) -> Result<bool, String> {
        let mut issues = self.issues.lock().unwrap();
        Ok(issues.remove(&id).is_some())
    }
}

use std::sync::{Arc, Mutex};

use crate::models::project::Project;
use crate::repositories::project_repository::ProjectRepository;

#[derive(Default)]
pub struct InMemoryProjectRepo {
    projects: Arc<Mutex<Vec<Project>>>,
}

impl InMemoryProjectRepo {
    pub fn new() -> Self {
        Self::default()
    }
}

impl ProjectRepository for InMemoryProjectRepo {
    fn save(&self, project: Project) -> anyhow::Result<()> {
        self.projects.lock().unwrap().push(project);
        Ok(())
    }

    fn list(&self) -> anyhow::Result<Vec<Project>> {
        Ok(self.projects.lock().unwrap().clone())
    }

    fn get_by_id(&self, id: uuid::Uuid) -> anyhow::Result<Option<Project>> {
        let projects = self.projects.lock().unwrap();
        Ok(projects.iter().find(|p| p.id == id).cloned())
    }

    fn delete(&self, id: uuid::Uuid) -> anyhow::Result<bool> {
        let mut projects = self.projects.lock().unwrap();
        let len_before = projects.len();
        projects.retain(|p| p.id != id);
        Ok(projects.len() < len_before)
    }

    fn update(&self, project: Project) -> anyhow::Result<bool> {
        let mut projects = self.projects.lock().unwrap();
        if let Some(existing) = projects.iter_mut().find(|p| p.id == project.id) {
            *existing = project;
            Ok(true)
        } else {
            Ok(false)
        }
    }
}

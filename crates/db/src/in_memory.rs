use std::sync::{Arc, RwLock};
use uuid::Uuid;
use openstudio_core::models::project::Project;
use openstudio_core::repositories::project_repository::ProjectRepository;

#[derive(Clone, Default)]
pub struct InMemoryProjectRepository {
    store: Arc<RwLock<Vec<Project>>>,
}

impl ProjectRepository for InMemoryProjectRepository {
    fn save(&self, project: Project) -> anyhow::Result<()> {
        self.store
            .write()
            .map_err(|e| anyhow::Error::msg(format!("RwLock poisoned: {}", e)))?
            .push(project.clone());
        Ok(())
    }

    fn list(&self) -> anyhow::Result<Vec<Project>> {
        let guard = self
            .store
            .read()
            .map_err(|e| anyhow::Error::msg(format!("RwLock poisoned: {}", e)))?;
        Ok(guard.clone())
    }

    fn get_by_id(&self, id: uuid::Uuid) -> anyhow::Result<Option<Project>> {
        let guard = self
            .store
            .read()
            .map_err(|e| anyhow::Error::msg(format!("RwLock poisoned: {}", e)))?;
        Ok(guard.iter().find(|p| p.id == id).cloned())
    }

    fn delete(&self, id: Uuid) -> Result<bool, anyhow::Error> {
        let mut guard = self
            .store
            .write()
            .map_err(|e| anyhow::Error::msg(format!("RwLock poisoned: {}", e)))?;
        let initial_len = guard.len();
        guard.retain(|p| p.id != id);
        let deleted = guard.len() < initial_len;
        Ok(deleted)
    }

    fn update(&self, project: Project) -> Result<bool, anyhow::Error> {
        let mut guard = self
            .store
            .write()
            .map_err(|e| anyhow::Error::msg(format!("RwLock poisoned: {}", e)))?;
        let updated = if let Some(existing) = guard.iter_mut().find(|p| p.id == project.id) {
            *existing = project.clone();
            true
        } else {
            false
        };
        Ok(updated)
    }
}

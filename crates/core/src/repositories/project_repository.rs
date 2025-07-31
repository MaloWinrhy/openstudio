use crate::models::project::Project;

pub trait ProjectRepository {
    fn save(&self, project: Project) -> anyhow::Result<()>;
    fn list(&self) -> anyhow::Result<Vec<Project>>;
    fn get_by_id(&self, id: uuid::Uuid) -> anyhow::Result<Option<Project>>;
    fn delete(&self, id: uuid::Uuid) -> anyhow::Result<bool>;
    fn update(&self, project: Project) -> anyhow::Result<bool>;
}

use crate::models::user::{User, ProjectMember, ProjectRole};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use uuid::Uuid;

pub struct InMemoryUserRepo {
    users: Arc<Mutex<HashMap<Uuid, User>>>,
    members: Arc<Mutex<Vec<ProjectMember>>>,
}

impl InMemoryUserRepo {
    pub fn new() -> Self {
        Self {
            users: Arc::new(Mutex::new(HashMap::new())),
            members: Arc::new(Mutex::new(Vec::new())),
        }
    }
    pub fn list_users(&self) -> Vec<User> {
        self.users.lock().unwrap().values().cloned().collect()
    }
    pub fn get_user(&self, id: Uuid) -> Option<User> {
        self.users.lock().unwrap().get(&id).cloned()
    }
    pub fn save_user(&self, user: User) {
        self.users.lock().unwrap().insert(user.id, user);
    }
    pub fn add_member(&self, member: ProjectMember) {
        self.members.lock().unwrap().push(member);
    }
    pub fn list_members(&self, project_id: Uuid) -> Vec<ProjectMember> {
        self.members.lock().unwrap().iter().filter(|m| m.project_id == project_id).cloned().collect()
    }
    pub fn remove_member(&self, project_id: Uuid, user_id: Uuid) {
        let mut members = self.members.lock().unwrap();
        members.retain(|m| !(m.project_id == project_id && m.user_id == user_id));
    }
}

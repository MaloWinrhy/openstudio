use chrono::Utc;
use uuid::Uuid;
use core::usecases::project::create_project;
use core::models::project_status::{ProjectStatus, Visibility};
use db::in_memory::InMemoryProjectRepository;
use core::repositories::project_repository::ProjectRepository;

#[test]
fn test_create_project() {
    let name = "Test Project";
    let description = "A test project description.";
    let project = create_project(name, description);

    println!("Création du projet: {{ id: {}, name: {}, description: {} }}", project.id, project.name, project.description);

    assert_eq!(project.name, name);
    assert_eq!(project.description, description);
    assert_eq!(project.visibility, Visibility::Private);
    assert_eq!(project.status, ProjectStatus::Draft);
    assert!(project.id != Uuid::nil());
    let now = Utc::now();
    let diff = now.signed_duration_since(project.created_at).num_seconds().abs();
    assert!(diff < 5);
    println!("Test de création OK");
}

#[test]
fn test_save_and_get_project() {
    let repo = InMemoryProjectRepository::default();
    let project = create_project("OpenStudio", "Collaborative Open Source Platform");
    repo.save(project.clone()).unwrap();
    let projects = repo.list().unwrap();
    println!("Projets en mémoire après save: {:?}", projects);
    assert_eq!(projects.len(), 1);
    assert_eq!(projects[0].name, "OpenStudio");
    println!("Test save & get OK");
}

#[test]
fn test_full_integration_api_usecase_db() {
    let repo = InMemoryProjectRepository::default();
    // API → usecase → db
    let project = create_project("API Project", "Test intégration");
    repo.save(project.clone()).unwrap();
    // Vérifie la persistance
    let all = repo.list().unwrap();
    println!("Après save: {:?}", all);
    assert_eq!(all.len(), 1);
    assert_eq!(all[0].name, "API Project");
    // Test get_by_id
    let found = repo.get_by_id(project.id).unwrap();
    println!("get_by_id: {:?}", found);
    assert!(found.is_some());
    assert_eq!(found.unwrap().description, "Test intégration");
    // Test update
    let mut updated = project.clone();
    updated.name = "Projet Modifié".to_string();
    let ok = repo.update(updated.clone()).unwrap();
    println!("update: {}", ok);
    assert!(ok);
    let found = repo.get_by_id(updated.id).unwrap().unwrap();
    println!("Après update: {:?}", found);
    assert_eq!(found.name, "Projet Modifié");
    // Test delete
    let deleted = repo.delete(updated.id).unwrap();
    println!("delete: {}", deleted);
    assert!(deleted);
    assert!(repo.get_by_id(updated.id).unwrap().is_none());
    println!("Test intégration complet OK");
}
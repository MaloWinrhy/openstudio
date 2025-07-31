use core::usecases::project::create_project;
use core::models::project_status::{ProjectStatus, Visibility};
use chrono::Utc;
use uuid::Uuid;

#[test]
fn test_create_project() {
    let name = "Test Project";
    let description = "A test project description.";
    let project = create_project(name, description);

    assert_eq!(project.name, name);
    assert_eq!(project.description, description);
    assert_eq!(project.visibility, Visibility::Private);
    assert_eq!(project.status, ProjectStatus::Draft);
    assert!(project.id != Uuid::nil());
    let now = Utc::now();
    let diff = now.signed_duration_since(project.created_at).num_seconds().abs();
    assert!(diff < 5);
}

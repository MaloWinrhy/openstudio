#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub enum Visibility {
    Public,
    Private,
    Unlisted,
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub enum ProjectStatus {
    Draft,
    Active,
    Archived,
}

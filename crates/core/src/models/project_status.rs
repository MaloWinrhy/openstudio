#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Visibility {
    Public,
    Private,
    Unlisted,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ProjectStatus {
    Draft,
    Active,
    Archived,
}

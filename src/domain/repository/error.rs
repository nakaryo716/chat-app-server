#[derive(Debug, Clone)]
pub enum RepositoryError {
    DbError,
    NotFound,
}

use crate::infrastructure::database::database::DATABASE_INSTANCE;
use sea_orm::DatabaseConnection;
use std::sync::Arc;

#[derive(Clone)]
pub struct DatabaseInstance<'a> {
    pub db: &'a Arc<DatabaseConnection>,
}

impl<'a> DatabaseInstance<'a> {
    pub fn new() -> Self {
        Self {
            db: &DATABASE_INSTANCE
                .get()
                .expect("Failed to get DatabaseConnection instance!"),
        }
    }
}

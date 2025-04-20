use crate::config::database::DATABASE_INSTANCE;
use sea_orm::DatabaseConnection;
use std::sync::Arc;

#[derive(Clone)]
pub struct DatabaseInstance<'a> {
    db: &'a Arc<DatabaseConnection>,
}

impl<'a> DatabaseInstance<'a> {
    fn new() -> Self {
        Self {
            db: &DATABASE_INSTANCE
                .get()
                .expect("Failed to get DatabaseConnection instance!"),
        }
    }
}

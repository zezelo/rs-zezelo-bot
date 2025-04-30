use sea_orm::sqlx::types::Json;
use std::any::Any;

struct PayloadHandler {
    payload: Box<Json<dyn Any>>,
}

impl PayloadHandler {
    fn new(payload: Box<dyn Any>) -> Self {
        Self {
            payload: Box::new(Json::from(payload)),
        }
    }

    fn save() {}
}

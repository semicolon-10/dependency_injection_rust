use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Clone)]
pub struct Product {
    pub id: Uuid,
    pub name: String
}

#[derive(Deserialize)]
pub struct ProductData{
    pub name: String
}

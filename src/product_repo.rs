use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use uuid::Uuid;
use crate::model::Product;

pub trait ProductRepo : Send + Sync {
    fn get_product(&self, id: Uuid) -> Option<Product>;

    fn save_product(&self, product: &Product);
}

#[derive(Debug, Clone, Default)]
pub struct InMemoryProductRepo {
    pub map: Arc<Mutex<HashMap<Uuid, Product>>>
}

impl ProductRepo for InMemoryProductRepo {
    fn get_product(&self, id: Uuid) -> Option<Product> {
        self.map.lock().unwrap().get(&id).cloned()
    }

    fn save_product(&self, product: &Product) {
       self.map.lock().unwrap().insert(product.id, product.clone());
    }
}

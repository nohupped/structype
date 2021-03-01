use serde::{Serialize, Deserialize};

pub type TypeMapVec = Vec<TypeMap>;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TypeMap {
    pub field_name: String,
    pub meta: std::collections::HashMap<String, String>,
}

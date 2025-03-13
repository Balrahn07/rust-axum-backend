use serde::{Serialize, Deserialize}; // ✅ Import serde for JSON handling

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct User {
    pub id: i32,  // ✅ Database auto-generates `id`
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NewUser {
    pub name: String, // ✅ Only `name` is required when creating a user
}

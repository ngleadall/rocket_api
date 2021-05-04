use super::schema::users;

#[derive(serde::Serialize, serde::Deserialize, Queryable, AsChangeset)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub email: String,
    #[serde(skip_deserializing)]
    pub created_at: String,
}

#[derive(serde::Deserialize, Insertable)]
#[table_name = "users"]
pub struct NewUser {
    pub name: String,
    pub email: String,
}

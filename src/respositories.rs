// Externals
use diesel::prelude::*;
use diesel::result::QueryResult;

// Internals
use super::models::*;
use super::schema::*;

pub struct UserRepository;

impl UserRepository {
  // Load 100 users
  pub fn load_all(c: &SqliteConnection) -> QueryResult<Vec<User>> {
    users::table.limit(100).load::<User>(c)
  }

  // Find user by id
  pub fn find(c: &SqliteConnection, id: i32) -> QueryResult<User> {
    users::table.find(id).get_result::<User>(c)
  }

  // Create user
  pub fn create(c: &SqliteConnection, new_user: NewUser) -> QueryResult<User> {
    diesel::insert_into(users::table)
      .values(new_user)
      .execute(c)?;

    let last_id = Self::last_id(c)?;

    Self::find(c, last_id)
  }

  // Update
  pub fn save(c: &SqliteConnection, user: User) -> QueryResult<User> {
    diesel::update(users::table.find(user.id))
      .set((
        users::name.eq(user.name.to_owned()),
        users::email.eq(user.email.to_owned()),
      ))
      .execute(c)?;

    Self::find(c, user.id)
  }

  pub fn delete(c: &SqliteConnection, id: i32) -> QueryResult<usize> {
    diesel::delete(users::table.find(id)).execute(c)
  }

  // Get last id
  fn last_id(c: &SqliteConnection) -> QueryResult<i32> {
    users::table
      .select(users::id)
      .order(users::id.desc())
      .first(c)
  }
}

use chrono::{NaiveDateTime, Utc};
use diesel::{prelude::*, result::Error};
use uuid::Uuid;

use crate::{schema::user, store::Store};
use bcrypt::{DEFAULT_COST, hash, verify};

#[derive(Queryable, Selectable, Insertable)]
#[diesel(table_name = user)]
#[diesel(check_for_backend(diesel::pg::Pg))]
struct User {
    id: String,
    username: String,
    password: String,
    created_at: NaiveDateTime,
    updated_at: NaiveDateTime,
}

impl Store {
    pub fn signup(&mut self, username: String, password: String) -> Result<String, Error> {
        let id = Uuid::new_v4();
        let hashed_password = hash(&password, DEFAULT_COST).unwrap();
        let current_time = Utc::now().naive_utc();
        let user = User {
            id: id.to_string(),
            username,
            password: hashed_password,
            created_at: current_time,
            updated_at: current_time,
        };

        let result = diesel::insert_into(user::table)
            .values(&user)
            .returning(User::as_returning())
            .get_result(&mut self.conn)?;

        Ok(result.id)
    }

    pub fn signin(
        &mut self,
        input_username: String,
        input_password: String,
    ) -> Result<String, Error> {
        let existing_user = user::table
            .filter(user::username.eq(input_username))
            .select(User::as_select())
            .first(&mut self.conn)?;

        let hash = &existing_user.password;
        let correct_password = verify(input_password, hash).unwrap();

        if correct_password {
            return Ok(existing_user.id);
        }

        Ok(String::from("wrong password!"))
    }
}

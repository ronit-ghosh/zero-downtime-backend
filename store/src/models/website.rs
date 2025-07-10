use chrono::{NaiveDateTime, Utc};
use diesel::{prelude::*, result::Error};
use uuid::Uuid;

use crate::{schema::website, store::Store};

#[derive(Queryable, Selectable, Insertable)]
#[diesel(table_name = website)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Website {
    pub id: String,
    pub url: String,
    pub user_id: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl Store {
    pub fn create_website(&mut self, user_id: String, input_url: String) -> Result<String, Error> {
        let existing_website = website::table
            .filter(website::url.eq(&input_url))
            .select(Website::as_returning())
            .first(&mut self.conn);

        match existing_website {
            Ok(website) => Ok(website.id),
            Err(Error::NotFound) => {
                let id = Uuid::new_v4();
                let current_time = Utc::now().naive_utc();
                let website = Website {
                    id: id.to_string(),
                    url: input_url,
                    user_id,
                    created_at: current_time,
                    updated_at: current_time,
                };

                let result = diesel::insert_into(website::table)
                    .values(website)
                    .returning(Website::as_returning())
                    .get_result(&mut self.conn)?;

                Ok(result.id)
            }
            Err(e) => Err(e),
        }
    }

    pub fn get_websites(&mut self) -> Result<Vec<Website>, Error> {
        let website = website::table
            .select(Website::as_returning())
            .load(&mut self.conn)?;

        Ok(website)
    }

    pub fn get_website_by_id(&mut self, website_id: String) -> Result<String, Error> {
        let website = website::table
            .filter(website::id.eq(&website_id))
            .select(Website::as_returning())
            .first(&mut self.conn)?;

        Ok(website.id)
    }
}

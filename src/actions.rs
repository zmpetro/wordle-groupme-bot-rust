use diesel::prelude::*;

use crate::models;

type DbError = Box<dyn std::error::Error + Send + Sync>;

pub fn get_all_time_stats(conn: &SqliteConnection) -> Result<Vec<models::AllTimeStats>, DbError> {
    use crate::schema::all_time_stats::dsl::*;
    let stats = all_time_stats.load::<models::AllTimeStats>(conn)?;
    Ok(stats)
}

pub fn get_name_from_user_id(conn: &SqliteConnection, user_id_: String) -> Result<Option<models::Name>, DbError> {
    use crate::schema::names::dsl::*;
    let id_and_name = names.filter(user_id.eq(user_id_)).first::<models::Name>(conn).optional()?;
    Ok(id_and_name)
}
use diesel::prelude::*;

use crate::models;

type DbError = Box<dyn std::error::Error + Send + Sync>;

pub fn get_all_time_stats(conn: &SqliteConnection) -> Result<Vec<models::AllTimeStats>, DbError> {
    use crate::schema::all_time_stats::dsl::*;
    let stats = all_time_stats.load::<models::AllTimeStats>(conn)?;
    Ok(stats)
}

pub fn get_names(conn: &SqliteConnection) -> Result<Vec<models::Name>, DbError> {
    use crate::schema::names::dsl::*;
    let ids_names = names.load::<models::Name>(conn)?;
    Ok(ids_names)
}

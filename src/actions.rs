use diesel::prelude::*;

use crate::models;

type DbError = Box<dyn std::error::Error + Send + Sync>;

pub fn get_all_time_stats(conn: &SqliteConnection) -> Result<Vec<models::AllTimeStats>, DbError> {
    use crate::schema::all_time_stats::dsl::*;
    let stats = all_time_stats.load::<models::AllTimeStats>(conn)?;
    Ok(stats)
}
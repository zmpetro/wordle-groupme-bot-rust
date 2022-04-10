use diesel::prelude::*;

use crate::models;

type DbError = Box<dyn std::error::Error + Send + Sync>;

pub fn get_aggregate_stats(
    conn: &SqliteConnection,
    which: bool,
) -> Result<Vec<models::AggregateStats>, DbError> {
    if which {
        use crate::schema::all_time_stats::dsl::*;
        let stats = all_time_stats.load::<models::AggregateStats>(conn)?;
        return Ok(stats);
    } else {
        use crate::schema::weekly_stats::dsl::*;
        let stats = weekly_stats.load::<models::AggregateStats>(conn)?;
        return Ok(stats);
    }
}

pub fn get_names(conn: &SqliteConnection) -> Result<Vec<models::Name>, DbError> {
    use crate::schema::names::dsl::*;
    let ids_names = names.load::<models::Name>(conn)?;
    Ok(ids_names)
}

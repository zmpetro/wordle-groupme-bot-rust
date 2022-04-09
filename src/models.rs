use serde::{Deserialize, Serialize};

#[derive(Debug, Queryable)]
pub struct AllTimeStats {
    pub player_id: String,
    pub games_played: i32,
    pub total_score: i32,
    pub avg_score: f32,
    pub num_1s: i32,
    pub num_2s: i32,
    pub num_3s: i32,
    pub num_4s: i32,
    pub num_5s: i32,
    pub num_6s: i32,
    pub num_xs: i32,
}

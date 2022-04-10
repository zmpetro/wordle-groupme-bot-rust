table! {
    all_time_stats (user_id) {
        user_id -> Text,
        games_played -> Integer,
        total_score -> Integer,
        avg_score -> Float,
        num_1s -> Integer,
        num_2s -> Integer,
        num_3s -> Integer,
        num_4s -> Integer,
        num_5s -> Integer,
        num_6s -> Integer,
        num_xs -> Integer,
    }
}

table! {
    daily_stats (user_id) {
        user_id -> Text,
        score -> Integer,
    }
}

table! {
    game_number (game) {
        game -> Integer,
    }
}

table! {
    names (user_id) {
        user_id -> Text,
        name -> Text,
    }
}

table! {
    player_ratings (user_id) {
        user_id -> Text,
        mu -> Float,
        sigma -> Float,
    }
}

table! {
    week_number (week) {
        week -> Integer,
    }
}

table! {
    weekly_stats (user_id) {
        user_id -> Text,
        games_played -> Integer,
        total_score -> Integer,
        avg_score -> Float,
        num_1s -> Integer,
        num_2s -> Integer,
        num_3s -> Integer,
        num_4s -> Integer,
        num_5s -> Integer,
        num_6s -> Integer,
        num_xs -> Integer,
    }
}

allow_tables_to_appear_in_same_query!(
    all_time_stats,
    daily_stats,
    game_number,
    names,
    player_ratings,
    week_number,
    weekly_stats,
);

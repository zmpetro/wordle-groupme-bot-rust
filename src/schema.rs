table! {
    ALL_TIME_STATS (PLAYER_ID) {
        PLAYER_ID -> Text,
        GAMES_PLAYED -> Nullable<Integer>,
        TOTAL_SCORE -> Nullable<Integer>,
        AVERAGE_SCORE -> Nullable<Float>,
        NUM_1S -> Nullable<Integer>,
        NUM_2S -> Nullable<Integer>,
        NUM_3S -> Nullable<Integer>,
        NUM_4S -> Nullable<Integer>,
        NUM_5S -> Nullable<Integer>,
        NUM_6S -> Nullable<Integer>,
        NUM_XS -> Nullable<Integer>,
    }
}

table! {
    DAILY_STATS (PLAYER_ID) {
        PLAYER_ID -> Text,
        SCORE -> Nullable<Integer>,
    }
}

table! {
    GAME_NUMBER (GAME) {
        GAME -> Integer,
    }
}

table! {
    NAMES (PLAYER_ID) {
        PLAYER_ID -> Text,
        NAME -> Text,
    }
}

table! {
    PLAYER_RATINGS (PLAYER_ID) {
        PLAYER_ID -> Text,
        MU -> Nullable<Float>,
        SIGMA -> Nullable<Float>,
    }
}

table! {
    WEEKLY_STATS (PLAYER_ID) {
        PLAYER_ID -> Text,
        GAMES_PLAYED -> Nullable<Integer>,
        TOTAL_SCORE -> Nullable<Integer>,
        AVERAGE_SCORE -> Nullable<Float>,
        NUM_1S -> Nullable<Integer>,
        NUM_2S -> Nullable<Integer>,
        NUM_3S -> Nullable<Integer>,
        NUM_4S -> Nullable<Integer>,
        NUM_5S -> Nullable<Integer>,
        NUM_6S -> Nullable<Integer>,
        NUM_XS -> Nullable<Integer>,
    }
}

table! {
    WEEK_NUMBER (WEEK) {
        WEEK -> Integer,
    }
}

allow_tables_to_appear_in_same_query!(
    ALL_TIME_STATS,
    DAILY_STATS,
    GAME_NUMBER,
    NAMES,
    PLAYER_RATINGS,
    WEEKLY_STATS,
    WEEK_NUMBER,
);

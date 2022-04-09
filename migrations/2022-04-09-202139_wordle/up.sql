-- Your SQL goes here
CREATE TABLE daily_stats
(player_id TEXT PRIMARY KEY NOT NULL,
score INT NOT NULL);

CREATE TABLE weekly_stats
(player_id TEXT PRIMARY KEY NOT NULL,
games_played INT NOT NULL,
total_score INT NOT NULL,
avg_score REAL NOT NULL,
num_1s INT NOT NULL,
num_2s INT NOT NULL,
num_3s INT NOT NULL,
num_4s INT NOT NULL,
num_5s INT NOT NULL,
num_6s INT NOT NULL,
num_xs INT NOT NULL);

CREATE TABLE all_time_stats
(player_id TEXT PRIMARY KEY NOT NULL,
games_played INT NOT NULL,
total_score INT NOT NULL,
avg_score REAL NOT NULL,
num_1s INT NOT NULL,
num_2s INT NOT NULL,
num_3s INT NOT NULL,
num_4s INT NOT NULL,
num_5s INT NOT NULL,
num_6s INT NOT NULL,
num_xs INT NOT NULL);

CREATE TABLE names
(player_id TEXT PRIMARY KEY NOT NULL,
name TEXT NOT NULL);

CREATE TABLE game_number
(game INT PRIMARY KEY NOT NULL);

CREATE TABLE week_number
(week INT PRIMARY KEY NOT NULL);

CREATE TABLE player_ratings
(player_id TEXT PRIMARY KEY NOT NULL,
mu REAL NOT NULL,
sigma REAL NOT NULL);

INSERT INTO game_number VALUES (0);

INSERT INTO week_number VALUES (0);
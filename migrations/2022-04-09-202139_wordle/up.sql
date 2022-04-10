-- Your SQL goes here
CREATE TABLE daily_stats
(user_id TEXT PRIMARY KEY NOT NULL,
score INT NOT NULL);

CREATE TABLE weekly_stats
(user_id TEXT PRIMARY KEY NOT NULL,
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
(user_id TEXT PRIMARY KEY NOT NULL,
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
(user_id TEXT PRIMARY KEY NOT NULL,
name TEXT NOT NULL);

CREATE TABLE game_number
(game INT PRIMARY KEY NOT NULL);

CREATE TABLE week_number
(week INT PRIMARY KEY NOT NULL);

CREATE TABLE player_ratings
(user_id TEXT PRIMARY KEY NOT NULL,
mu REAL NOT NULL,
sigma REAL NOT NULL);

INSERT INTO game_number VALUES (0);

INSERT INTO week_number VALUES (0);




INSERT INTO all_time_stats VALUES (1,1,1,1.1,1,1,1,1,1,1,1);
INSERT INTO all_time_stats VALUES (3,3,3,3.3,3,3,3,3,3,3,3);
INSERT INTO all_time_stats VALUES (5,5,5,5.392845849,5,5,5,5,5,5,5);
INSERT INTO all_time_stats VALUES (4,4,4,4.4,4,4,4,4,4,4,4);
INSERT INTO all_time_stats VALUES (2,2,2,2.2,2,2,2,2,2,2,2);
INSERT INTO names VALUES (1,"Johnson");
INSERT INTO names VALUES (2,"Blevins");
INSERT INTO names VALUES (3,"Gaspack");
INSERT INTO names VALUES (4,"Stod");
INSERT INTO names VALUES (5,"Frail");
INSERT INTO weekly_stats VALUES (1,9,9,9.32423423,9,9,9,9,9,9,9);
INSERT INTO weekly_stats VALUES (3,3,3,3.3,3,3,3,3,3,3,3);
INSERT INTO weekly_stats VALUES (5,5,5,5.392845849,5,5,5,5,5,5,5);
INSERT INTO weekly_stats VALUES (4,4,4,4.4,4,4,4,4,4,4,4);
INSERT INTO weekly_stats VALUES (2,2,2,2.2,2,2,2,2,2,2,2);
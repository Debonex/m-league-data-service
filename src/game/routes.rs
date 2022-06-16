use super::{Game, GamePro, HistoryResult};
use crate::common::format_sql_vec;
use rocket::{
    serde::{json::Json, Deserialize},
    State,
};
use sqlx::{Pool, Row, Sqlite};

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct HistoryProProParams {
    pro_id: i64,
    pro_id2: i64,
    seasons: Option<Vec<i64>>,
}

#[post("/history/pro_pro", format = "json", data = "<params>")]
pub async fn history_pro_pro(
    pool: &State<Pool<Sqlite>>,
    params: Json<HistoryProProParams>,
) -> Json<HistoryResult> {
    if params.pro_id == params.pro_id2 {
        return Json(HistoryResult {
            point: 0.0,
            games: vec![],
        });
    }

    let where_seasons = where_seasons_sql(&params.seasons);

    let games_sql = "
    SELECT
        *,
        ( CASE WHEN pid_0 = {pro_id} THEN pp_0 WHEN pid_1 = {pro_id} THEN pp_1 WHEN pid_2 = {pro_id} THEN pp_2 ELSE pp_3 END ) AS pro_point,
        ( CASE WHEN pid_0 = {pro_id2} THEN pp_0 WHEN pid_1 = {pro_id2} THEN pp_1 WHEN pid_2 = {pro_id2} THEN pp_2 ELSE pp_3 END ) AS pro_point2,
        season.season_name as season_name,
        pro_0.pro_name as pro_name_0,
        pro_1.pro_name as pro_name_1,
        pro_2.pro_name as pro_name_2,
        pro_3.pro_name as pro_name_3 
    FROM
        game
        LEFT JOIN season on season_id = season.id
        LEFT JOIN pro as pro_0 on pid_0 = pro_0.id
        LEFT JOIN pro as pro_1 on pid_1 = pro_1.id
        LEFT JOIN pro as pro_2 on pid_2 = pro_2.id
        LEFT JOIN pro as pro_3 on pid_3 = pro_3.id
    WHERE
        {where_seasons}
        ( pid_0 = {pro_id} OR pid_1 = {pro_id} OR pid_2 = {pro_id} OR pid_3 = {pro_id} ) 
    AND ( pid_0 = {pro_id2} OR pid_1 = {pro_id2} OR pid_2 = {pro_id2} OR pid_3 = {pro_id2} )";

    let games_sql = games_sql
        .replace("{pro_id}", &params.pro_id.to_string())
        .replace("{pro_id2}", &params.pro_id2.to_string())
        .replace("{where_seasons}", &where_seasons);
    let (point, games) = get_point_games(pool, &games_sql, "pro_point", "pro_point2").await;

    Json(HistoryResult { point, games })
}

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct HistoryProTeamParams {
    pro_id: i64,
    team_id: i64,
    seasons: Option<Vec<i64>>,
}

#[post("/history/pro_team", format = "json", data = "<params>")]
pub async fn history_pro_team(
    pool: &State<Pool<Sqlite>>,
    params: Json<HistoryProTeamParams>,
) -> Json<HistoryResult> {
    let where_seasons = where_seasons_sql(&params.seasons);

    let games_sql = "
    SELECT
        * ,
        ( CASE WHEN pid_0 = {pro_id} THEN pp_0 WHEN pid_1 = {pro_id} THEN pp_1 WHEN pid_2 = {pro_id} THEN pp_2 ELSE pp_3 END ) AS pro_point,
        ( CASE WHEN team_id_0 = {team_id} THEN pp_0 WHEN team_id_1 = {team_id} THEN pp_1 WHEN team_id_2 = {team_id} THEN pp_2 ELSE pp_3 END ) AS team_point,
        season.season_name as season_name,
        pro_0.pro_name as pro_name_0,
        pro_1.pro_name as pro_name_1,
        pro_2.pro_name as pro_name_2,
        pro_3.pro_name as pro_name_3 
    FROM
	    game 
        LEFT JOIN season on season_id = season.id
        LEFT JOIN pro as pro_0 on pid_0 = pro_0.id
        LEFT JOIN pro as pro_1 on pid_1 = pro_1.id
        LEFT JOIN pro as pro_2 on pid_2 = pro_2.id
        LEFT JOIN pro as pro_3 on pid_3 = pro_3.id
    WHERE
        {where_seasons}
	    ( pid_0 = {pro_id} OR pid_1 = {pro_id} OR pid_2 = {pro_id} OR pid_3 = {pro_id} ) 
	AND (
        ( CASE WHEN pid_0 = {pro_id} THEN - 1 ELSE team_id_0 END ) = {team_id} 
    OR ( CASE WHEN pid_1 = {pro_id} THEN - 1 ELSE team_id_1 END ) = {team_id} 
    OR ( CASE WHEN pid_2 = {pro_id} THEN - 1 ELSE team_id_2 END ) = {team_id} 
    OR ( CASE WHEN pid_3 = {pro_id} THEN - 1 ELSE team_id_3 END ) = {team_id})";

    let games_sql = games_sql
        .replace("{pro_id}", &params.pro_id.to_string())
        .replace("{team_id}", &params.team_id.to_string())
        .replace("{where_seasons}", &where_seasons);
    let (point, games) = get_point_games(pool, &games_sql, "pro_point", "team_point").await;

    Json(HistoryResult { point, games })
}

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct HistoryTeamTeamParams {
    team_id: i64,
    team_id2: i64,
    seasons: Option<Vec<i64>>,
}

#[post("/history/team_team", format = "json", data = "<params>")]
pub async fn history_team_team(
    pool: &State<Pool<Sqlite>>,
    params: Json<HistoryTeamTeamParams>,
) -> Json<HistoryResult> {
    if params.team_id == params.team_id2 {
        return Json(HistoryResult {
            point: 0.0,
            games: vec![],
        });
    }

    let where_seasons = where_seasons_sql(&params.seasons);

    let games_sql = "
    SELECT
        *,
        ( CASE WHEN team_id_0 = {team_id} THEN pp_0 WHEN team_id_1 = {team_id} THEN pp_1 WHEN team_id_2 = {team_id} THEN pp_2 ELSE pp_3 END ) AS team_point,
        ( CASE WHEN team_id_0 = {team_id2} THEN pp_0 WHEN team_id_1 = {team_id2} THEN pp_1 WHEN team_id_2 = {team_id2} THEN pp_2 ELSE pp_3 END ) AS team_point2,
        season.season_name as season_name,
        pro_0.pro_name as pro_name_0,
        pro_1.pro_name as pro_name_1,
        pro_2.pro_name as pro_name_2,
        pro_3.pro_name as pro_name_3 
    FROM
        game
        LEFT JOIN season on season_id = season.id
        LEFT JOIN pro as pro_0 on pid_0 = pro_0.id
        LEFT JOIN pro as pro_1 on pid_1 = pro_1.id
        LEFT JOIN pro as pro_2 on pid_2 = pro_2.id
        LEFT JOIN pro as pro_3 on pid_3 = pro_3.id
    WHERE
        {where_seasons}
        ( team_id_0 = {team_id} OR team_id_1 = {team_id} OR team_id_2 = {team_id} OR team_id_3 = {team_id} ) 
    AND ( team_id_0 = {team_id2} OR team_id_1 = {team_id2} OR team_id_2 = {team_id2} OR team_id_3 = {team_id2} )";

    let games_sql = games_sql
        .replace("{team_id}", &params.team_id.to_string())
        .replace("{team_id2}", &params.team_id2.to_string())
        .replace("{where_seasons}", &where_seasons);
    let (point, games) = get_point_games(pool, &games_sql, "team_point", "team_point2").await;

    Json(HistoryResult { point, games })
}

fn where_seasons_sql(seasons: &Option<Vec<i64>>) -> String {
    match seasons {
        Some(seasons) => format!("season_id in {} AND", format_sql_vec(seasons)),
        None => String::new(),
    }
}

async fn get_point_games(
    pool: &State<Pool<Sqlite>>,
    games_sql: &str,
    point_field: &str,
    point_field2: &str,
) -> (f32, Vec<Game>) {
    let mut point = 0.0;
    let games = sqlx::query(games_sql)
        .fetch_all(pool.inner())
        .await
        .unwrap_or_default()
        .iter()
        .map(|row| {
            point += row.try_get::<f32, &str>(point_field).unwrap_or_default()
                - row.try_get::<f32, &str>(point_field2).unwrap_or_default();
            Game {
                id: row.try_get("id").unwrap_or_default(),
                season_id: row.try_get("season_id").unwrap_or_default(),
                season_name: row.try_get("season_name").unwrap_or_default(),
                time: row.try_get("time").unwrap_or_default(),
                pros: vec![0, 1, 2, 3]
                    .iter()
                    .map(|idx| GamePro {
                        id: row.try_get(&*format!("pid_{}", idx)).unwrap_or_default(),
                        team_id: row
                            .try_get(&*format!("team_id_{}", idx))
                            .unwrap_or_default(),
                        point: row.try_get(&*format!("pp_{}", idx)).unwrap_or_default(),
                        pro_name: row
                            .try_get(&*format!("pro_name_{}", idx))
                            .unwrap_or_default(),
                    })
                    .collect(),
            }
        })
        .collect();

    point = (point * 10.0).round() / 10.0;

    (point, games)
}

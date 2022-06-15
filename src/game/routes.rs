use crate::common::format_sql_vec;

use super::{Game, GamePro, HistoryResult};
use rocket::{
    serde::{json::Json, Deserialize},
    tokio::join,
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

    let where_seasons = match &params.seasons {
        Some(seasons) => format!("season_id in {} AND", format_sql_vec(seasons)),
        None => String::new(),
    };

    let point_sql = "SELECT
    ( CASE WHEN pid_0 = p1 THEN pp_0 WHEN pid_1 = p1 THEN pp_1 WHEN pid_2 = p1 THEN pp_2 ELSE pp_3 END ) AS pp_1,
    ( CASE WHEN pid_0 = p2 THEN pp_0 WHEN pid_1 = p2 THEN pp_1 WHEN pid_2 = p2 THEN pp_2 ELSE pp_3 END ) AS pp_2
  FROM
    game
  WHERE
    where_seasons
    ( pid_0 = p1 OR pid_1 = p1 OR pid_2 = p1 OR pid_3 = p1 ) 
    AND ( pid_0 = p2 OR pid_1 = p2 OR pid_2 = p2 OR pid_3 = p2 )"
    .replace("p1", &params.pro_id.to_string()).replace("p2", &params.pro_id2.to_string()).replace("where_seasons",&where_seasons);

    let games_sql = format!(
        "SELECT *, {} AS p1, {} AS p2, 
        pro_0.pro_name as pro_name_0,
        pro_0.team_id as team_id_0,
        pro_1.pro_name as pro_name_1,
        pro_1.team_id as team_id_1,
        pro_2.pro_name as pro_name_2,
        pro_2.team_id as team_id_2,
        pro_3.pro_name as pro_name_3,
        pro_3.team_id as team_id_3
        FROM game
        LEFT JOIN pro as pro_0 on pid_0 = pro_0.id
        LEFT JOIN pro as pro_1 on pid_1 = pro_1.id
        LEFT JOIN pro as pro_2 on pid_2 = pro_2.id
        LEFT JOIN pro as pro_3 on pid_3 = pro_3.id
        WHERE
        {}
        ( pid_0 = p1 OR pid_1 = p1 OR pid_2 = p1 OR pid_3 = p1 ) 
        AND ( pid_0 = p2 OR pid_1 = p2 OR pid_2 = p2 OR pid_3 = p2 )
    ",
        &params.pro_id, &params.pro_id2, &where_seasons,
    );

    let (point_rows, games_rows) = join!(
        sqlx::query(&point_sql).fetch_all(pool.inner()),
        sqlx::query(&games_sql).fetch_all(pool.inner())
    );

    let mut point = 0.0;
    point_rows.unwrap_or_default().iter().for_each(|row| {
        let pp1 = row.try_get::<f32, &str>("pp_1").unwrap_or_default();
        let pp2 = row.try_get::<f32, &str>("pp_2").unwrap_or_default();
        point += pp1 - pp2;
    });
    point = (point * 10.0).round() / 10.0;

    let games = games_rows
        .unwrap_or_default()
        .iter()
        .map(|row| Game {
            id: row.try_get("id").unwrap_or_default(),
            season_id: row.try_get("season_id").unwrap_or_default(),
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
        })
        .collect();

    Json(HistoryResult { point, games })
}

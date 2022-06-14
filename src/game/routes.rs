use super::{Game, GamePro, HistoryResult};
use rocket::{serde::json::Json, tokio::join, State};
use sqlx::{Pool, Row, Sqlite};

#[get("/history?<pro_id>&<pro_id2>")]
pub async fn history_pro_pro(
    pool: &State<Pool<Sqlite>>,
    pro_id: i64,
    pro_id2: i64,
) -> Json<HistoryResult> {
    let point_sql = "SELECT
    ( CASE WHEN pid_0 = p1 THEN pp_0 WHEN pid_1 = p1 THEN pp_1 WHEN pid_2 = p1 THEN pp_2 ELSE pp_3 END ) AS pp_1,
    ( CASE WHEN pid_0 = p2 THEN pp_0 WHEN pid_1 = p2 THEN pp_1 WHEN pid_2 = p2 THEN pp_2 ELSE pp_3 END ) AS pp_2
  FROM
    game
  WHERE
    ( pid_0 = p1 OR pid_1 = p1 OR pid_2 = p1 OR pid_3 = p1 ) 
    AND ( pid_0 = p2 OR pid_1 = p2 OR pid_2 = p2 OR pid_3 = p2 )".replace("p1", &pro_id.to_string()).replace("p2", &pro_id2.to_string());

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
        ( pid_0 = p1 OR pid_1 = p1 OR pid_2 = p1 OR pid_3 = p1 ) 
        AND ( pid_0 = p2 OR pid_1 = p2 OR pid_2 = p2 OR pid_3 = p2 )
    ",
        pro_id, pro_id2
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
            pros: vec![
                GamePro {
                    id: row.try_get("pid_0").unwrap_or_default(),
                    team_id: row.try_get("team_id_0").unwrap_or_default(),
                    point: row.try_get("pp_0").unwrap_or_default(),
                    pro_name: row.try_get("pro_name_0").unwrap_or_default(),
                },
                GamePro {
                    id: row.try_get("pid_1").unwrap_or_default(),
                    team_id: row.try_get("team_id_1").unwrap_or_default(),
                    point: row.try_get("pp_1").unwrap_or_default(),
                    pro_name: row.try_get("pro_name_1").unwrap_or_default(),
                },
                GamePro {
                    id: row.try_get("pid_2").unwrap_or_default(),
                    team_id: row.try_get("team_id_2").unwrap_or_default(),
                    point: row.try_get("pp_2").unwrap_or_default(),
                    pro_name: row.try_get("pro_name_2").unwrap_or_default(),
                },
                GamePro {
                    id: row.try_get("pid_3").unwrap_or_default(),
                    team_id: row.try_get("team_id_3").unwrap_or_default(),
                    point: row.try_get("pp_3").unwrap_or_default(),
                    pro_name: row.try_get("pro_name_3").unwrap_or_default(),
                },
            ],
        })
        .collect();

    Json(HistoryResult { point, games })
}

use super::{Team, TeamValueItem};
use crate::common::{
    format_sql_vec,
    ranks::{self, Value},
    season_pro,
    statistics::{self, Statistic},
};
use rocket::{
    serde::{json::Json, Deserialize},
    State,
};
use sqlx::{Pool, Row, Sqlite};

#[get("/all")]
pub async fn all(pool: &State<Pool<Sqlite>>) -> Json<Vec<Team>> {
    let teams = sqlx::query_as!(Team, "select * from team")
        .fetch_all(pool.inner())
        .await
        .unwrap_or_default();

    Json(teams)
}

#[get("/info/<team_id>")]
pub async fn info(pool: &State<Pool<Sqlite>>, team_id: i64) -> Json<Team> {
    let team = sqlx::query_as!(Team, "select * from team where id = ?", team_id)
        .fetch_one(pool.inner())
        .await
        .unwrap_or_default();

    Json(team)
}

#[get("/info_by_pro_id/<pro_id>")]
pub async fn info_by_pro_id(pool: &State<Pool<Sqlite>>, pro_id: i64) -> Json<Team> {
    let team = sqlx::query_as!(
        Team,
        "select team.id as id, team_name, team_code from team left join pro on team.id = pro.team_id where pro.id = ?",
        pro_id
    )
    .fetch_one(pool.inner())
    .await
    .unwrap_or_default();

    Json(team)
}

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct TeamStatisticParams {
    team_id: i64,
    seasons: Option<Vec<i64>>,
}

#[post("/statistic", format = "json", data = "<params>")]
pub async fn statistic(
    pool: &State<Pool<Sqlite>>,
    params: Json<TeamStatisticParams>,
) -> Json<Statistic> {
    let sp_list =
        season_pro::select_season_pro_by_team_id(pool, params.team_id, &params.seasons).await;
    Json(statistics::statistics(sp_list))
}

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct TeamRankParams {
    key: String,
    seasons: Option<Vec<i64>>,
}

#[post("/rank", format = "json", data = "<params>")]
pub async fn rank(
    pool: &State<Pool<Sqlite>>,
    params: Json<TeamRankParams>,
) -> Json<Vec<TeamValueItem>> {
    Json(ranks::rank(pool, &params.key, &params.seasons, value_list, divided_list).await)
}

async fn value_list(
    pool: &State<Pool<Sqlite>>,
    value_sql: &str,
    seasons: &Option<Vec<i64>>,
    is_float: bool,
) -> Vec<TeamValueItem> {
    let where_seasons = match seasons {
        Some(seasons) => format!("AND season_pro.season_id IN {}", format_sql_vec(seasons)),
        _ => String::new(),
    };

    let sql_string = format!(
        "SELECT season_pro.team_id, {} AS value, team.team_name
        FROM season_pro, team 
        WHERE season_pro.team_id = team.id
        {}
        GROUP BY team_id 
        ORDER BY value DESC",
        &value_sql, where_seasons
    );

    sqlx::query(&sql_string)
        .fetch_all(pool.inner())
        .await
        .unwrap_or_default()
        .iter()
        .map(|row| TeamValueItem {
            team_name: row.try_get("team_name").unwrap_or_default(),
            team_id: row.try_get("team_id").unwrap_or_default(),
            value: {
                match is_float {
                    true => Value::Float(row.try_get("value").unwrap_or_default()),
                    false => Value::Integer(row.try_get("value").unwrap_or_default()),
                }
            },
        })
        .collect()
}

async fn divided_list(
    pool: &State<Pool<Sqlite>>,
    value_sql: &str,
    value_sql2: &str,
    seasons: &Option<Vec<i64>>,
    is_float: bool,
) -> Vec<TeamValueItem> {
    let where_seasons = match seasons {
        Some(seasons) => format!("AND season_pro.season_id IN {}", format_sql_vec(seasons)),
        _ => String::new(),
    };
    let sql_string = format!(
        "SELECT season_pro.team_id, {} AS value, {} AS value2, team.team_name
        FROM season_pro, team 
        WHERE season_pro.team_id = team.id
        {}
        GROUP BY team_id 
        ORDER BY value DESC",
        &value_sql, &&value_sql2, where_seasons
    );

    let mut list: Vec<TeamValueItem> = sqlx::query(&sql_string)
        .fetch_all(pool.inner())
        .await
        .unwrap_or_default()
        .iter()
        .map(|row| {
            let value2: i64 = row.try_get("value2").unwrap_or_default();
            TeamValueItem {
                team_name: row.try_get("team_name").unwrap_or_default(),
                team_id: row.try_get("team_id").unwrap_or_default(),
                value: {
                    if value2 == 0 {
                        Value::Float(0.0)
                    } else {
                        match is_float {
                            true => {
                                let value: f32 = row.try_get("value").unwrap_or_default();
                                Value::Float(value / value2 as f32)
                            }
                            false => {
                                let value: i64 = row.try_get("value").unwrap_or_default();
                                Value::Float(value as f32 / value2 as f32)
                            }
                        }
                    }
                },
            }
        })
        .collect();

    list.sort_unstable_by(|a, b| b.value.partial_cmp(&a.value).unwrap());
    list
}

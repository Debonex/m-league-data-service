use super::{Pro, ProValueItem};
use crate::common::{
    format_sql_vec, ranks,
    ranks::Value,
    season_pro::select_season_pro_by_pro_id,
    statistics::{statistics, Statistic},
};
use rocket::{
    serde::{json::Json, Deserialize},
    State,
};
use sqlx::{Pool, Row, Sqlite};

#[get("/all")]
pub async fn all(pool: &State<Pool<Sqlite>>) -> Json<Vec<Pro>> {
    let pros = sqlx::query_as!(Pro, "select * from pro")
        .fetch_all(pool.inner())
        .await
        .unwrap_or_default();

    Json(pros)
}

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct ProStatisticParams {
    id: i64,
    seasons: Option<Vec<i64>>,
}

#[post("/statistic", format = "json", data = "<params>")]
pub async fn statistic(
    pool: &State<Pool<Sqlite>>,
    params: Json<ProStatisticParams>,
) -> Json<Statistic> {
    let sp_list = select_season_pro_by_pro_id(pool, params.id, &params.seasons).await;
    Json(statistics(sp_list))
}

#[get("/info/<id>")]
pub async fn info(pool: &State<Pool<Sqlite>>, id: i64) -> Json<Pro> {
    let pro = sqlx::query_as!(Pro, "SELECT * FROM pro WHERE id = ?", id)
        .fetch_one(pool.inner())
        .await
        .unwrap_or_default();

    Json(pro)
}

#[get("/list_by_team_id/<team_id>")]
pub async fn list_by_team_id(pool: &State<Pool<Sqlite>>, team_id: i64) -> Json<Vec<Pro>> {
    Json(super::services::list_by_team_id(pool, team_id).await)
}

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct ProRankParams {
    key: String,
    seasons: Option<Vec<i64>>,
}

#[post("/rank", format = "json", data = "<params>")]
pub async fn rank(
    pool: &State<Pool<Sqlite>>,
    params: Json<ProRankParams>,
) -> Json<Vec<ProValueItem>> {
    Json(ranks::rank(pool, &params.key, &params.seasons, value_list, divided_list).await)
}

async fn value_list(
    pool: &State<Pool<Sqlite>>,
    value_sql: &str,
    seasons: &Option<Vec<i64>>,
    is_float: bool,
) -> Vec<ProValueItem> {
    let where_seasons = match seasons {
        Some(seasons) => format!("AND season_pro.season_id IN {}", format_sql_vec(seasons)),
        _ => String::new(),
    };

    let sql_string = format!(
        "SELECT season_pro.pro_id, {} AS value, pro.pro_name, pro.team_id 
        FROM season_pro, pro 
        WHERE season_pro.pro_id = pro.id
        {}
        GROUP BY pro_id 
        ORDER BY value DESC",
        &value_sql, where_seasons
    );

    sqlx::query(&sql_string)
        .fetch_all(pool.inner())
        .await
        .unwrap_or_default()
        .iter()
        .map(|row| ProValueItem {
            pro_id: row.try_get("pro_id").unwrap_or_default(),
            pro_name: row.try_get("pro_name").unwrap_or_default(),
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
) -> Vec<ProValueItem> {
    let where_seasons = match seasons {
        Some(seasons) => format!("AND season_pro.season_id IN {}", format_sql_vec(seasons)),
        _ => String::new(),
    };
    let sql_string = format!(
        "SELECT season_pro.pro_id, {} AS value, {} AS value2, pro.pro_name, pro.team_id 
        FROM season_pro, pro 
        WHERE season_pro.pro_id = pro.id
        {}
        GROUP BY pro_id 
        ORDER BY value DESC",
        &value_sql, &&value_sql2, where_seasons
    );

    let mut list: Vec<ProValueItem> = sqlx::query(&sql_string)
        .fetch_all(pool.inner())
        .await
        .unwrap_or_default()
        .iter()
        .map(|row| {
            let value2: i64 = row.try_get("value2").unwrap_or_default();
            ProValueItem {
                pro_id: row.try_get("pro_id").unwrap_or_default(),
                pro_name: row.try_get("pro_name").unwrap_or_default(),
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

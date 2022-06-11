use super::{Pro, ProValueItem, Value};
use crate::common::{
    format_sql_vec,
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
    let pro = sqlx::query_as!(Pro, "SELECT * FROM pro WHERE team_id = ?", team_id)
        .fetch_all(pool.inner())
        .await
        .unwrap_or_default();

    Json(pro)
}

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct ProRankParams {
    key: String,
    seasons: Option<Vec<i64>>,
}

const POINT: &str = "sum(score_point + rank_point)";
const KYOKU_NUM: &str = "sum(kyoku_east_num + kyoku_north_num + kyoku_west_num + kyoku_north_num)";
const AGARI_NUM: &str = "sum(agari_dama_ron_num + agari_dama_tsumo_num + 
    agari_furo_ron_num + agari_furo_tsumo_num + 
    agari_richi_ron_num + agari_richi_tsumo_num)";
const RICHI_NUM: &str = "sum(richi_num)";
const HOUJUU_NUM: &str =
    "sum(houjuu_dama_furo_num + houjuu_dama_menzen_num + houjuu_dama_richi_num 
    + houjuu_furo_furo_num + houjuu_furo_menzen_num + houjuu_furo_richi_num
    + houjuu_richi_furo_num + houjuu_richi_menzen_num + houjuu_richi_richi_num)";
const TSUMO_NUM: &str = "sum(agari_dama_tsumo_num + agari_furo_tsumo_num + agari_richi_tsumo_num)";
const RYUKYOKU_NUM: &str =
    "sum(ryukyoku_tenpai_menzen_num + ryukyoku_tenpai_furo_num + ryukyoku_tenpai_richi_num +
    ryukyoku_noten_menzen_num + ryukyoku_noten_furo_num + ryukyoku_noten_richi_num)";
const AGARI_RICHI_NUM: &str = "sum(agari_richi_ron_num + agari_richi_tsumo_num)";
const HOUJUU_FURO_NUM: &str =
    "sum(houjuu_dama_furo_num + houjuu_furo_furo_num + houjuu_richi_furo_num)";
const HOUJUU_RICHI_NUM: &str =
    "sum(houjuu_dama_richi_num + houjuu_furo_richi_num + houjuu_richi_richi_num)";

#[post("/rank", format = "json", data = "<params>")]
pub async fn rank(
    pool: &State<Pool<Sqlite>>,
    params: Json<ProRankParams>,
) -> Json<Vec<ProValueItem>> {
    let key: &str = &params.key;
    let seasons = &params.seasons;

    let rank_list = match key {
        "rank_point" | "score_point" => {
            value_list(pool, format!("sum({})", key).as_str(), seasons, true).await
        }
        "game_num" => value_list(pool, "sum(game_num)", seasons, false).await,
        "point" => {
            let mut list = value_list(pool, POINT, seasons, true).await;
            // result calculated by sqlite engine is not precise, to fixed 1.
            for item in &mut list {
                let value = item.value.float_value().unwrap_or_default();
                item.value = Value::Float((value * 10.0).round() / 10.0);
            }
            list
        }
        "kyoku_num" => value_list(pool, KYOKU_NUM, seasons, false).await,
        "avg_point" => divided_list(pool, POINT, "sum(game_num)", seasons, true).await,
        "agari_rate" => divided_list(pool, AGARI_NUM, KYOKU_NUM, seasons, false).await,
        "houjuu_rate" => divided_list(pool, HOUJUU_NUM, KYOKU_NUM, seasons, false).await,
        "houjuu_menzen_rate" => {
            let houjuu_menzen_num =
                "sum(houjuu_dama_menzen_num + houjuu_furo_menzen_num + houjuu_richi_menzen_num)";
            divided_list(pool, houjuu_menzen_num, HOUJUU_NUM, seasons, false).await
        }
        "houjuu_furo_rate" => divided_list(pool, HOUJUU_FURO_NUM, HOUJUU_NUM, seasons, false).await,
        "houjuu_richi_rate" => {
            divided_list(pool, HOUJUU_RICHI_NUM, HOUJUU_NUM, seasons, false).await
        }
        "houjuu_to_dama_rate" => {
            let houjuu_to_dama_num =
                "sum(houjuu_dama_menzen_num + houjuu_dama_furo_num + houjuu_dama_richi_num)";
            divided_list(pool, houjuu_to_dama_num, HOUJUU_NUM, seasons, false).await
        }
        "houjuu_to_furo_rate" => {
            let houjuu_to_furo_num =
                "sum(houjuu_furo_menzen_num + houjuu_furo_furo_num + houjuu_furo_richi_num)";
            divided_list(pool, houjuu_to_furo_num, HOUJUU_NUM, seasons, false).await
        }
        "houjuu_to_richi_rate" => {
            let houjuu_to_richi_num =
                "sum(houjuu_richi_menzen_num + houjuu_richi_furo_num + houjuu_richi_richi_num)";
            divided_list(pool, houjuu_to_richi_num, HOUJUU_NUM, seasons, false).await
        }
        "tsumo_rate" => divided_list(pool, TSUMO_NUM, AGARI_NUM, seasons, false).await,
        "agari_dama_rate" => {
            let agari_dama_num = "sum(agari_dama_ron_num + agari_dama_tsumo_num)";
            divided_list(pool, agari_dama_num, AGARI_NUM, seasons, false).await
        }
        "agari_furo_rate" => {
            let agari_furo_num = "sum(agari_furo_ron_num + agari_furo_tsumo_num)";
            divided_list(pool, agari_furo_num, AGARI_NUM, seasons, false).await
        }
        "agari_richi_rate" => {
            let agari_richi_num = "sum(agari_richi_ron_num + agari_richi_tsumo_num)";
            divided_list(pool, agari_richi_num, AGARI_NUM, seasons, false).await
        }
        "ryukyoku_rate" => divided_list(pool, RYUKYOKU_NUM, KYOKU_NUM, seasons, false).await,
        "ryukyoku_tenpai_rate" => {
            let ryu_ten_num = "sum(ryukyoku_tenpai_menzen_num + ryukyoku_tenpai_furo_num + ryukyoku_tenpai_richi_num)";
            divided_list(pool, ryu_ten_num, RYUKYOKU_NUM, seasons, false).await
        }
        "furo_rate" => divided_list(pool, "sum(furo_num)", KYOKU_NUM, seasons, false).await,
        "furo_agari_rate" => {
            let agari_furo_num = "sum(agari_furo_ron_num + agari_furo_tsumo_num)";
            divided_list(pool, agari_furo_num, "sum(furo_num)", seasons, false).await
        }
        "furo_ryukyoku_rate" => {
            let furo_ryukyoku_num = "sum(ryukyoku_noten_furo_num + ryukyoku_tenpai_furo_num)";
            divided_list(pool, furo_ryukyoku_num, "sum(furo_num)", seasons, false).await
        }
        "furo_houjuu_rate" => {
            divided_list(pool, HOUJUU_FURO_NUM, "sum(furo_num)", seasons, false).await
        }
        "avg_furo_agari_score" => {
            let agari_furo_num = "sum(agari_furo_ron_num + agari_furo_tsumo_num)";
            divided_list(
                pool,
                "sum(agari_furo_score)",
                agari_furo_num,
                seasons,
                false,
            )
            .await
        }
        "richi_rate" => divided_list(pool, RICHI_NUM, KYOKU_NUM, seasons, false).await,
        "avg_agari_turn" => {
            divided_list(pool, "sum(agari_turn_num)", AGARI_NUM, seasons, false).await
        }
        "avg_agari_score" => {
            let agari_score = "sum(agari_dama_score + agari_furo_score + agari_richi_score)";
            divided_list(pool, agari_score, AGARI_NUM, seasons, false).await
        }
        "avg_houjuu_score" => {
            let houjuu_score = "sum(houjuu_dama_score + houjuu_furo_score + houjuu_richi_score)";
            divided_list(pool, houjuu_score, HOUJUU_NUM, seasons, false).await
        }
        "avg_rank" => {
            let rank_total =
                "sum(first_east_num + first_south_num + first_west_num + first_north_num +
                (second_east_num + second_south_num + second_west_num + second_north_num) * 2 +
                (third_east_num + third_south_num + third_west_num + third_north_num) * 3 +
                (fourth_east_num + fourth_south_num + fourth_west_num + fourth_north_num) * 4
            )";
            divided_list(pool, rank_total, "sum(game_num)", seasons, false).await
        }
        "blown_rate" => {
            divided_list(
                pool,
                "sum(blown_num)",
                "sum(kyoku_east_num)",
                seasons,
                false,
            )
            .await
        }
        "avg_blown_score" => {
            divided_list(pool, "sum(blown_score)", "sum(blown_num)", seasons, false).await
        }
        "first_rate" => {
            let first_num =
                "sum(first_east_num + first_south_num + first_west_num + first_north_num)";
            divided_list(pool, first_num, "sum(game_num)", seasons, false).await
        }
        "avg_first_score" => {
            let first_num =
                "sum(first_east_num + first_south_num + first_west_num + first_north_num)";
            divided_list(pool, "sum(first_score)", first_num, seasons, false).await
        }
        "second_rate" => {
            let second_num =
                "sum(second_east_num + second_south_num + second_west_num + second_north_num)";
            divided_list(pool, second_num, "sum(game_num)", seasons, false).await
        }
        "avg_second_score" => {
            let second_num =
                "sum(second_east_num + second_south_num + second_west_num + second_north_num)";
            divided_list(pool, "sum(second_score)", second_num, seasons, false).await
        }
        "third_rate" => {
            let third_num =
                "sum(third_east_num + third_south_num + third_west_num + third_north_num)";
            divided_list(pool, third_num, "sum(game_num)", seasons, false).await
        }
        "avg_third_score" => {
            let third_num =
                "sum(third_east_num + third_south_num + third_west_num + third_north_num)";
            divided_list(pool, "sum(third_score)", third_num, seasons, false).await
        }
        "fourth_rate" => {
            let fourth_num =
                "sum(fourth_east_num + fourth_south_num + fourth_west_num + fourth_north_num)";
            divided_list(pool, fourth_num, "sum(game_num)", seasons, false).await
        }
        "avg_fourth_score" => {
            let fourth_num =
                "sum(fourth_east_num + fourth_south_num + fourth_west_num + fourth_north_num)";
            divided_list(pool, "sum(fourth_score)", fourth_num, seasons, false).await
        }
        "richi_agari_rate" => divided_list(pool, AGARI_RICHI_NUM, RICHI_NUM, seasons, false).await,
        "richi_houjuu_rate" => {
            let richi_houjuu_num =
                "sum(houjuu_dama_richi_num + houjuu_furo_richi_num + houjuu_richi_richi_num)";
            divided_list(pool, richi_houjuu_num, RICHI_NUM, seasons, false).await
        }
        "richi_tsumo_rate" => {
            divided_list(
                pool,
                "sum(agari_richi_tsumo_num)",
                AGARI_RICHI_NUM,
                seasons,
                false,
            )
            .await
        }
        "avg_richi_agari_score" => {
            divided_list(
                pool,
                "sum(agari_richi_score)",
                AGARI_RICHI_NUM,
                seasons,
                false,
            )
            .await
        }
        "richi_ryukyoku_rate" => {
            let richi_ryukyoku_num = "sum(ryukyoku_noten_richi_num + ryukyoku_tenpai_richi_num)";
            divided_list(pool, richi_ryukyoku_num, RICHI_NUM, seasons, false).await
        }
        "avg_richi_turn" => {
            divided_list(pool, "sum(richi_turn_num)", RICHI_NUM, seasons, false).await
        }
        "avg_richi_dora" => {
            divided_list(pool, "sum(richi_dora_num)", RICHI_NUM, seasons, false).await
        }
        "richi_first_rate" => {
            divided_list(pool, "sum(richi_first_num)", RICHI_NUM, seasons, false).await
        }
        "richi_chase_rate" => {
            divided_list(pool, "sum(richi_chase_num)", RICHI_NUM, seasons, false).await
        }
        "richi_chased_rate" => {
            divided_list(pool, "sum(richi_chased_num)", RICHI_NUM, seasons, false).await
        }
        "ippatsu_rate" => {
            let ippatsu_num = "sum(agari_richi_ron_ippatsu_num + agari_richi_tsumo_ippatsu_num)";
            divided_list(pool, ippatsu_num, AGARI_RICHI_NUM, seasons, false).await
        }
        "uradora_rate" => {
            let uradora_kyoku_num = "sum(total.agari_richi_ron_uradora_kyoku_num
                + total.agari_richi_tsumo_uradora_kyoku_num)";
            divided_list(pool, uradora_kyoku_num, AGARI_RICHI_NUM, seasons, false).await
        }
        "highest_score" => value_list(pool, "max(game_highest_score)", seasons, false).await,
        "lowest_score" => value_list(pool, "min(game_lowest_score)", seasons, false).await,
        "renchan_max_num" => value_list(pool, "max(renchan_max_num)", seasons, false).await,
        _ => vec![],
    };

    Json(rank_list)
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

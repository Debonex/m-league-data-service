use super::statistic as StatisticDomain;
use crate::bo::ProTupleValueItem;
use crate::bo::Value;
use crate::data::pro as ProDao;
use crate::data::season_pro as SeasonProDao;
use crate::entity::pro::Model as ProModel;
use crate::vo::ProValueItem;
use crate::vo::Statistic;
use sea_orm::DatabaseConnection;
use std::vec;

/// 获取所有选手的基本信息
pub async fn all(db: &DatabaseConnection) -> Vec<ProModel> {
    ProDao::select_all_pro(db).await
}

/// 根据id获取某个选手的基本信息
pub async fn info(db: &DatabaseConnection, id: i32) -> ProModel {
    ProDao::get_pro(db, id).await.unwrap_or(ProModel {
        id,
        birth: None,
        birth_place: None,
        org: None,
        pro_name: None,
        pro_year: None,
    })
}

/// 获取某个选手的统计数据，可根据赛季id进行筛选
pub async fn statistic(
    db: &DatabaseConnection,
    pro_id: i32,
    seasons: &Option<Vec<i32>>,
) -> Statistic {
    let season_pro_list = SeasonProDao::select_season_pro_by_pro_id(db, pro_id, seasons).await;
    StatisticDomain::statistic(season_pro_list)
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

/// 获取选手某项统计数据的排名，可根据赛季id进行筛选
pub async fn rank(
    db: &DatabaseConnection,
    key: &str,
    seasons: &Option<Vec<i32>>,
) -> Vec<ProValueItem> {
    match key {
        "rank_point" | "score_point" => {
            value_list(db, format!("sum({})", key).as_str(), seasons, true, true).await
        }
        "game_num" => value_list(db, format!("sum({})", key).as_str(), seasons, false, true).await,
        "point" => {
            let mut list = value_list(db, POINT, seasons, true, true).await;
            // result calculated by sqlite engine is not precise, to fixed 1.
            for item in &mut list {
                let value = item.value.float_value().unwrap();
                item.value = Value::Float((value * 10.0).round() / 10.0);
            }
            list
        }
        "kyoku_num" => value_list(db, KYOKU_NUM, seasons, false, true).await,
        "avg_point" => divided_list(db, POINT, "sum(game_num)", seasons, true, true).await,
        "agari_rate" => divided_list(db, AGARI_NUM, KYOKU_NUM, seasons, false, true).await,
        "houjuu_rate" => divided_list(db, HOUJUU_NUM, KYOKU_NUM, seasons, false, true).await,
        "houjuu_menzen_rate" => {
            let houjuu_menzen_num =
                "sum(houjuu_dama_menzen_num + houjuu_furo_menzen_num + houjuu_richi_menzen_num)";
            divided_list(db, houjuu_menzen_num, HOUJUU_NUM, seasons, false, true).await
        }
        "houjuu_furo_rate" => {
            divided_list(db, HOUJUU_FURO_NUM, HOUJUU_NUM, seasons, false, true).await
        }
        "houjuu_richi_rate" => {
            divided_list(db, HOUJUU_RICHI_NUM, HOUJUU_NUM, seasons, false, true).await
        }
        "houjuu_to_dama_rate" => {
            let houjuu_to_dama_num =
                "sum(houjuu_dama_menzen_num + houjuu_dama_furo_num + houjuu_dama_richi_num)";
            divided_list(db, houjuu_to_dama_num, HOUJUU_NUM, seasons, false, true).await
        }
        "houjuu_to_furo_rate" => {
            let houjuu_to_furo_num =
                "sum(houjuu_furo_menzen_num + houjuu_furo_furo_num + houjuu_furo_richi_num)";
            divided_list(db, houjuu_to_furo_num, HOUJUU_NUM, seasons, false, true).await
        }
        "houjuu_to_richi_rate" => {
            let houjuu_to_richi_num =
                "sum(houjuu_richi_menzen_num + houjuu_richi_furo_num + houjuu_richi_richi_num)";
            divided_list(db, houjuu_to_richi_num, HOUJUU_NUM, seasons, false, true).await
        }
        "tsumo_rate" => divided_list(db, TSUMO_NUM, AGARI_NUM, seasons, false, true).await,
        "agari_dama_rate" => {
            let agari_dama_num = "sum(agari_dama_ron_num + agari_dama_tsumo_num)";
            divided_list(db, agari_dama_num, AGARI_NUM, seasons, false, true).await
        }
        "agari_furo_rate" => {
            let agari_furo_num = "sum(agari_furo_ron_num + agari_furo_tsumo_num)";
            divided_list(db, agari_furo_num, AGARI_NUM, seasons, false, true).await
        }
        "agari_richi_rate" => {
            let agari_richi_num = "sum(agari_richi_ron_num + agari_richi_tsumo_num)";
            divided_list(db, agari_richi_num, AGARI_NUM, seasons, false, true).await
        }
        "ryukyoku_rate" => divided_list(db, RYUKYOKU_NUM, KYOKU_NUM, seasons, false, true).await,
        "ryukyoku_tenpai_rate" => {
            let ryu_ten_num = "sum(ryukyoku_tenpai_menzen_num + ryukyoku_tenpai_furo_num + ryukyoku_tenpai_richi_num)";
            divided_list(db, ryu_ten_num, RYUKYOKU_NUM, seasons, false, true).await
        }
        "furo_rate" => divided_list(db, "sum(furo_num)", KYOKU_NUM, seasons, false, true).await,
        "furo_agari_rate" => {
            let agari_furo_num = "sum(agari_furo_ron_num + agari_furo_tsumo_num)";
            divided_list(db, agari_furo_num, "sum(furo_num)", seasons, false, true).await
        }
        "furo_ryukyoku_rate" => {
            let furo_ryukyoku_num = "sum(ryukyoku_noten_furo_num + ryukyoku_tenpai_furo_num)";
            divided_list(db, furo_ryukyoku_num, "sum(furo_num)", seasons, false, true).await
        }
        "furo_houjuu_rate" => {
            divided_list(db, HOUJUU_FURO_NUM, "sum(furo_num)", seasons, false, true).await
        }
        "avg_furo_agari_score" => {
            let agari_furo_num = "sum(agari_furo_ron_num + agari_furo_tsumo_num)";
            divided_list(
                db,
                "sum(agari_furo_score)",
                agari_furo_num,
                seasons,
                false,
                true,
            )
            .await
        }
        "richi_rate" => divided_list(db, RICHI_NUM, KYOKU_NUM, seasons, false, true).await,
        "avg_agari_turn" => {
            divided_list(db, "sum(agari_turn_num)", AGARI_NUM, seasons, false, true).await
        }
        "avg_agari_score" => {
            let agari_score = "sum(agari_dama_score + agari_furo_score + agari_richi_score)";
            divided_list(db, agari_score, AGARI_NUM, seasons, false, true).await
        }
        "avg_houjuu_score" => {
            let houjuu_score = "sum(houjuu_dama_score + houjuu_furo_score + houjuu_richi_score)";
            divided_list(db, houjuu_score, HOUJUU_NUM, seasons, false, true).await
        }
        "avg_rank" => {
            let rank_total =
                "sum(first_east_num + first_south_num + first_west_num + first_north_num +
                (second_east_num + second_south_num + second_west_num + second_north_num) * 2 +
                (third_east_num + third_south_num + third_west_num + third_north_num) * 3 +
                (fourth_east_num + fourth_south_num + fourth_west_num + fourth_north_num) * 4
            )";
            divided_list(db, rank_total, "sum(game_num)", seasons, false, true).await
        }
        "blown_rate" => {
            divided_list(
                db,
                "sum(blown_num)",
                "sum(kyoku_east_num)",
                seasons,
                false,
                true,
            )
            .await
        }
        "avg_blown_score" => {
            divided_list(
                db,
                "sum(blown_score)",
                "sum(blown_num)",
                seasons,
                false,
                true,
            )
            .await
        }
        "first_rate" => {
            let first_num =
                "sum(first_east_num + first_south_num + first_west_num + first_north_num)";
            divided_list(db, first_num, "sum(game_num)", seasons, false, true).await
        }
        "second_rate" => {
            let second_num =
                "sum(second_east_num + second_south_num + second_west_num + second_north_num)";
            divided_list(db, second_num, "sum(game_num)", seasons, false, true).await
        }
        "third_rate" => {
            let third_num =
                "sum(third_east_num + third_south_num + third_west_num + third_north_num)";
            divided_list(db, third_num, "sum(game_num)", seasons, false, true).await
        }
        "fourth_rate" => {
            let fourth_num =
                "sum(fourth_east_num + fourth_south_num + fourth_west_num + fourth_north_num)";
            divided_list(db, fourth_num, "sum(game_num)", seasons, false, true).await
        }
        "richi_agari_rate" => {
            divided_list(db, AGARI_RICHI_NUM, RICHI_NUM, seasons, false, true).await
        }
        "richi_houjuu_rate" => {
            let richi_houjuu_num =
                "sum(houjuu_dama_richi_num + houjuu_furo_richi_num + houjuu_richi_richi_num)";
            divided_list(db, richi_houjuu_num, RICHI_NUM, seasons, false, true).await
        }
        "richi_tsumo_rate" => {
            divided_list(
                db,
                "sum(agari_richi_tsumo_num)",
                AGARI_RICHI_NUM,
                seasons,
                false,
                true,
            )
            .await
        }
        "avg_richi_agari_score" => {
            divided_list(
                db,
                "sum(agari_richi_score)",
                AGARI_RICHI_NUM,
                seasons,
                false,
                true,
            )
            .await
        }
        "richi_ryukyoku_rate" => {
            let richi_ryukyoku_num = "sum(ryukyoku_noten_richi_num + ryukyoku_tenpai_richi_num)";
            divided_list(db, richi_ryukyoku_num, RICHI_NUM, seasons, false, true).await
        }
        "avg_richi_turn" => {
            divided_list(db, "sum(richi_turn_num)", RICHI_NUM, seasons, false, true).await
        }
        "avg_richi_dora" => {
            divided_list(db, "sum(richi_dora_num)", RICHI_NUM, seasons, false, true).await
        }
        "richi_first_rate" => {
            divided_list(db, "sum(richi_first_num)", RICHI_NUM, seasons, false, true).await
        }
        "richi_chase_rate" => {
            divided_list(db, "sum(richi_chase_num)", RICHI_NUM, seasons, false, true).await
        }
        "richi_chased_rate" => {
            divided_list(db, "sum(richi_chased_num)", RICHI_NUM, seasons, false, true).await
        }
        "ippatsu_rate" => {
            let ippatsu_num = "sum(agari_richi_ron_ippatsu_num + agari_richi_tsumo_ippatsu_num)";
            divided_list(db, ippatsu_num, AGARI_RICHI_NUM, seasons, false, true).await
        }
        "uradora_rate" => {
            let uradora_kyoku_num = "sum(total.agari_richi_ron_uradora_kyoku_num
                + total.agari_richi_tsumo_uradora_kyoku_num)";
            divided_list(db, uradora_kyoku_num, AGARI_RICHI_NUM, seasons, false, true).await
        }
        "highest_score" => {
            let mut list = pro_value_list(db, "max(game_highest_score)", seasons, false).await;
            sort_value_list(&mut list);
            list
        }
        "lowest_score" => {
            let mut list = pro_value_list(db, "min(game_lowest_score)", seasons, false).await;
            sort_value_list(&mut list);
            list
        }
        "renchan_max_num" => value_list(db, "max(renchan_max_num)", seasons, false, true).await,
        _ => vec![],
    }
}

fn sort_value_list(list: &mut Vec<ProValueItem>) {
    list.sort_unstable_by(|a, b| a.value.partial_cmp(&b.value).unwrap());
}

async fn value_list(
    db: &DatabaseConnection,
    value_sql: &str,
    seasons: &Option<Vec<i32>>,
    is_float: bool,
    sort: bool,
) -> Vec<ProValueItem> {
    let mut list = pro_value_list(db, value_sql, seasons, is_float).await;
    if sort {
        sort_value_list(&mut list);
    }
    list
}

async fn divided_list(
    db: &DatabaseConnection,
    value_sql: &str,
    value_sql2: &str,
    seasons: &Option<Vec<i32>>,
    is_float: bool,
    sort: bool,
) -> Vec<ProValueItem> {
    let list = pro_tuple_value_list(db, value_sql, value_sql2, seasons, is_float, false).await;

    let mut list = list
        .iter()
        .map(|item| ProValueItem {
            pro_id: item.pro_id,
            pro_name: item.pro_name.clone(),
            value: {
                let divide_num = item.values.1.integer_value().unwrap() as f64;
                let divided_num = {
                    if is_float {
                        item.values.0.float_value().unwrap()
                    } else {
                        item.values.0.integer_value().unwrap() as f64
                    }
                };
                if divide_num == 0.0 {
                    Value::Float(0.0)
                } else {
                    Value::Float(divided_num / divide_num)
                }
            },
        })
        .collect();
    if sort {
        sort_value_list(&mut list);
    }
    list
}

async fn pro_value_list(
    db: &DatabaseConnection,
    value_sql: &str,
    seasons: &Option<Vec<i32>>,
    is_float: bool,
) -> Vec<ProValueItem> {
    SeasonProDao::sum_column_group_by_pro(db, value_sql, seasons)
        .await
        .iter()
        .map(|res| ProValueItem {
            pro_id: res.try_get("", "pro_id").unwrap_or_default(),
            pro_name: res.try_get("", "pro_name").unwrap_or_default(),
            value: {
                if is_float {
                    Value::Float(res.try_get("", "value").unwrap_or_default())
                } else {
                    Value::Integer(res.try_get("", "value").unwrap_or_default())
                }
            },
        })
        .collect()
}

/// TODO 结合上面的函数，简化？
async fn pro_tuple_value_list(
    db: &DatabaseConnection,
    value_sql: &str,
    value_sql2: &str,
    seasons: &Option<Vec<i32>>,
    is_float: bool,
    is_float2: bool,
) -> Vec<ProTupleValueItem> {
    SeasonProDao::sum_two_column_group_by_pro(db, value_sql, value_sql2, seasons)
        .await
        .iter()
        .map(|res| ProTupleValueItem {
            pro_id: res.try_get("", "pro_id").unwrap_or_default(),
            pro_name: res.try_get("", "pro_name").unwrap_or_default(),
            values: (
                {
                    if is_float {
                        Value::Float(res.try_get("", "value").unwrap_or_default())
                    } else {
                        Value::Integer(res.try_get("", "value").unwrap_or_default())
                    }
                },
                {
                    if is_float2 {
                        Value::Float(res.try_get("", "value2").unwrap_or_default())
                    } else {
                        Value::Integer(res.try_get("", "value2").unwrap_or_default())
                    }
                },
            ),
        })
        .collect()
}

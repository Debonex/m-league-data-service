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

/// 获取某个选手的统计数据，可根据赛季id进行筛选
pub async fn statistic(
    db: &DatabaseConnection,
    pro_id: i32,
    seasons: &Option<Vec<i32>>,
) -> Statistic {
    let season_pro_list = SeasonProDao::select_season_pro_by_pro_id(db, pro_id, seasons).await;
    StatisticDomain::statistic(season_pro_list)
}

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
            let mut list =
                value_list(db, "sum(score_point + rank_point)", seasons, true, true).await;
            // result calculated by sqlite engine is not precise, to fixed 1.
            for item in &mut list {
                let value = item.value.float_value().unwrap();
                item.value = Value::Float((value * 10.0).round() / 10.0);
            }
            list
        }
        "kyoku_num" => {
            value_list(
                db,
                "sum(kyoku_east_num + kyoku_north_num + kyoku_west_num + kyoku_north_num)",
                seasons,
                false,
                true,
            )
            .await
        }
        "avg_point" => {
            let list = pro_sum_two_value(
                db,
                "sum(score_point + rank_point)",
                "sum(game_num)",
                seasons,
                true,
                false,
            )
            .await;

            let mut list = list
                .iter()
                .map(|item| ProValueItem {
                    pro_id: item.pro_id,
                    pro_name: item.pro_name.clone(),
                    value: Value::Float(
                        item.values.0.float_value().unwrap()
                            / item.values.1.integer_value().unwrap() as f64,
                    ),
                })
                .collect();
            sort_value_list(&mut list);
            list
        }
        _ => vec![],
    }
}

fn sort_value_list(list: &mut Vec<ProValueItem>) {
    list.sort_unstable_by(|a, b| a.value.partial_cmp(&b.value).unwrap());
}

/// TODO DELETE
async fn value_list(
    db: &DatabaseConnection,
    value_sql: &str,
    seasons: &Option<Vec<i32>>,
    is_float: bool,
    sort: bool,
) -> Vec<ProValueItem> {
    let mut list = pro_sum_value(db, value_sql, seasons, is_float).await;
    if sort {
        sort_value_list(&mut list);
    }
    list
}

async fn pro_sum_value(
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
async fn pro_sum_two_value(
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

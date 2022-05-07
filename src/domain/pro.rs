use super::statistic as StatisticDomain;
use crate::data::pro as ProDao;
use crate::data::season_pro as SeasonProDao;
use crate::entity::pro::Model as ProModel;
use crate::vo::ProFloatValueItem;
use crate::vo::ProIntegerValueItem;
use crate::vo::ProRankItem;
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
) -> Vec<ProRankItem> {
    match key {
        "rank_point" | "score_point" => {
            let mut list = pro_sum_float_value(db, format!("sum({})", key).as_str(), seasons).await;
            list.sort_unstable_by(|a, b| a.value.partial_cmp(&b.value).unwrap());
            list.iter()
                .map(|x| {
                    ProRankItem::Float(ProFloatValueItem {
                        pro_id: x.pro_id,
                        pro_name: x.pro_name.clone(),
                        value: x.value,
                    })
                })
                .collect()
        }
        "point" => {
            let mut list = pro_sum_float_value(db, "sum(score_point + rank_point)", seasons).await;
            list.sort_unstable_by(|a, b| a.value.partial_cmp(&b.value).unwrap());
            list.iter()
                .map(|x| {
                    ProRankItem::Float(ProFloatValueItem {
                        pro_id: x.pro_id,
                        pro_name: x.pro_name.clone(),
                        value: x.value,
                    })
                })
                .collect()
        }
        "game_num" => {
            let mut list = pro_sum_integer_value(db, "sum(game_num)", seasons).await;
            list.sort_unstable_by(|a, b| a.value.cmp(&b.value));
            list.iter()
                .map(|x| {
                    ProRankItem::Integer(ProIntegerValueItem {
                        pro_id: x.pro_id,
                        pro_name: x.pro_name.clone(),
                        value: x.value,
                    })
                })
                .collect()
        }
        _ => vec![],
    }
}

async fn pro_sum_float_value(
    db: &DatabaseConnection,
    key: &str,
    seasons: &Option<Vec<i32>>,
) -> Vec<ProFloatValueItem> {
    SeasonProDao::sum_column_group_by_pro(db, key, seasons)
        .await
        .iter()
        .map(|res| ProFloatValueItem {
            pro_id: res.try_get("", "pro_id").unwrap_or_default(),
            pro_name: res.try_get("", "pro_name").unwrap_or_default(),
            value: res.try_get("", "value").unwrap_or_default(),
        })
        .collect()
}

async fn pro_sum_integer_value(
    db: &DatabaseConnection,
    key: &str,
    seasons: &Option<Vec<i32>>,
) -> Vec<ProIntegerValueItem> {
    SeasonProDao::sum_column_group_by_pro(db, key, seasons)
        .await
        .iter()
        .map(|res| ProIntegerValueItem {
            pro_id: res.try_get("", "pro_id").unwrap_or_default(),
            pro_name: res.try_get("", "pro_name").unwrap_or_default(),
            value: res.try_get("", "value").unwrap_or_default(),
        })
        .collect()
}

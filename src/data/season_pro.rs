use super::*;
use sea_orm::Condition;

/// 从db获取某个选手的赛季数据，可根据赛季id进行筛选
pub async fn select_season_pro_by_pro_id(
    db: &DatabaseConnection,
    pro_id: i32,
    seasons: &Option<Vec<i32>>,
) -> Vec<SeasonProModel> {
    let mut condition = Condition::all().add(SeasonProColumn::ProId.eq(pro_id));
    if let Some(seasons) = seasons {
        condition = condition.add(SeasonProColumn::SeasonId.is_in(seasons.into_iter().map(|x| *x)));
    }

    SeasonProEntity::find()
        .filter(condition)
        .all(db)
        .await
        .unwrap_or(vec![])
}

/// 从db获取赛季数据，可根据选手id、赛季id进行筛选
pub async fn select_season_pro(
    db: &DatabaseConnection,
    pros: &Option<Vec<i32>>,
    seasons: &Option<Vec<i32>>,
) -> Vec<SeasonProModel> {
    let mut condition = Condition::all();
    if let Some(pros) = pros {
        condition = condition.add(SeasonProColumn::ProId.is_in(pros.into_iter().map(|x| *x)));
    }
    if let Some(seasons) = seasons {
        condition = condition.add(SeasonProColumn::SeasonId.is_in(seasons.into_iter().map(|x| *x)));
    }

    SeasonProEntity::find()
        .filter(condition)
        .all(db)
        .await
        .unwrap_or(vec![])
}

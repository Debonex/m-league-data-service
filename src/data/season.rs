use super::*;
use sea_orm::Condition;

const SEASON_TYPE_REGULAR: &str = "regular";
const SEASON_TYPE_SEMI_FINAL: &str = "semi_final";
const SEASON_TYPE_FINAL: &str = "final";

/// 从db获取所有赛季的基本信息
pub async fn get_all_season_list(db: &DatabaseConnection) -> Vec<SeasonModel> {
    SeasonEntity::find().all(db).await.unwrap_or(vec![])
}

/// 从db获取赛季的基本信息，可根据赛季年id、赛季类型筛选
pub async fn select_season(
    db: &DatabaseConnection,
    season_year_list: &Option<Vec<i32>>,
    season_type: &Option<String>,
) -> Vec<SeasonModel> {
    let mut condition = Condition::all();
    if let Some(list) = season_year_list {
        condition = condition.add(SeasonColumn::SeasonYearId.is_in(list.iter().map(|x| *x)));
    }
    if let Some(season_type) = season_type {
        if season_type == SEASON_TYPE_REGULAR
            || season_type == SEASON_TYPE_SEMI_FINAL
            || season_type == SEASON_TYPE_FINAL
        {
            condition = condition.add(SeasonColumn::SeasonType.eq(season_type.as_str()));
        } else {
            return vec![];
        }
    }
    SeasonEntity::find()
        .filter(condition)
        .all(db)
        .await
        .unwrap_or(vec![])
}

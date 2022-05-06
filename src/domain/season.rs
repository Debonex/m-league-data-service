use crate::data::season as SeasonDao;
use crate::data::season_pro as SeasonProDao;
use crate::domain::statistic as StatisticDomain;
use crate::entity::season::Model as SeasonModel;
use crate::vo::Statistic;
use sea_orm::DatabaseConnection;

/// 获取所有赛季的基本信息
pub async fn all_season_list(db: &DatabaseConnection) -> Vec<SeasonModel> {
    SeasonDao::get_all_season_list(db).await
}

/// 获取赛季的基本信息，可根据赛季年id、赛季类型进行筛选
pub async fn season_list(
    db: &DatabaseConnection,
    season_year_list: &Option<Vec<i32>>,
    season_type: &Option<String>,
) -> Vec<SeasonModel> {
    SeasonDao::select_season(db, season_year_list, season_type).await
}

/// 获取赛季的统计数据，可根据赛季id进行筛选
pub async fn statistic_by_id(db: &DatabaseConnection, season_list: &Option<Vec<i32>>) -> Statistic {
    let sp_list = SeasonProDao::select_season_pro(db, &None, season_list).await;
    StatisticDomain::statistic(sp_list)
}

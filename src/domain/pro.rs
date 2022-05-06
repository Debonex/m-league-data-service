use super::statistic as StatisticDomain;
use crate::data::pro as ProDao;
use crate::data::season_pro as SeasonProDao;
use crate::entity::pro::Model as ProModel;
use crate::vo::Statistic;
use sea_orm::DatabaseConnection;

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

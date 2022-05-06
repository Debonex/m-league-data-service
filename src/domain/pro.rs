use super::statistic as StatisticDomain;
use crate::data::pro as ProDao;
use crate::data::season_pro as SeasonProDao;
use crate::entity::pro::Model as ProModel;
use crate::vo::Statistic;
use sea_orm::DatabaseConnection;

pub async fn pro_data(db: &DatabaseConnection, id: u32) -> Statistic {
    let season_pro_list = SeasonProDao::get_season_pro_by_pro_id(db, id).await;
    StatisticDomain::data_from_season_pro_list(season_pro_list)
}

pub async fn pro_list(db: &DatabaseConnection) -> Vec<ProModel> {
    ProDao::get_pro_list(db).await
}

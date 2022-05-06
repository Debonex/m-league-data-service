use crate::data::season_pro as SeasonProDao;
use crate::data::season_year as SeasonYearDao;
use crate::domain::season as SeasonDomain;
use crate::domain::statistic as StatisticDomain;
use crate::entity::season_year::Model as SeasonYearModel;
use crate::vo::Statistic;
use sea_orm::DatabaseConnection;

/// 获取所有赛季年的基本信息
pub async fn all(db: &DatabaseConnection) -> Vec<SeasonYearModel> {
    SeasonYearDao::select_all_season_year(db).await
}

/// 获取某个赛季年的统计数据
pub async fn statistic(db: &DatabaseConnection, season_year_id: i32) -> Statistic {
    let season_list: Vec<i32> = SeasonDomain::season_list(db, &Some(vec![season_year_id]), &None)
        .await
        .iter()
        .map(|x| x.id)
        .collect();
    let season_pro_list = SeasonProDao::select_season_pro(db, &None, &Some(season_list)).await;
    StatisticDomain::statistic(season_pro_list)
}

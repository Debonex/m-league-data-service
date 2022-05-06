use crate::data::season_year as SeasonYearDao;
use crate::entity::season_year::Model as SeasonYearModel;
use sea_orm::DatabaseConnection;

pub async fn season_year_list(db: &DatabaseConnection) -> Vec<SeasonYearModel> {
    SeasonYearDao::get_season_year_list(db).await
}

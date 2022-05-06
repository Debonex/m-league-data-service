use crate::data::season as SeasonDao;
use crate::entity::season::Model as SeasonModel;
use sea_orm::DatabaseConnection;

pub async fn season_list(db: &DatabaseConnection) -> Vec<SeasonModel> {
    SeasonDao::get_season_list(db).await
}

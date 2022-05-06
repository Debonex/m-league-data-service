use sea_orm::DatabaseConnection;

use crate::data::team as TeamDao;
use crate::entity::team::Model as TeamModel;

pub async fn team_list(db: &DatabaseConnection) -> Vec<TeamModel> {
    TeamDao::get_team_list(db).await
}

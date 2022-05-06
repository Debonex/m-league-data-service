use sea_orm::DatabaseConnection;

use crate::data::team as TeamDao;
use crate::entity::team::Model as TeamModel;

/// 获取所有队伍的基本信息
pub async fn all(db: &DatabaseConnection) -> Vec<TeamModel> {
    TeamDao::select_all_team(db).await
}

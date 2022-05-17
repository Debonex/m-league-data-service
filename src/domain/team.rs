use sea_orm::DatabaseConnection;

use crate::data::team as TeamDao;
use crate::entity::team::Model as TeamModel;

/// 获取所有队伍的基本信息
pub async fn all(db: &DatabaseConnection) -> Vec<TeamModel> {
    TeamDao::select_all_team(db).await
}

/// 根据队伍id获取队伍的基本信息
pub async fn info(db: &DatabaseConnection, team_id: i32) -> TeamModel {
    TeamDao::select_team(db, team_id)
        .await
        .unwrap_or(TeamModel {
            id: team_id,
            team_code: None,
            team_name: None,
        })
}

/// 根据选手id获取队伍的基本信息
pub async fn info_by_pro_id(db: &DatabaseConnection, pro_id: i32) -> TeamModel {
    TeamDao::select_by_pro_id(db, pro_id)
        .await
        .unwrap_or(TeamModel {
            id: 0,
            team_code: None,
            team_name: None,
        })
}

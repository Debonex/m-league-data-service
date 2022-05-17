use super::*;
use sea_orm::Statement;

/// 获取所有队伍的基本信息
pub async fn select_all_team(db: &DatabaseConnection) -> Vec<TeamModel> {
    TeamEntity::find().all(db).await.unwrap_or(vec![])
}

/// 根据队伍id获取队伍的基本信息
pub async fn select_team(db: &DatabaseConnection, team_id: i32) -> Option<TeamModel> {
    TeamEntity::find_by_id(team_id)
        .one(db)
        .await
        .unwrap_or(None)
}

/// 根据选手id获取队伍的基本信息
pub async fn select_by_pro_id(db: &DatabaseConnection, pro_id: i32) -> Option<TeamModel> {
    TeamEntity::find()
        .from_raw_sql(Statement::from_string(
            sea_orm::DatabaseBackend::Sqlite,
            format!(
                "select * from team left join pro on team.id = pro.team_id where pro.id = {}",
                pro_id
            )
            .to_owned(),
        ))
        .one(db)
        .await
        .unwrap_or(None)

    // TODO join no work?
    // TeamEntity::find()
    //     .join(JoinType::LeftJoin, pro::Relation::Team.def())
    //     .filter(ProColumn::Id.eq(pro_id))
    //     .one(db)
    //     .await
    //     .unwrap_or(None)
}

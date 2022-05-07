use super::*;
use sea_orm::{Condition, ConnectionTrait, QueryResult, Statement};

/// 从db获取某个选手的赛季数据，可根据赛季id进行筛选
pub async fn select_season_pro_by_pro_id(
    db: &DatabaseConnection,
    pro_id: i32,
    seasons: &Option<Vec<i32>>,
) -> Vec<SeasonProModel> {
    let mut condition = Condition::all().add(SeasonProColumn::ProId.eq(pro_id));
    if let Some(seasons) = seasons {
        condition = condition.add(SeasonProColumn::SeasonId.is_in(seasons.into_iter().map(|x| *x)));
    }

    SeasonProEntity::find()
        .filter(condition)
        .all(db)
        .await
        .unwrap_or(vec![])
}

/// 从db获取赛季数据，可根据选手id、赛季id进行筛选
pub async fn select_season_pro(
    db: &DatabaseConnection,
    pros: &Option<Vec<i32>>,
    seasons: &Option<Vec<i32>>,
) -> Vec<SeasonProModel> {
    let mut condition = Condition::all();
    if let Some(pros) = pros {
        condition = condition.add(SeasonProColumn::ProId.is_in(pros.into_iter().map(|x| *x)));
    }
    if let Some(seasons) = seasons {
        condition = condition.add(SeasonProColumn::SeasonId.is_in(seasons.into_iter().map(|x| *x)));
    }

    SeasonProEntity::find()
        .filter(condition)
        .all(db)
        .await
        .unwrap_or(vec![])
}

/// 从db获取所有选手的某一赛季数据，赛季id进行筛选
pub async fn sum_column_group_by_pro(
    db: &DatabaseConnection,
    value: &str,
    seasons: &Option<Vec<i32>>,
) -> Vec<QueryResult> {
    let sql_select_from = format!(
        "SELECT season_pro.pro_id, {} AS value, pro.pro_name FROM season_pro, pro",
        value
    );
    let where_seasons_and = match seasons {
        Some(seasons) => format!("season_pro.season_id in {} and", vec_str(seasons.to_vec())),
        _ => String::new(),
    };
    let sql_where = format!("WHERE {} season_pro.pro_id = pro.id", where_seasons_and);
    let sql = format!("{} {} group by pro_id", sql_select_from, sql_where);
    let stmt = Statement::from_string(sea_orm::DatabaseBackend::Sqlite, sql.to_owned());

    db.query_all(stmt).await.unwrap_or(vec![])
}

/// 把Vec<i32> 转为sql中合法的列表字符串
/// vec![1,2,3] => "(1, 2, 3)"
fn vec_str(vec: Vec<i32>) -> String {
    let str = format!("{:?}", vec);
    let len = str.len();
    format!("({})", &str[1..len - 1])
}

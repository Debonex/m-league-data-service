// data access
use crate::entity::pro::{Entity as ProEntity, Model as ProModel};
use crate::entity::season::{Column as SeasonColumn, Entity as SeasonEntity, Model as SeasonModel};
use crate::entity::season_pro::{
    Column as SeasonProColumn, Entity as SeasonProEntity, Model as SeasonProModel,
};
use crate::entity::season_year::{Entity as SeasonYearEntity, Model as SeasonYearModel};
use crate::entity::team::{Entity as TeamEntity, Model as TeamModel};
use sea_orm::DatabaseConnection;
use sea_orm::{entity::*, EntityTrait, QueryFilter};

pub mod pro;
pub mod season;
pub mod season_pro;
pub mod season_year;
pub mod team;

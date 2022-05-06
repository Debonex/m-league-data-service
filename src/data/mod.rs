// data access
use crate::entity::pro::{Entity as ProEntity, Model as ProModel};
use crate::entity::season::{Entity as SeasonEntity, Model as SeasonModel};
use crate::entity::season_pro::{Column as SPColumn, Entity as SPEntity, Model as SPModel};
use crate::entity::season_year::{Entity as SeasonYearEntity, Model as SeasonYearModel};
use crate::entity::team::{Entity as TeamEntity, Model as TeamModel};
use sea_orm::DatabaseConnection;
use sea_orm::{entity::*, EntityTrait, QueryFilter};

pub mod pro;
pub mod season;
pub mod season_pro;
pub mod season_year;
pub mod team;

use crate::common::format_sql_vec;
use rocket::{serde, State};
use sqlx::{Pool, Sqlite};
use std::{
    cmp::{max, min},
    collections::HashMap,
    iter::Sum,
    ops::Add,
};

/// 从db获取某个选手的赛季数据，可根据赛季id进行筛选
pub async fn select_season_pro_by_pro_id(
    pool: &State<Pool<Sqlite>>,
    pro_id: i64,
    seasons: &Option<Vec<i64>>,
) -> Vec<SeasonPro> {
    let sql_string = if let Some(seasons) = seasons {
        format!(
            "select * from season_pro where pro_id = ? and season_id in {}",
            format_sql_vec(seasons)
        )
    } else {
        "select * from season_pro where pro_id = ?".to_string()
    };
    sqlx::query_as::<_, SeasonPro>(&sql_string)
        .bind(pro_id)
        .fetch_all(pool.inner())
        .await
        .unwrap_or_default()
}

/// 从db获取赛季数据，可根据选手id、赛季id进行筛选
pub async fn select_season_pro(
    pool: &State<Pool<Sqlite>>,
    pros: &Option<Vec<i64>>,
    seasons: &Option<Vec<i64>>,
) -> Vec<SeasonPro> {
    let sql_string = if let Some(pros) = pros {
        if let Some(seasons) = seasons {
            format!(
                "select * from season_pro where pro_id in {} and season_id in {}",
                format_sql_vec(pros),
                format_sql_vec(seasons)
            )
        } else {
            format!(
                "select * from season_pro where pro_id in {}",
                format_sql_vec(pros)
            )
        }
    } else if let Some(seasons) = seasons {
        format!(
            "select * from season_pro where season_id in {}",
            format_sql_vec(seasons)
        )
    } else {
        "select * from season_pro".to_string()
    };

    sqlx::query_as::<_, SeasonPro>(&sql_string)
        .fetch_all(pool.inner())
        .await
        .unwrap_or_default()
}

/// 从db获取赛季数据，可根据队伍id、赛季id进行筛选
pub async fn select_season_pro_by_teams(
    pool: &State<Pool<Sqlite>>,
    teams: &Option<Vec<i64>>,
    seasons: &Option<Vec<i64>>,
) -> Vec<SeasonPro> {
    let sql_string = if let Some(teams) = teams {
        if let Some(seasons) = seasons {
            format!(
                "select * from season_pro where team_id in {} and season_id in {}",
                format_sql_vec(teams),
                format_sql_vec(seasons)
            )
        } else {
            format!(
                "select * from season_pro where team_id in {}",
                format_sql_vec(teams)
            )
        }
    } else if let Some(seasons) = seasons {
        format!(
            "select * from season_pro where season_id in {}",
            format_sql_vec(seasons)
        )
    } else {
        "select * from season_pro".to_string()
    };

    sqlx::query_as::<_, SeasonPro>(&sql_string)
        .fetch_all(pool.inner())
        .await
        .unwrap_or_default()
}

/// 从db获取某个队伍的赛季数据，可根据赛季id进行筛选
pub async fn select_season_pro_by_team_id(
    pool: &State<Pool<Sqlite>>,
    team_id: i64,
    seasons: &Option<Vec<i64>>,
) -> Vec<SeasonPro> {
    let sql_string = if let Some(seasons) = seasons {
        format!(
            "select * from season_pro where team_id = ? and season_id in {}",
            format_sql_vec(seasons)
        )
    } else {
        "select * from season_pro where team_id = ?".to_string()
    };
    sqlx::query_as::<_, SeasonPro>(&sql_string)
        .bind(team_id)
        .fetch_all(pool.inner())
        .await
        .unwrap_or_default()
}

#[derive(Debug, sqlx::FromRow)]
pub struct SeasonPro {
    pub id: i64,
    pub season_id: Option<i64>,
    pub pro_id: Option<i64>,
    pub team_id: Option<i64>,
    pub game_num: i64,
    pub game_east_start_num: i64,
    pub game_south_start_num: i64,
    pub game_west_start_num: i64,
    pub game_north_start_num: i64,
    pub game_highest_score: Option<i64>,
    pub game_lowest_score: Option<i64>,
    pub first_east_num: i64,
    pub first_south_num: i64,
    pub first_west_num: i64,
    pub first_north_num: i64,
    pub second_east_num: i64,
    pub second_south_num: i64,
    pub second_west_num: i64,
    pub second_north_num: i64,
    pub third_east_num: i64,
    pub third_south_num: i64,
    pub third_west_num: i64,
    pub third_north_num: i64,
    pub fourth_east_num: i64,
    pub fourth_south_num: i64,
    pub fourth_west_num: i64,
    pub fourth_north_num: i64,
    pub first_score: i64,
    pub second_score: i64,
    pub third_score: i64,
    pub fourth_score: i64,
    pub score_point: f32,
    pub rank_point: f32,
    pub kyoku_east_num: i64,
    pub kyoku_south_num: i64,
    pub kyoku_west_num: i64,
    pub kyoku_north_num: i64,
    pub shanten_num: i64,
    pub haipai_dora_num: i64,
    pub renchan_max_num: i64,
    pub furo_num: i64,
    pub richi_num: i64,
    pub richi_first_num: i64,
    pub richi_chase_num: i64,
    pub richi_good_num: i64,
    pub richi_stupid_num: i64,
    pub richi_machi_num: i64,
    pub richi_turn_num: i64,
    pub richi_suji_num: i64,
    pub richi_furiten_num: i64,
    pub richi_dora_num: i64,
    pub richi_han_num: i64,
    pub richi_aka_num: i64,
    pub richi_chased_num: i64,
    pub ryukyoku_tenpai_richi_num: i64,
    pub ryukyoku_tenpai_menzen_num: i64,
    pub ryukyoku_tenpai_furo_num: i64,
    pub ryukyoku_noten_richi_num: i64,
    pub ryukyoku_noten_menzen_num: i64,
    pub ryukyoku_noten_furo_num: i64,
    pub agari_dama_ron_num: i64,
    pub agari_dama_tsumo_num: i64,
    pub agari_dama_score: i64,
    pub agari_richi_ron_num: i64,
    pub agari_richi_tsumo_num: i64,
    pub agari_richi_score: i64,
    pub agari_richi_ron_ippatsu_num: i64,
    pub agari_richi_tsumo_ippatsu_num: i64,
    pub agari_richi_ron_uradora_kyoku_num: i64,
    pub agari_richi_tsumo_uradora_kyoku_num: i64,
    pub agari_furo_ron_num: i64,
    pub agari_furo_tsumo_num: i64,
    pub agari_furo_score: i64,
    pub agari_turn_num: i64,
    pub houjuu_dama_menzen_num: i64,
    pub houjuu_dama_furo_num: i64,
    pub houjuu_dama_richi_num: i64,
    pub houjuu_dama_score: i64,
    pub houjuu_richi_menzen_num: i64,
    pub houjuu_richi_furo_num: i64,
    pub houjuu_richi_richi_num: i64,
    pub houjuu_richi_score: i64,
    pub houjuu_richi_ippatsu_num: i64,
    pub houjuu_furo_menzen_num: i64,
    pub houjuu_furo_furo_num: i64,
    pub houjuu_furo_richi_num: i64,
    pub houjuu_furo_score: i64,
    pub blown_num: i64,
    pub blown_score: i64,
    pub yaku: String,
}

impl SeasonPro {
    pub fn first_num(&self) -> i64 {
        self.first_east_num + self.first_south_num + self.first_west_num + self.first_north_num
    }

    pub fn second_num(&self) -> i64 {
        self.second_east_num + self.second_south_num + self.second_west_num + self.second_north_num
    }

    pub fn third_num(&self) -> i64 {
        self.third_east_num + self.third_south_num + self.third_west_num + self.third_north_num
    }

    pub fn fourth_num(&self) -> i64 {
        self.fourth_east_num + self.fourth_south_num + self.fourth_west_num + self.fourth_north_num
    }

    pub fn rank_total(&self) -> i64 {
        self.first_num() + self.second_num() * 2 + self.third_num() * 3 + self.fourth_num() * 4
    }

    pub fn kyoku_num(&self) -> i64 {
        self.kyoku_east_num + self.kyoku_north_num + self.kyoku_west_num + self.kyoku_north_num
    }

    pub fn point(&self) -> f32 {
        self.rank_point + self.score_point
    }

    pub fn agari_dama_num(&self) -> i64 {
        self.agari_dama_ron_num + self.agari_dama_tsumo_num
    }

    pub fn agari_furo_num(&self) -> i64 {
        self.agari_furo_ron_num + self.agari_furo_tsumo_num
    }

    pub fn agari_richi_num(&self) -> i64 {
        self.agari_richi_ron_num + self.agari_richi_tsumo_num
    }

    pub fn agari_num(&self) -> i64 {
        self.agari_dama_num() + self.agari_furo_num() + self.agari_richi_num()
    }

    pub fn houjuu_num(&self) -> i64 {
        self.houjuu_dama_furo_num
            + self.houjuu_dama_menzen_num
            + self.houjuu_dama_richi_num
            + self.houjuu_furo_furo_num
            + self.houjuu_furo_menzen_num
            + self.houjuu_furo_richi_num
            + self.houjuu_richi_furo_num
            + self.houjuu_richi_menzen_num
            + self.houjuu_richi_richi_num
    }

    pub fn tsumo_num(&self) -> i64 {
        self.agari_dama_tsumo_num + self.agari_furo_tsumo_num + self.agari_richi_tsumo_num
    }

    pub fn ron_num(&self) -> i64 {
        self.agari_dama_ron_num + self.agari_furo_ron_num + self.agari_richi_ron_num
    }

    pub fn ryukyoku_tenpai_num(&self) -> i64 {
        self.ryukyoku_tenpai_menzen_num
            + self.ryukyoku_tenpai_furo_num
            + self.ryukyoku_tenpai_richi_num
    }

    pub fn ryukyoku_noten_num(&self) -> i64 {
        self.ryukyoku_noten_menzen_num
            + self.ryukyoku_noten_furo_num
            + self.ryukyoku_noten_richi_num
    }

    pub fn ryukyoku_num(&self) -> i64 {
        self.ryukyoku_noten_num() + self.ryukyoku_tenpai_num()
    }

    pub fn agari_score(&self) -> i64 {
        self.agari_dama_score + self.agari_furo_score + self.agari_richi_score
    }

    pub fn houjuu_score(&self) -> i64 {
        self.houjuu_dama_score + self.houjuu_furo_score + self.houjuu_richi_score
    }
}

impl Default for SeasonPro {
    fn default() -> Self {
        Self {
            id: 0,
            season_id: None,
            pro_id: None,
            team_id: None,
            game_num: 0,
            game_east_start_num: 0,
            game_south_start_num: 0,
            game_west_start_num: 0,
            game_north_start_num: 0,
            game_highest_score: None,
            game_lowest_score: None,
            first_east_num: 0,
            first_south_num: 0,
            first_west_num: 0,
            first_north_num: 0,
            second_east_num: 0,
            second_south_num: 0,
            second_west_num: 0,
            second_north_num: 0,
            third_east_num: 0,
            third_south_num: 0,
            third_west_num: 0,
            third_north_num: 0,
            fourth_east_num: 0,
            fourth_south_num: 0,
            fourth_west_num: 0,
            fourth_north_num: 0,
            first_score: 0,
            second_score: 0,
            third_score: 0,
            fourth_score: 0,
            score_point: 0.0,
            rank_point: 0.0,
            kyoku_east_num: 0,
            kyoku_south_num: 0,
            kyoku_west_num: 0,
            kyoku_north_num: 0,
            shanten_num: 0,
            haipai_dora_num: 0,
            renchan_max_num: 0,
            furo_num: 0,
            richi_num: 0,
            richi_first_num: 0,
            richi_chase_num: 0,
            richi_good_num: 0,
            richi_stupid_num: 0,
            richi_machi_num: 0,
            richi_turn_num: 0,
            richi_suji_num: 0,
            richi_furiten_num: 0,
            richi_dora_num: 0,
            richi_han_num: 0,
            richi_aka_num: 0,
            richi_chased_num: 0,
            ryukyoku_tenpai_richi_num: 0,
            ryukyoku_tenpai_menzen_num: 0,
            ryukyoku_tenpai_furo_num: 0,
            ryukyoku_noten_richi_num: 0,
            ryukyoku_noten_menzen_num: 0,
            ryukyoku_noten_furo_num: 0,
            agari_dama_ron_num: 0,
            agari_dama_tsumo_num: 0,
            agari_dama_score: 0,
            agari_richi_ron_num: 0,
            agari_richi_tsumo_num: 0,
            agari_richi_score: 0,
            agari_richi_ron_ippatsu_num: 0,
            agari_richi_tsumo_ippatsu_num: 0,
            agari_richi_ron_uradora_kyoku_num: 0,
            agari_richi_tsumo_uradora_kyoku_num: 0,
            agari_furo_ron_num: 0,
            agari_furo_tsumo_num: 0,
            agari_furo_score: 0,
            agari_turn_num: 0,
            houjuu_dama_menzen_num: 0,
            houjuu_dama_furo_num: 0,
            houjuu_dama_richi_num: 0,
            houjuu_dama_score: 0,
            houjuu_richi_menzen_num: 0,
            houjuu_richi_furo_num: 0,
            houjuu_richi_richi_num: 0,
            houjuu_richi_score: 0,
            houjuu_richi_ippatsu_num: 0,
            houjuu_furo_menzen_num: 0,
            houjuu_furo_furo_num: 0,
            houjuu_furo_richi_num: 0,
            houjuu_furo_score: 0,
            blown_num: 0,
            blown_score: 0,
            yaku: "{}".to_string(),
        }
    }
}

impl<'a> Add<&'a SeasonPro> for SeasonPro {
    type Output = Self;

    fn add(self, other: &'a SeasonPro) -> Self::Output {
        let mut yaku: HashMap<String, i32> =
            serde::json::from_str(&self.yaku).unwrap_or(HashMap::default());
        let other_yaku: HashMap<String, i32> =
            serde::json::from_str(&other.yaku).unwrap_or(HashMap::default());
        for yaku_pair in other_yaku {
            let count = yaku.entry(yaku_pair.0).or_insert(0);
            *count += yaku_pair.1;
        }
        Self {
            id: self.id,
            season_id: self.season_id,
            pro_id: self.pro_id,
            team_id: self.team_id,
            game_num: self.game_num + other.game_num,
            game_east_start_num: self.game_east_start_num + other.game_east_start_num,
            game_south_start_num: self.game_south_start_num + other.game_south_start_num,
            game_west_start_num: self.game_west_start_num + other.game_west_start_num,
            game_north_start_num: self.game_north_start_num + other.game_north_start_num,
            game_highest_score: max(
                self.game_highest_score.or(other.game_highest_score),
                other.game_highest_score,
            ),
            game_lowest_score: min(
                self.game_lowest_score.or(other.game_lowest_score),
                other.game_lowest_score,
            ),
            first_east_num: self.first_east_num + other.first_east_num,
            first_south_num: self.first_south_num + other.first_south_num,
            first_west_num: self.first_west_num + other.first_west_num,
            first_north_num: self.first_north_num + other.first_north_num,
            second_east_num: self.second_east_num + other.second_east_num,
            second_south_num: self.second_south_num + other.second_south_num,
            second_west_num: self.second_west_num + other.second_west_num,
            second_north_num: self.second_north_num + other.second_north_num,
            third_east_num: self.third_east_num + other.third_east_num,
            third_south_num: self.third_south_num + other.third_south_num,
            third_west_num: self.third_west_num + other.third_west_num,
            third_north_num: self.third_north_num + other.third_north_num,
            fourth_east_num: self.fourth_east_num + other.fourth_east_num,
            fourth_south_num: self.fourth_south_num + other.fourth_south_num,
            fourth_west_num: self.fourth_west_num + other.fourth_west_num,
            fourth_north_num: self.fourth_north_num + other.fourth_north_num,
            first_score: self.first_score + other.first_score,
            second_score: self.second_score + other.second_score,
            third_score: self.third_score + other.third_score,
            fourth_score: self.fourth_score + other.fourth_score,
            score_point: self.score_point + other.score_point,
            rank_point: self.rank_point + other.rank_point,
            kyoku_east_num: self.kyoku_east_num + other.kyoku_east_num,
            kyoku_south_num: self.kyoku_south_num + other.kyoku_south_num,
            kyoku_west_num: self.kyoku_west_num + other.kyoku_west_num,
            kyoku_north_num: self.kyoku_north_num + other.kyoku_north_num,
            shanten_num: self.shanten_num + other.shanten_num,
            haipai_dora_num: self.haipai_dora_num + other.haipai_dora_num,
            renchan_max_num: max(self.renchan_max_num, other.renchan_max_num),
            furo_num: self.furo_num + other.furo_num,
            richi_num: self.richi_num + other.richi_num,
            richi_first_num: self.richi_first_num + other.richi_first_num,
            richi_chase_num: self.richi_chase_num + other.richi_chase_num,
            richi_good_num: self.richi_good_num + other.richi_good_num,
            richi_stupid_num: self.richi_stupid_num + other.richi_stupid_num,
            richi_machi_num: self.richi_machi_num + other.richi_machi_num,
            richi_turn_num: self.richi_turn_num + other.richi_turn_num,
            richi_suji_num: self.richi_suji_num + other.richi_suji_num,
            richi_furiten_num: self.richi_furiten_num + other.richi_furiten_num,
            richi_dora_num: self.richi_dora_num + other.richi_dora_num,
            richi_han_num: self.richi_han_num + other.richi_han_num,
            richi_aka_num: self.richi_aka_num + other.richi_aka_num,
            richi_chased_num: self.richi_chased_num + other.richi_chased_num,
            ryukyoku_tenpai_richi_num: self.ryukyoku_tenpai_richi_num
                + other.ryukyoku_tenpai_richi_num,
            ryukyoku_tenpai_menzen_num: self.ryukyoku_tenpai_menzen_num
                + other.ryukyoku_tenpai_menzen_num,
            ryukyoku_tenpai_furo_num: self.ryukyoku_tenpai_furo_num
                + other.ryukyoku_tenpai_furo_num,
            ryukyoku_noten_richi_num: self.ryukyoku_noten_richi_num
                + other.ryukyoku_noten_richi_num,
            ryukyoku_noten_menzen_num: self.ryukyoku_noten_menzen_num
                + other.ryukyoku_noten_menzen_num,
            ryukyoku_noten_furo_num: self.ryukyoku_noten_furo_num + other.ryukyoku_noten_furo_num,
            agari_dama_ron_num: self.agari_dama_ron_num + other.agari_dama_ron_num,
            agari_dama_tsumo_num: self.agari_dama_tsumo_num + other.agari_dama_tsumo_num,
            agari_dama_score: self.agari_dama_score + other.agari_dama_score,
            agari_richi_ron_num: self.agari_richi_ron_num + other.agari_richi_ron_num,
            agari_richi_tsumo_num: self.agari_richi_tsumo_num + other.agari_richi_tsumo_num,
            agari_richi_score: self.agari_richi_score + other.agari_richi_score,
            agari_richi_ron_ippatsu_num: self.agari_richi_ron_ippatsu_num
                + other.agari_richi_ron_ippatsu_num,
            agari_richi_tsumo_ippatsu_num: self.agari_richi_tsumo_ippatsu_num
                + other.agari_richi_tsumo_ippatsu_num,
            agari_richi_ron_uradora_kyoku_num: self.agari_richi_ron_uradora_kyoku_num
                + other.agari_richi_ron_uradora_kyoku_num,
            agari_richi_tsumo_uradora_kyoku_num: self.agari_richi_tsumo_uradora_kyoku_num
                + other.agari_richi_tsumo_uradora_kyoku_num,
            agari_furo_ron_num: self.agari_furo_ron_num + other.agari_furo_ron_num,
            agari_furo_tsumo_num: self.agari_furo_tsumo_num + other.agari_furo_tsumo_num,
            agari_furo_score: self.agari_furo_score + other.agari_furo_score,
            agari_turn_num: self.agari_turn_num + other.agari_turn_num,
            houjuu_dama_menzen_num: self.houjuu_dama_menzen_num + other.houjuu_dama_menzen_num,
            houjuu_dama_furo_num: self.houjuu_dama_furo_num + other.houjuu_dama_furo_num,
            houjuu_dama_richi_num: self.houjuu_dama_richi_num + other.houjuu_dama_richi_num,
            houjuu_dama_score: self.houjuu_dama_score + other.houjuu_dama_score,
            houjuu_richi_menzen_num: self.houjuu_richi_menzen_num + other.houjuu_richi_menzen_num,
            houjuu_richi_furo_num: self.houjuu_richi_furo_num + other.houjuu_richi_furo_num,
            houjuu_richi_richi_num: self.houjuu_richi_richi_num + other.houjuu_richi_richi_num,
            houjuu_richi_score: self.houjuu_richi_score + other.houjuu_richi_score,
            houjuu_richi_ippatsu_num: self.houjuu_richi_ippatsu_num
                + other.houjuu_richi_ippatsu_num,
            houjuu_furo_menzen_num: self.houjuu_furo_menzen_num + other.houjuu_furo_menzen_num,
            houjuu_furo_furo_num: self.houjuu_furo_furo_num + other.houjuu_furo_furo_num,
            houjuu_furo_richi_num: self.houjuu_furo_richi_num + other.houjuu_furo_richi_num,
            houjuu_furo_score: self.houjuu_furo_score + other.houjuu_furo_score,
            blown_num: self.blown_num + other.blown_num,
            blown_score: self.blown_score + other.blown_score,
            yaku: format!("{:?}", yaku),
        }
    }
}

impl<'a> Sum<&'a Self> for SeasonPro {
    fn sum<I: Iterator<Item = &'a Self>>(iter: I) -> Self {
        iter.fold(Self::default(), Add::add)
    }
}

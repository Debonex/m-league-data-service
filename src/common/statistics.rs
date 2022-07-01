use super::season_pro::SeasonPro;
use rocket::serde::{self, Serialize};
use std::collections::HashMap;

fn divide(divided: f64, divide: f64) -> f64 {
    if divide == 0.0 {
        return 0.0;
    }
    divided / divide as f64
}

/// 根据赛季数据列表进行统计，给出统计数据
pub fn statistics(sp_list: Vec<SeasonPro>) -> Statistic {
    let total: SeasonPro = sp_list.iter().sum();

    let total_point = total.point();
    let total_kyoku_num = total.kyoku_num();
    let total_agari_num = total.agari_num() as f64;
    let total_houjuu_num = total.houjuu_num() as f64;
    let total_agari_richi_num = total.agari_richi_num() as f64;
    let total_ryukyoku_num = total.ryukyoku_num() as f64;

    let total_houjuu_furo_num = (total.houjuu_dama_furo_num
        + total.houjuu_furo_furo_num
        + total.houjuu_richi_furo_num) as f64;

    let total_houjuu_richi_num = (total.houjuu_dama_richi_num
        + total.houjuu_furo_richi_num
        + total.houjuu_richi_richi_num) as f64;

    Statistic {
        game_num: total.game_num,
        kyoku_num: total_kyoku_num,
        point: (total_point * 10.0).round() / 10.0,
        avg_point: divide(total_point as f64, total.game_num as f64),
        agari_rate: divide(total_agari_num, total_kyoku_num as f64),
        houjuu_rate: divide(total_houjuu_num, total_kyoku_num as f64),
        houjuu_menzen_rate: divide(
            (total.houjuu_dama_menzen_num
                + total.houjuu_furo_menzen_num
                + total.houjuu_richi_menzen_num) as f64,
            total_houjuu_num,
        ),
        houjuu_furo_rate: divide(total_houjuu_furo_num, total_houjuu_num),
        houjuu_richi_rate: divide(total_houjuu_richi_num, total_houjuu_num as f64),
        houjuu_to_dama_rate: divide(
            (total.houjuu_dama_menzen_num
                + total.houjuu_dama_furo_num
                + total.houjuu_dama_richi_num) as f64,
            total_houjuu_num as f64,
        ),
        houjuu_to_furo_rate: divide(
            (total.houjuu_furo_menzen_num
                + total.houjuu_furo_furo_num
                + total.houjuu_furo_richi_num) as f64,
            total_houjuu_num as f64,
        ),
        houjuu_to_richi_rate: divide(
            (total.houjuu_richi_menzen_num
                + total.houjuu_richi_furo_num
                + total.houjuu_richi_richi_num) as f64,
            total_houjuu_num as f64,
        ),
        tsumo_rate: divide(total.tsumo_num() as f64, total_agari_num),
        agari_dama_rate: divide(total.agari_dama_num() as f64, total_agari_num),
        agari_furo_rate: divide(total.agari_furo_num() as f64, total_agari_num),
        agari_richi_rate: divide(total.agari_richi_num() as f64, total_agari_num),
        ryukyoku_rate: divide(total_ryukyoku_num, total_kyoku_num as f64),
        ryukyoku_tenpai_rate: divide(total.ryukyoku_tenpai_num() as f64, total_ryukyoku_num),
        furo_rate: divide(total.furo_num as f64, total_kyoku_num as f64),
        furo_agari_rate: divide(total.agari_furo_num() as f64, total.furo_num as f64),
        furo_ryukyoku_rate: divide(
            (total.ryukyoku_noten_furo_num + total.ryukyoku_tenpai_furo_num) as f64,
            total.furo_num as f64,
        ),
        furo_houjuu_rate: divide(total_houjuu_furo_num, total.furo_num as f64),
        avg_furo_agari_score: divide(total.agari_furo_score as f64, total.agari_furo_num() as f64),
        richi_rate: divide(total.richi_num as f64, total_kyoku_num as f64),
        avg_agari_turn: divide(total.agari_turn_num as f64, total_agari_num),
        avg_agari_score: divide(total.agari_score() as f64, total_agari_num),
        avg_houjuu_score: divide(total.houjuu_score() as f64, total_houjuu_num),
        avg_rank: divide(total.rank_total() as f64, total.game_num as f64),
        blown_rate: divide(total.blown_num as f64, total.kyoku_east_num as f64),
        avg_blown_score: divide(total.blown_score as f64, total.blown_num as f64),
        ron_rate: divide(total.ron_num() as f64, total_agari_num),
        first_rate: divide(total.first_num() as f64, total.game_num as f64),
        second_rate: divide(total.second_num() as f64, total.game_num as f64),
        third_rate: divide(total.third_num() as f64, total.game_num as f64),
        fourth_rate: divide(total.fourth_num() as f64, total.game_num as f64),
        avg_first_score: divide(total.first_score as f64, total.first_num() as f64),
        avg_second_score: divide(total.second_score as f64, total.second_num() as f64),
        avg_third_score: divide(total.third_score as f64, total.third_num() as f64),
        avg_fourth_score: divide(total.fourth_score as f64, total.fourth_num() as f64),
        richi_agari_rate: divide(total_agari_richi_num, total.richi_num as f64),
        richi_houjuu_rate: divide(total_houjuu_richi_num, total.richi_num as f64),
        richi_tsumo_rate: divide(total.agari_richi_tsumo_num as f64, total_agari_richi_num),
        avg_richi_agari_score: divide(total.agari_richi_score as f64, total_agari_richi_num),
        richi_ryukyoku_rate: divide(
            (total.ryukyoku_noten_richi_num + total.ryukyoku_tenpai_richi_num) as f64,
            total.richi_num as f64,
        ),
        avg_richi_turn: divide(total.richi_turn_num as f64, total.richi_num as f64),
        avg_richi_dora: divide(total.richi_dora_num as f64, total.richi_num as f64),
        richi_first_rate: divide(total.richi_first_num as f64, total.richi_num as f64),
        richi_chase_rate: divide(total.richi_chase_num as f64, total.richi_num as f64),
        richi_chased_rate: divide(total.richi_chased_num as f64, total.richi_num as f64),
        ippatsu_rate: divide(
            (total.agari_richi_ron_ippatsu_num + total.agari_richi_tsumo_ippatsu_num) as f64,
            total_agari_richi_num,
        ),
        uradora_rate: divide(
            (total.agari_richi_ron_uradora_kyoku_num + total.agari_richi_tsumo_uradora_kyoku_num)
                as f64,
            total_agari_richi_num,
        ),
        highest_score: total.game_highest_score.unwrap_or_default(),
        lowest_score: total.game_lowest_score.unwrap_or_default(),
        renchan_max_num: total.renchan_max_num,
        yaku: serde::json::from_str(&total.yaku).unwrap(),
    }
}

#[derive(Serialize, Debug)]
#[serde(crate = "rocket::serde")]
pub struct Statistic {
    pub game_num: i64,
    pub kyoku_num: i64,
    pub point: f32,
    pub avg_point: f64,
    pub agari_rate: f64,
    pub houjuu_rate: f64,
    pub houjuu_menzen_rate: f64,
    pub houjuu_furo_rate: f64,
    pub houjuu_richi_rate: f64,
    pub houjuu_to_dama_rate: f64,
    pub houjuu_to_furo_rate: f64,
    pub houjuu_to_richi_rate: f64,
    pub tsumo_rate: f64,
    pub agari_dama_rate: f64,
    pub agari_furo_rate: f64,
    pub agari_richi_rate: f64,
    pub ryukyoku_rate: f64,
    pub ryukyoku_tenpai_rate: f64,
    pub furo_rate: f64,
    pub furo_agari_rate: f64,
    pub furo_ryukyoku_rate: f64,
    pub furo_houjuu_rate: f64,
    pub avg_furo_agari_score: f64,
    pub richi_rate: f64,
    pub avg_agari_turn: f64,
    pub avg_agari_score: f64,
    pub avg_houjuu_score: f64,
    pub avg_rank: f64,
    pub blown_rate: f64,
    pub avg_blown_score: f64,
    pub ron_rate: f64,
    pub first_rate: f64,
    pub second_rate: f64,
    pub third_rate: f64,
    pub fourth_rate: f64,
    pub avg_first_score: f64,
    pub avg_second_score: f64,
    pub avg_third_score: f64,
    pub avg_fourth_score: f64,
    pub richi_agari_rate: f64,
    pub richi_houjuu_rate: f64,
    pub richi_tsumo_rate: f64,
    pub avg_richi_agari_score: f64,
    pub richi_ryukyoku_rate: f64,
    pub avg_richi_turn: f64,
    pub avg_richi_dora: f64,
    pub richi_first_rate: f64,
    pub richi_chase_rate: f64,
    pub richi_chased_rate: f64,
    pub ippatsu_rate: f64,
    pub uradora_rate: f64,
    pub highest_score: i64,
    pub lowest_score: i64,
    pub renchan_max_num: i64,
    pub yaku: HashMap<String, i32>,
}

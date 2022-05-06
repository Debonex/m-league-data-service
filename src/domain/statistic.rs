use crate::entity::season_pro::Model as SeasonProModel;
use crate::vo::Statistic;
use rust_decimal::prelude::ToPrimitive;

/// 根据赛季数据列表进行统计，给出统计数据
pub fn statistic(sp_list: Vec<SeasonProModel>) -> Statistic {
    let total: SeasonProModel = sp_list.iter().sum();

    let total_point = total.point().to_f64().unwrap_or(0.0);
    let total_kyoku_num = total.kyoku_num();
    let total_agari_num = total.agari_num() as f64;
    let total_houjuu_num = total.houjuu_num() as f64;
    let total_agari_richi_num = total.agari_richi_num() as f64;
    let total_ryukyoku_num = total.ryukyoku_num() as f64;
    Statistic {
        game_num: total.game_num,
        kyoku_num: total_kyoku_num,
        point: total_point,
        avg_point: total_point / total.game_num as f64,
        agari_rate: total_agari_num / total_kyoku_num as f64,
        houjuu_rate: total_houjuu_num / total_kyoku_num as f64,
        tsumo_rate: total.tsumo_num() as f64 / total_agari_num,
        dama_rate: total.dama_num() as f64 / total_agari_num,
        ryukyoku_rate: total_ryukyoku_num / total_kyoku_num as f64,
        ryukyoku_tenpai_rate: total.ryukyoku_tenpai_num() as f64 / total_ryukyoku_num,
        furo_rate: total.furo_num as f64 / total_kyoku_num as f64,
        richi_rate: total.richi_num as f64 / total_kyoku_num as f64,
        avg_agari_turn: total.agari_turn_num as f64 / total_agari_num,
        avg_agari_score: total.agari_score() as f64 / total_agari_num,
        avg_houjuu_score: total.houjuu_score() as f64 / total_houjuu_num,
        avg_rank: total.rank_total() as f64 / total.game_num as f64,
        blown_rate: total.blown_num as f64 / total.kyoku_east_num as f64,
        avg_blown_score: total.blown_score as f64 / total.blown_num as f64,
        ron_rate: total.ron_num() as f64 / total_agari_num,
        first_rate: total.first_num() as f64 / total.game_num as f64,
        second_rate: total.second_num() as f64 / total.game_num as f64,
        third_rate: total.third_num() as f64 / total.game_num as f64,
        fourth_rate: total.fourth_num() as f64 / total.game_num as f64,
        richi_agari_rate: total_agari_richi_num / total.richi_num as f64,
        richi_houjuu_rate: (total.houjuu_dama_richi_num
            + total.houjuu_furo_richi_num
            + total.houjuu_richi_richi_num) as f64
            / total_houjuu_num,
        richi_tsumo_rate: total.agari_richi_tsumo_num as f64 / total_agari_richi_num,
        avg_richi_agari_score: total.agari_richi_score as f64 / total_agari_richi_num,
        richi_ryukyoku_rate: (total.ryukyoku_noten_richi_num + total.ryukyoku_tenpai_richi_num)
            as f64
            / total.richi_num as f64,
        avg_richi_turn: total.richi_turn_num as f64 / total.richi_num as f64,
        avg_richi_dora: total.richi_dora_num as f64 / total.richi_num as f64,
        richi_first_rate: total.richi_first_num as f64 / total.richi_num as f64,
        richi_chase_rate: total.richi_chase_num as f64 / total.richi_num as f64,
        richi_chased_rate: total.richi_chased_num as f64 / total.richi_num as f64,
        ippatsu_rate: (total.agari_richi_ron_ippatsu_num + total.agari_richi_tsumo_ippatsu_num)
            as f64
            / total_agari_richi_num,
        uradora_rate: (total.agari_richi_ron_uradora_kyoku_num
            + total.agari_richi_tsumo_uradora_kyoku_num) as f64
            / total_agari_richi_num,
        highest_score: total.game_highest_score.unwrap_or_default(),
        lowest_score: total.game_lowest_score.unwrap_or_default(),
    }
}

use crate::entity::season_pro::Model as SeasonProModel;
use crate::vo::Statistic;
use rust_decimal::prelude::ToPrimitive;

fn divide(divided: f64, divide: f64) -> f64 {
    if divide == 0.0 {
        return 0.0;
    }
    divided / divide as f64
}

/// 根据赛季数据列表进行统计，给出统计数据
pub fn statistic(sp_list: Vec<SeasonProModel>) -> Statistic {
    let total: SeasonProModel = sp_list.iter().sum();

    let total_point = total.point().to_f64().unwrap_or(0.0);
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
    println!(
        "{:?}",
        Statistic {
            game_num: total.game_num,
            kyoku_num: total_kyoku_num,
            point: total_point,
            avg_point: divide(total_point, total.game_num as f64),
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
            avg_furo_agari_score: divide(
                total.agari_furo_score as f64,
                total.agari_furo_num() as f64
            ),
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
                (total.agari_richi_ron_uradora_kyoku_num
                    + total.agari_richi_tsumo_uradora_kyoku_num) as f64,
                total_agari_richi_num,
            ),
            highest_score: total.game_highest_score.unwrap_or_default(),
            lowest_score: total.game_lowest_score.unwrap_or_default(),
            renchan_max_num: total.renchan_max_num,
        }
    );
    Statistic {
        game_num: total.game_num,
        kyoku_num: total_kyoku_num,
        point: total_point,
        avg_point: divide(total_point, total.game_num as f64),
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
    }
}

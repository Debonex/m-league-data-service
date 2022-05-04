use crate::entity::season_pro::{Column as SPColumn, Entity as SPEntity, Model as SPModel};
use crate::{dto::ProData, pool::Db};
use rocket::serde::json::Json;
use rust_decimal::prelude::ToPrimitive;
use sea_orm::{entity::*, prelude::*, EntityTrait, QueryFilter};
use sea_orm_rocket::Connection;
use std::ops::Add;

#[get("/data/<id>")]
pub async fn pro_data(conn: Connection<'_, Db>, id: u32) -> Json<ProData> {
    let db = conn.into_inner();
    let result = SPEntity::find()
        .filter(SPColumn::ProId.eq(id))
        .all(db)
        .await;
    return match result {
        Ok(list) => Json(data_from_season_pro_list(list)),
        Err(_) => Json(ProData::empty()),
    };
}

fn data_from_season_pro_list(sp_list: Vec<SPModel>) -> ProData {
    let mut game_num = 0;
    let mut kyoku_num = 0;
    let mut point = Decimal::ZERO;
    let mut rank_num = 0;
    let mut agari_num = 0;
    let mut houjuu_num = 0;
    let mut tsumo_num = 0;
    let mut dama_num = 0;
    let mut ron_num = 0;
    let mut ryukyoku_num = 0;
    let mut ryukyoku_tenpai_num = 0;
    let mut furo_num = 0;
    let mut richi_num = 0;
    let mut agari_turn = 0;
    let mut agari_score = 0;
    let mut houjuu_score = 0;
    let mut kyoku_east_num = 0;
    let mut blown_num = 0;
    let mut blown_score = 0;
    let mut first_num = 0;
    let mut second_num = 0;
    let mut third_num = 0;
    let mut fourth_num = 0;
    let mut richi_agari_num = 0;
    let mut richi_houjuu_num = 0;
    let mut richi_tsumo_num = 0;
    let mut richi_agari_score = 0;
    let mut richi_ryukyoku_num = 0;
    let mut richi_turn = 0;
    let mut richi_dora_num = 0;
    let mut richi_first_num = 0;
    let mut richi_chase_num = 0;
    let mut richi_chased_num = 0;
    let mut ippatsu_num = 0;
    let mut uradora_kyoku_num = 0;
    let mut highest_score: Option<i32> = None;
    let mut lowest_score: Option<i32> = None;

    for sp in sp_list {
        game_num += sp.game_num;
        kyoku_num += sp.kyoku_num();
        point = point.add(sp.rank_point).add(sp.score_point);
        rank_num += sp.rank_total();
        agari_num += sp.agari_num();
        houjuu_num += sp.houjuu_num();
        tsumo_num += sp.tsumo_num();
        dama_num += sp.dama_num();
        ron_num += sp.ron_num();
        ryukyoku_num += sp.ryukyoku_num();
        ryukyoku_tenpai_num += sp.ryukyoku_tenpai_num();
        furo_num += sp.furo_num;
        richi_num += sp.richi_num;
        agari_turn += sp.agari_turn_num;
        agari_score += sp.agari_score();
        houjuu_score += sp.houjuu_score();
        kyoku_east_num += sp.kyoku_east_num;
        blown_num += sp.blown_num;
        blown_score += sp.blown_score;
        first_num += sp.first_num();
        second_num += sp.second_num();
        third_num += sp.third_num();
        fourth_num += sp.fourth_num();
        richi_agari_num += sp.agari_richi_ron_num + sp.agari_richi_tsumo_num;
        richi_houjuu_num +=
            sp.houjuu_richi_furo_num + sp.houjuu_richi_menzen_num + sp.houjuu_richi_richi_num;
        richi_tsumo_num += sp.agari_richi_tsumo_num;
        richi_agari_score += sp.agari_richi_score;
        richi_ryukyoku_num += sp.ryukyoku_noten_richi_num + sp.ryukyoku_tenpai_richi_num;
        richi_turn += sp.richi_turn_num;
        richi_dora_num += sp.richi_dora_num;
        richi_first_num += sp.richi_first_num;
        richi_chase_num += sp.richi_chase_num;
        richi_chased_num += sp.richi_chased_num;
        ippatsu_num += sp.agari_richi_ron_ippatsu_num + sp.agari_richi_tsumo_ippatsu_num;
        uradora_kyoku_num +=
            sp.agari_richi_ron_uradora_kyoku_num + sp.agari_richi_tsumo_uradora_kyoku_num;

        let sp_highest_score = sp.game_highest_score.unwrap_or(25000);
        match highest_score {
            Some(score) => {
                if sp_highest_score > score {
                    highest_score = Some(sp_highest_score);
                }
            }
            None => highest_score = Some(sp_highest_score),
        }

        let sp_lowest_score = sp.game_lowest_score.unwrap_or(25000);
        match lowest_score {
            Some(score) => {
                if sp_lowest_score < score {
                    lowest_score = Some(sp_lowest_score);
                }
            }
            None => lowest_score = Some(sp_lowest_score),
        }
    }

    return ProData {
        game_num,
        kyoku_num,
        point: point.to_f64().unwrap_or(0.0),
        avg_point: point
            .checked_div(Decimal::from(game_num))
            .unwrap_or(Decimal::ZERO)
            .to_f64()
            .unwrap_or(0.0),
        avg_rank: rank_num as f64 / game_num as f64,
        agari_rate: agari_num as f64 / kyoku_num as f64,
        houjuu_rate: houjuu_num as f64 / kyoku_num as f64,
        tsumo_rate: tsumo_num as f64 / agari_num as f64,
        dama_rate: dama_num as f64 / agari_num as f64,
        ron_rate: ron_num as f64 / agari_num as f64,
        ryukyoku_rate: ryukyoku_num as f64 / kyoku_num as f64,
        ryukyoku_tenpai_rate: ryukyoku_tenpai_num as f64 / ryukyoku_num as f64,
        furo_rate: furo_num as f64 / kyoku_num as f64,
        richi_rate: richi_num as f64 / kyoku_num as f64,
        avg_agari_turn: agari_turn as f64 / agari_num as f64,
        avg_agari_score: agari_score as f64 / agari_num as f64,
        avg_houjuu_score: houjuu_score as f64 / houjuu_num as f64,
        blown_rate: blown_num as f64 / kyoku_east_num as f64,
        avg_blown_score: blown_score as f64 / blown_num as f64,
        first_rate: first_num as f64 / game_num as f64,
        second_rate: second_num as f64 / game_num as f64,
        third_rate: third_num as f64 / game_num as f64,
        fourth_rate: fourth_num as f64 / game_num as f64,
        richi_agari_rate: richi_agari_num as f64 / richi_num as f64,
        richi_houjuu_rate: richi_houjuu_num as f64 / richi_num as f64,
        richi_tsumo_rate: richi_tsumo_num as f64 / richi_agari_num as f64,
        avg_richi_agari_score: richi_agari_score as f64 / richi_agari_num as f64,
        richi_ryukyoku_rate: richi_ryukyoku_num as f64 / richi_num as f64,
        avg_richi_turn: richi_turn as f64 / richi_num as f64,
        avg_richi_dora: richi_dora_num as f64 / richi_num as f64,
        richi_first_rate: richi_first_num as f64 / richi_num as f64,
        richi_chase_rate: richi_chase_num as f64 / richi_num as f64,
        richi_chased_rate: richi_chased_num as f64 / richi_num as f64,
        ippatsu_rate: ippatsu_num as f64 / richi_agari_num as f64,
        uradora_rate: uradora_kyoku_num as f64 / richi_agari_num as f64,
        highest_score: highest_score.unwrap_or(25000),
        lowest_score: lowest_score.unwrap_or(25000),
    };
}

impl SPModel {
    fn first_num(&self) -> i32 {
        self.first_east_num + self.first_south_num + self.first_west_num + self.first_north_num
    }

    fn second_num(&self) -> i32 {
        self.second_east_num + self.second_south_num + self.second_west_num + self.second_north_num
    }

    fn third_num(&self) -> i32 {
        self.third_east_num + self.third_south_num + self.third_west_num + self.third_north_num
    }

    fn fourth_num(&self) -> i32 {
        self.fourth_east_num + self.fourth_south_num + self.fourth_west_num + self.fourth_north_num
    }

    fn rank_total(&self) -> i32 {
        self.first_num() + self.second_num() * 2 + self.third_num() * 3 + self.fourth_num() * 4
    }

    fn kyoku_num(&self) -> i32 {
        self.kyoku_east_num + self.kyoku_north_num + self.kyoku_west_num + self.kyoku_north_num
    }

    fn agari_num(&self) -> i32 {
        self.agari_dama_ron_num
            + self.agari_dama_tsumo_num
            + self.agari_furo_ron_num
            + self.agari_furo_tsumo_num
            + self.agari_richi_ron_num
            + self.agari_richi_tsumo_num
    }

    fn houjuu_num(&self) -> i32 {
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

    fn tsumo_num(&self) -> i32 {
        self.agari_dama_tsumo_num + self.agari_furo_tsumo_num + self.agari_richi_tsumo_num
    }

    fn dama_num(&self) -> i32 {
        self.agari_dama_ron_num + self.agari_dama_tsumo_num
    }

    fn ron_num(&self) -> i32 {
        self.agari_dama_ron_num + self.agari_furo_ron_num + self.agari_richi_ron_num
    }

    fn ryukyoku_tenpai_num(&self) -> i32 {
        self.ryukyoku_tenpai_menzen_num
            + self.ryukyoku_tenpai_furo_num
            + self.ryukyoku_tenpai_richi_num
    }

    fn ryukyoku_noten_num(&self) -> i32 {
        self.ryukyoku_noten_menzen_num
            + self.ryukyoku_noten_furo_num
            + self.ryukyoku_noten_richi_num
    }

    fn ryukyoku_num(&self) -> i32 {
        self.ryukyoku_noten_num() + self.ryukyoku_tenpai_num()
    }

    fn agari_score(&self) -> i32 {
        self.agari_dama_score + self.agari_furo_score + self.agari_richi_score
    }

    fn houjuu_score(&self) -> i32 {
        self.houjuu_dama_score + self.houjuu_furo_score + self.houjuu_richi_score
    }
}

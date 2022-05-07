use serde::Serialize;

#[derive(Serialize, Debug)]
pub struct Statistic {
    pub game_num: i32,
    pub kyoku_num: i32,
    pub point: f64,
    pub avg_point: f64,
    pub agari_rate: f64,
    pub houjuu_rate: f64,
    pub tsumo_rate: f64,
    pub dama_rate: f64,
    pub ryukyoku_rate: f64,
    pub ryukyoku_tenpai_rate: f64,
    pub furo_rate: f64,
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
    pub highest_score: i32,
    pub lowest_score: i32,
}

#[derive(Debug, Serialize)]
pub struct ProFloatValueItem {
    pub pro_id: i32,
    pub pro_name: String,
    pub value: f64,
}

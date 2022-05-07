use serde::ser::SerializeStruct;
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

#[derive(Debug, Serialize)]
pub struct ProIntegerValueItem {
    pub pro_id: i32,
    pub pro_name: String,
    pub value: i32,
}

#[derive(Debug)]
pub enum ProRankItem {
    Float(ProFloatValueItem),
    Integer(ProIntegerValueItem),
}

impl Serialize for ProRankItem {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            ProRankItem::Float(item) => {
                let mut state = serializer.serialize_struct("ProFloatValueItem", 3)?;
                state.serialize_field("pro_id", &item.pro_id)?;
                state.serialize_field("pro_name", &item.pro_name)?;
                state.serialize_field("value", &item.value)?;
                state.end()
            }
            ProRankItem::Integer(item) => {
                let mut state = serializer.serialize_struct("ProIntegerValueItem", 3)?;
                state.serialize_field("pro_id", &item.pro_id)?;
                state.serialize_field("pro_name", &item.pro_name)?;
                state.serialize_field("value", &item.value)?;
                state.end()
            }
        }
    }
}

use rocket::{
    serde::{Serialize, Serializer},
    State,
};
use sqlx::{Pool, Sqlite};
use std::future::Future;

#[derive(Debug, PartialEq, PartialOrd)]
pub enum Value {
    Integer(i64),
    Float(f32),
}

impl Serialize for Value {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Value::Integer(v) => serializer.serialize_i64(*v),
            Value::Float(v) => serializer.serialize_f32(*v),
        }
    }
}

pub trait ValueItem {
    fn get_value(self) -> Value;
    fn set_value(&mut self, value: Value);
}

const POINT: &str = "sum(score_point + rank_point)";
const KYOKU_NUM: &str = "sum(kyoku_east_num + kyoku_north_num + kyoku_west_num + kyoku_north_num)";
const AGARI_NUM: &str = "sum(agari_dama_ron_num + agari_dama_tsumo_num + 
    agari_furo_ron_num + agari_furo_tsumo_num + 
    agari_richi_ron_num + agari_richi_tsumo_num)";
const RICHI_NUM: &str = "sum(richi_num)";
const HOUJUU_NUM: &str =
    "sum(houjuu_dama_furo_num + houjuu_dama_menzen_num + houjuu_dama_richi_num 
    + houjuu_furo_furo_num + houjuu_furo_menzen_num + houjuu_furo_richi_num
    + houjuu_richi_furo_num + houjuu_richi_menzen_num + houjuu_richi_richi_num)";
const TSUMO_NUM: &str = "sum(agari_dama_tsumo_num + agari_furo_tsumo_num + agari_richi_tsumo_num)";
const RYUKYOKU_NUM: &str =
    "sum(ryukyoku_tenpai_menzen_num + ryukyoku_tenpai_furo_num + ryukyoku_tenpai_richi_num +
    ryukyoku_noten_menzen_num + ryukyoku_noten_furo_num + ryukyoku_noten_richi_num)";
const AGARI_RICHI_NUM: &str = "sum(agari_richi_ron_num + agari_richi_tsumo_num)";
const HOUJUU_FURO_NUM: &str =
    "sum(houjuu_dama_furo_num + houjuu_furo_furo_num + houjuu_richi_furo_num)";
const HOUJUU_RICHI_NUM: &str =
    "sum(houjuu_dama_richi_num + houjuu_furo_richi_num + houjuu_richi_richi_num)";

pub async fn rank<'r, T, FutV, FutD>(
    pool: &'r State<Pool<Sqlite>>,
    key: &'r str,
    seasons: &'r Option<Vec<i64>>,
    value_list: impl Fn(&'r State<Pool<Sqlite>>, &'r str, &'r Option<Vec<i64>>, bool) -> FutV,
    divided_list: impl Fn(&'r State<Pool<Sqlite>>, &'r str, &'r str, &'r Option<Vec<i64>>, bool) -> FutD,
) -> Vec<T>
where
    FutV: Future<Output = Vec<T>>,
    FutD: Future<Output = Vec<T>>,
    T: ValueItem,
{
    let value_list = |sql: &'r str, float: bool| value_list(pool, sql, seasons, float);
    let divided_list =
        |sql: &'r str, sql2: &'r str, float: bool| divided_list(pool, sql, sql2, seasons, float);
    match key {
        "rank_point" => value_list("sum(rank_point)", true).await,
        "score_point" => value_list("sum(score_point)", true).await,
        "game_num" => value_list("sum(game_num)", false).await,
        "point" => value_list(POINT, true).await,
        "kyoku_num" => value_list(KYOKU_NUM, false).await,
        "avg_point" => divided_list(POINT, "sum(game_num)", true).await,
        "agari_rate" => divided_list(AGARI_NUM, KYOKU_NUM, false).await,
        "houjuu_rate" => divided_list(HOUJUU_NUM, KYOKU_NUM, false).await,
        "houjuu_menzen_rate" => {
            let houjuu_menzen_num =
                "sum(houjuu_dama_menzen_num + houjuu_furo_menzen_num + houjuu_richi_menzen_num)";
            divided_list(houjuu_menzen_num, HOUJUU_NUM, false).await
        }
        "houjuu_furo_rate" => divided_list(HOUJUU_FURO_NUM, HOUJUU_NUM, false).await,
        "houjuu_richi_rate" => divided_list(HOUJUU_RICHI_NUM, HOUJUU_NUM, false).await,
        "houjuu_to_dama_rate" => {
            let houjuu_to_dama_num =
                "sum(houjuu_dama_menzen_num + houjuu_dama_furo_num + houjuu_dama_richi_num)";
            divided_list(houjuu_to_dama_num, HOUJUU_NUM, false).await
        }
        "houjuu_to_furo_rate" => {
            let houjuu_to_furo_num =
                "sum(houjuu_furo_menzen_num + houjuu_furo_furo_num + houjuu_furo_richi_num)";
            divided_list(houjuu_to_furo_num, HOUJUU_NUM, false).await
        }
        "houjuu_to_richi_rate" => {
            let houjuu_to_richi_num =
                "sum(houjuu_richi_menzen_num + houjuu_richi_furo_num + houjuu_richi_richi_num)";
            divided_list(houjuu_to_richi_num, HOUJUU_NUM, false).await
        }
        "tsumo_rate" => divided_list(TSUMO_NUM, AGARI_NUM, false).await,
        "agari_dama_rate" => {
            let agari_dama_num = "sum(agari_dama_ron_num + agari_dama_tsumo_num)";
            divided_list(agari_dama_num, AGARI_NUM, false).await
        }
        "agari_furo_rate" => {
            let agari_furo_num = "sum(agari_furo_ron_num + agari_furo_tsumo_num)";
            divided_list(agari_furo_num, AGARI_NUM, false).await
        }
        "agari_richi_rate" => {
            let agari_richi_num = "sum(agari_richi_ron_num + agari_richi_tsumo_num)";
            divided_list(agari_richi_num, AGARI_NUM, false).await
        }
        "ryukyoku_rate" => divided_list(RYUKYOKU_NUM, KYOKU_NUM, false).await,
        "ryukyoku_tenpai_rate" => {
            let ryu_ten_num = "sum(ryukyoku_tenpai_menzen_num + ryukyoku_tenpai_furo_num + ryukyoku_tenpai_richi_num)";
            divided_list(ryu_ten_num, RYUKYOKU_NUM, false).await
        }
        "furo_rate" => divided_list("sum(furo_num)", KYOKU_NUM, false).await,
        "furo_agari_rate" => {
            let agari_furo_num = "sum(agari_furo_ron_num + agari_furo_tsumo_num)";
            divided_list(agari_furo_num, "sum(furo_num)", false).await
        }
        "furo_ryukyoku_rate" => {
            let furo_ryukyoku_num = "sum(ryukyoku_noten_furo_num + ryukyoku_tenpai_furo_num)";
            divided_list(furo_ryukyoku_num, "sum(furo_num)", false).await
        }
        "furo_houjuu_rate" => divided_list(HOUJUU_FURO_NUM, "sum(furo_num)", false).await,
        "avg_furo_agari_score" => {
            let agari_furo_num = "sum(agari_furo_ron_num + agari_furo_tsumo_num)";
            divided_list("sum(agari_furo_score)", agari_furo_num, false).await
        }
        "richi_rate" => divided_list(RICHI_NUM, KYOKU_NUM, false).await,
        "avg_agari_turn" => divided_list("sum(agari_turn_num)", AGARI_NUM, false).await,
        "avg_agari_score" => {
            let agari_score = "sum(agari_dama_score + agari_furo_score + agari_richi_score)";
            divided_list(agari_score, AGARI_NUM, false).await
        }
        "avg_houjuu_score" => {
            let houjuu_score = "sum(houjuu_dama_score + houjuu_furo_score + houjuu_richi_score)";
            divided_list(houjuu_score, HOUJUU_NUM, false).await
        }
        "avg_rank" => {
            let rank_total =
                "sum(first_east_num + first_south_num + first_west_num + first_north_num +
                (second_east_num + second_south_num + second_west_num + second_north_num) * 2 +
                (third_east_num + third_south_num + third_west_num + third_north_num) * 3 +
                (fourth_east_num + fourth_south_num + fourth_west_num + fourth_north_num) * 4
            )";
            divided_list(rank_total, "sum(game_num)", false).await
        }
        "blown_rate" => divided_list("sum(blown_num)", "sum(kyoku_east_num)", false).await,
        "avg_blown_score" => divided_list("sum(blown_score)", "sum(blown_num)", false).await,
        "first_rate" => {
            let first_num =
                "sum(first_east_num + first_south_num + first_west_num + first_north_num)";
            divided_list(first_num, "sum(game_num)", false).await
        }
        "avg_first_score" => {
            let first_num =
                "sum(first_east_num + first_south_num + first_west_num + first_north_num)";
            divided_list("sum(first_score)", first_num, false).await
        }
        "second_rate" => {
            let second_num =
                "sum(second_east_num + second_south_num + second_west_num + second_north_num)";
            divided_list(second_num, "sum(game_num)", false).await
        }
        "avg_second_score" => {
            let second_num =
                "sum(second_east_num + second_south_num + second_west_num + second_north_num)";
            divided_list("sum(second_score)", second_num, false).await
        }
        "third_rate" => {
            let third_num =
                "sum(third_east_num + third_south_num + third_west_num + third_north_num)";
            divided_list(third_num, "sum(game_num)", false).await
        }
        "avg_third_score" => {
            let third_num =
                "sum(third_east_num + third_south_num + third_west_num + third_north_num)";
            divided_list("sum(third_score)", third_num, false).await
        }
        "fourth_rate" => {
            let fourth_num =
                "sum(fourth_east_num + fourth_south_num + fourth_west_num + fourth_north_num)";
            divided_list(fourth_num, "sum(game_num)", false).await
        }
        "avg_fourth_score" => {
            let fourth_num =
                "sum(fourth_east_num + fourth_south_num + fourth_west_num + fourth_north_num)";
            divided_list("sum(fourth_score)", fourth_num, false).await
        }
        "richi_agari_rate" => divided_list(AGARI_RICHI_NUM, RICHI_NUM, false).await,
        "richi_houjuu_rate" => {
            let richi_houjuu_num =
                "sum(houjuu_dama_richi_num + houjuu_furo_richi_num + houjuu_richi_richi_num)";
            divided_list(richi_houjuu_num, RICHI_NUM, false).await
        }
        "richi_tsumo_rate" => {
            divided_list("sum(agari_richi_tsumo_num)", AGARI_RICHI_NUM, false).await
        }
        "avg_richi_agari_score" => {
            divided_list("sum(agari_richi_score)", AGARI_RICHI_NUM, false).await
        }
        "richi_ryukyoku_rate" => {
            let richi_ryukyoku_num = "sum(ryukyoku_noten_richi_num + ryukyoku_tenpai_richi_num)";
            divided_list(richi_ryukyoku_num, RICHI_NUM, false).await
        }
        "avg_richi_turn" => divided_list("sum(richi_turn_num)", RICHI_NUM, false).await,
        "avg_richi_dora" => divided_list("sum(richi_dora_num)", RICHI_NUM, false).await,
        "richi_first_rate" => divided_list("sum(richi_first_num)", RICHI_NUM, false).await,
        "richi_chase_rate" => divided_list("sum(richi_chase_num)", RICHI_NUM, false).await,
        "richi_chased_rate" => divided_list("sum(richi_chased_num)", RICHI_NUM, false).await,
        "ippatsu_rate" => {
            let ippatsu_num = "sum(agari_richi_ron_ippatsu_num + agari_richi_tsumo_ippatsu_num)";
            divided_list(ippatsu_num, AGARI_RICHI_NUM, false).await
        }
        "uradora_rate" => {
            let uradora_kyoku_num = "sum(total.agari_richi_ron_uradora_kyoku_num
                + total.agari_richi_tsumo_uradora_kyoku_num)";
            divided_list(uradora_kyoku_num, AGARI_RICHI_NUM, false).await
        }
        "highest_score" => value_list("max(game_highest_score)", false).await,
        "lowest_score" => value_list("min(game_lowest_score)", false).await,
        "renchan_max_num" => value_list("max(renchan_max_num)", false).await,
        _ => vec![],
    }
}

pub mod season_pro;
pub mod statistics;

pub fn format_sql_vec(list: &[i64]) -> String {
    let str = format!("{:?}", list);
    let len = str.len();
    format!("({})", &str[1..len - 1])
}

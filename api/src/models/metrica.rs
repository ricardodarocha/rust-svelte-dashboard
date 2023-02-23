use serde::{Serialize, Deserialize};
use sqlx::{FromRow};

#[derive(Serialize, Deserialize, FromRow)] 
// #[sqlx(rename_all = "camelCase")]
pub struct QuiuMetricas {
    pub id: i32,
    pub nome: String,
    #[sqlx(rename = "valora")]
    pub valor_a:f32,
    #[sqlx(rename = "valorb")]
    pub valor_b:f32,
    #[sqlx(rename = "valorc")]
    pub valor_c:f32,
}

#[derive(Serialize, Deserialize, FromRow)] 
pub struct ListQuiuMetricas {
    pub id: i32,
    pub nome: String}

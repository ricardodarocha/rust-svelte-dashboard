use actix_web::{
    get, 
    web,
    HttpRequest,
    HttpResponse
};

  use sqlx::postgres::{PgPool};
  use crate::models::metrica::
  {QuiuMetricas, ListQuiuMetricas}; 

#[get("/metricas/")]
pub async fn get_metricas(pool: web::Data<PgPool>) -> HttpResponse {
    let metricas = sqlx::query_as::<_, ListQuiuMetricas>(
        "select distinct 
        max(id) id,
        nome
        from QUIUMETRICAS
        group by 2
        order by 2")
        .fetch_all(pool.get_ref())
        .await
        .unwrap();
    HttpResponse::Ok().json(metricas) 
}

#[get("/metrica/{name}/{qtd}")]
pub async fn get_metrica_by_name(pool: web::Data<PgPool>,
    req: HttpRequest
) -> HttpResponse {
    let name: String = req.match_info().get("name").unwrap().parse().unwrap();
    let qtd: i32 = req.match_info().query("qtd").parse().unwrap_or(30);

    let metricas = sqlx::query_as::<_, QuiuMetricas>(
        "select 
        id,
        nome,
        valora::FLOAT4,
        valorb::FLOAT4,
        valorc::FLOAT4
        from QUIUMETRICAS
        where upper(nome) = upper($1)
        order by 1 desc
        limit $2")
        .bind(name)
        .bind(qtd)
        .fetch_all(pool.get_ref())
        .await
        .unwrap();
    HttpResponse::Ok().json(metricas) 
}
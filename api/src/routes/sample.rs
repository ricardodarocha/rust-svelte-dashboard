use actix_web::{
    get, 
    // error::ResponseError,
    // Error, 
    web,
    // web::Path,
    // web::Json,
    // web::Data, 
    // App, 
    // HttpServer, 
    // HttpResponse,
    Responder,
    // http::{header::ContentType, StatusCode}
};

  use sqlx::postgres::{PgPool};
//   use sqlx::postgres::{PgPool, PgPoolOptions, PgRow};
//   use sqlx::{FromRow, Row};
  use sqlx::Row;

// use crate::models::user::{User, NewUser};

#[get("/ping")]
pub async fn ping(pool: web::Data<PgPool>) -> impl Responder {
    let row = sqlx::query("select 1 as id")
        .fetch_one(pool.get_ref())
        .await
        .unwrap();
    let id: i32 = row.try_get("id").unwrap();

    return format!("{:?}", id)
}

#[get("/pong")]
pub async fn pong(pool: web::Data<PgPool>) -> impl Responder {
    let row = sqlx::query("select 1 as id")
        .fetch_one(pool.get_ref())
        .await
        .unwrap();
    let one: i32 = row.try_get("id").unwrap();

   return format!("value: {:?}", one)
}
use actix_web::{
        get, 
        post, 
        put, 
        delete, 
        web,
        http::{StatusCode},
        HttpResponse};
use sqlx::postgres::{PgPool}; 

use crate::models::user::{
    User,
    NewUser};    

#[get("/user")]
pub async fn user(pool: web::Data<PgPool>) -> HttpResponse {
    let users = sqlx::query_as::<_, User>("select * from users")
        .fetch_all(pool.get_ref())
        .await
        .unwrap();

    HttpResponse::Ok().json(users) 
}

#[post("/user")]
pub async fn post_user(pool: web::Data<PgPool>, usr: web::Json<NewUser>) -> HttpResponse {
    let usercreated = sqlx::query_as::<_, User>(
        "insert into users
            (nome, email, senha) 
            VALUES ($1, $2, $3)
            returning id, nome, email, senha")
            .bind(usr.nome.to_owned())
            .bind(usr.email.to_owned())
            .bind(usr.senha.to_owned())
        .fetch_one(pool.get_ref())
        .await
        .unwrap();
        
      HttpResponse::Ok()
      .status(StatusCode::CREATED)
      .json(usercreated)
}

#[put("/user/{id}")]
pub async fn update_user(pool: web::Data<PgPool>, usr: web::Json<User>) -> HttpResponse {
     let user_saved = sqlx::query_as::<_, User>(
        "update users set 
            nome = $2, 
            email = $3, 
            senha = $4
            where id = $1
            returning id, nome, email, senha")
            .bind(usr.id.to_owned())
            .bind(usr.nome.to_owned())
            .bind(usr.email.to_owned())
            .bind(usr.senha.to_owned())
        .fetch_one(pool.get_ref())
        .await
        .unwrap();
        
      HttpResponse::Ok()
      .status(StatusCode::CREATED)
      .json(user_saved)
}

#[delete("/user/{id}")]
pub async fn delete_user(pool: web::Data<PgPool>, usr: web::Json<User>) -> HttpResponse {
    sqlx::query("delete from users where id = ?")
    .bind(usr.id.to_owned())
    .execute(pool.get_ref())
    .await
    .unwrap();

    HttpResponse::Ok().body(format!("user {} has deleted", usr.id))
}


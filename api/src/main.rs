use clap::Parser;
use actix_web::{
    App, 
    HttpServer,    
    middleware::Logger,
    // http::{header::ContentType, StatusCode}
};
use sqlx::postgres::{PgPool};
use actix_cors::Cors;

mod routes;
mod models;

use routes::sample::*;

use routes::user::*;

use routes::metrica::*;

///   🔮 dash api -  🐧 Pinguins (api v1 2022)
/// Esta api retorna as últimas medições de \nqualquer parâmetro disponível
/// Para configurar as métricas acesse monitor.exe

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]

// Args é usado para representar os parâmetros que podem ser passados pelo CMD
struct Args {

    /// Usuário
    #[clap(short, long, value_parser, default_value = "postgres")]
    usuario: String,

    /// Senha
    #[clap(short, long, value_parser, default_value = "masterkey")]
    senha: String,

    /// Host 
    #[clap(short, long, value_parser, default_value = "localhost")]
    host: String,

    /// Database Name 
    #[clap(short, long, value_parser, default_value = "quiu")]
    database: String,

    /// Porta do Banco de Dados
    #[clap(short, long, value_parser, default_value_t = 5432)]
    porta: u16,

    /// Porta HTTP do Servidor
    #[clap(short('a'), long, value_parser, default_value_t = 3001)]
    porta_serv: u16,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    // ativa o log no modo DEBUG
    std::env::set_var("RUST_LOG", "debug");
    
    // ativa o log detalhado
    // std::env::set_var("RUST_BACKTRACE", "1");

    env_logger::init();

    let args = Args::parse(); 
    let string_conexao = format!(
        "postgres://{}:{}@{}:{}/{}",
        args.usuario, args.senha, args.host, args.porta, args.database
        ); 
    let pool = PgPool::connect(&string_conexao)
        .await
        .expect("⚠️  Não foi possível iniciar o Pool de conexão ");

        println!("Running on port ... http://localhost:{}", args.porta_serv);

        HttpServer::new( move || {
        let logger = Logger::default();
        let cors = Cors::default()
          .allow_any_origin();

        App::new()
        .data( pool.clone() )
        .wrap(logger)
        .wrap(cors)
        .service(ping)
        .service(pong)
        .service(user)
        .service(post_user)
        .service(get_metricas)
        .service(get_metrica_by_name)
    })
    .bind(("localhost", args.porta_serv))?
    .run()
    .await
}
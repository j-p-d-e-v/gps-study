use actix_web::{
    http::StatusCode, rt, web, App, Error, HttpRequest, HttpResponse, HttpServer, Responder,
};
use actix_ws::{AggregatedMessage, Session};
use actix_ws::{CloseCode, CloseReason};
use clap::Parser;
use futures_util::StreamExt as _;
use gps_tracker::actions::CoordinatesData;
use gps_tracker::config::{Config, WebConfig};
use gps_tracker::db::Db;
use gps_tracker::user::User;
use serde::{Deserialize, Serialize};
use surrealdb::sql::Datetime;

#[derive(Debug, Clone, Parser, Serialize, Deserialize)]
pub struct Args {
    #[arg(short, long)]
    pub config_path: String,
}

async fn users() -> impl Responder {
    match User::new().await {
        Ok(user) => match user.get_users().await {
            Ok(data) => HttpResponse::Ok().json(data),
            Err(error) => HttpResponse::build(StatusCode::BAD_REQUEST).body(error),
        },
        Err(error) => HttpResponse::build(StatusCode::BAD_REQUEST).body(error),
    }
}

#[derive(Debug, Serialize)]
pub struct Data {
    pub user_id: String,
    pub lat: f64,
    pub lon: f64,
    pub timestamp: Datetime,
}

async fn close_session_with_error(session: Session, error: String) {
    if let Err(error) = session
        .close(Some(CloseReason {
            code: CloseCode::Error,
            description: Some(error.to_string()),
        }))
        .await
    {
        eprintln!("SESSION ERROR: {:?}", error);
    }
}

async fn ws(req: HttpRequest, stream: web::Payload) -> Result<HttpResponse, Error> {
    let (res, session, stream) = actix_ws::handle(&req, stream)?;

    let mut stream = stream.aggregate_continuations().max_continuation_size(1024);

    rt::spawn(async move {
        while let Some(msg) = stream.next().await {
            match msg {
                Ok(AggregatedMessage::Text(_)) => match Db::connect().await {
                    Ok(db) => {
                        match db
                            .client
                            .select::<Vec<CoordinatesData>>("coordinates")
                            .live()
                            .await
                        {
                            Ok(mut coords_stream) => {
                                let _session = session.clone();

                                while let Some(result) = coords_stream.next().await {
                                    if let Ok(item) = result {
                                        let data = item.data;
                                        match serde_json::to_string(&Data {
                                            user_id: data.user.to_string(),
                                            lat: data.latitude,
                                            lon: data.longitude,
                                            timestamp: data.timestamp,
                                        }) {
                                            Ok(value) => {
                                                if let Err(error) =
                                                    _session.clone().text(value).await
                                                {
                                                    eprintln!("SENDING ERROR: {:?}", error);
                                                    break;
                                                }
                                            }
                                            Err(error) => {
                                                close_session_with_error(
                                                    _session,
                                                    error.to_string(),
                                                )
                                                .await;
                                                break;
                                            }
                                        }
                                    }
                                }
                            }
                            Err(error) => {
                                close_session_with_error(session, error.to_string()).await;
                                break;
                            }
                        }
                    }
                    Err(error) => {
                        close_session_with_error(session, error).await;
                        break;
                    }
                },
                Ok(AggregatedMessage::Close(msg)) => {
                    println!("CLOSED WS:{:?}", msg);
                    if let Err(error) = session.close(msg).await {
                        eprintln!("SESSION CLOSED ERROR: {:?}", error)
                    }
                    break;
                }
                _ => {}
            }
        }
    });

    Ok(res)
}

fn app_config(config: &mut web::ServiceConfig) {
    config
        .service(web::resource("/users").get(users))
        .service(web::resource("/ws").get(ws));
}

#[actix_web::main]
async fn main() -> Result<(), String> {
    let args = Args::parse();
    let config_path = args.config_path;
    match Config::load(Some(config_path)).await {
        Ok(config) => {
            let web_config: WebConfig = config.web;
            println!(
                "Web Server Address: {}:{}",
                &web_config.host, &web_config.port
            );
            match HttpServer::new(move || App::new().configure(app_config))
                .workers(4)
                .bind((web_config.host, web_config.port as u16))
            {
                Ok(server) => {
                    if let Err(error) = server.run().await {
                        return Err(error.to_string());
                    }
                    Ok(())
                }
                Err(error) => Err(error.to_string()),
            }
        }
        Err(error) => Err(error),
    }
}

#[cfg(test)]
mod test_server {

    use super::*;
    use actix_web::test;
    use std::env;
    #[actix_web::test]
    async fn test_users() {
        env::set_var("APP_CONFIG_PATH", "../config.toml");
        let app = test::init_service(App::new().configure(app_config)).await;
        let req = test::TestRequest::get().uri("/users").to_request();

        let resp = test::call_service(&app, req).await;

        assert_eq!(resp.status(), StatusCode::OK);
    }
}

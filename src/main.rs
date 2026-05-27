use axum::{routing::{get}, Router};
use std::env;
use dotenv::dotenv;
use tokio::net::TcpListener;
use tower_http::{cors::CorsLayer, services::ServeDir};

mod handlers;
mod models;
mod state;

use state::AppState;
use handlers::{root, get_movie_videos, get_tranding_movies, search_content};


#[tokio::main]
pub async fn main(){
    dotenv().ok();
    let api_key= env::var("TMDB_API_KEY").expect("TMDB_API_KEY not set");

    let state = AppState{
        tmbd_api_key: api_key,
        client:reqwest::Client::new(),
    };

    let cors = CorsLayer::new().allow_origin(tower_http::cors::Any);

    let app = Router::new()
        .route("/", get(root))
        .route("/api/trending", get(get_tranding_movies))
        .route("/api/search", get(search_content))
        .route("/api/movie/{id}/videos", get(get_movie_videos))
        .nest_service(
            "/stream",ServeDir::new("assets")    
        )
        .layer(cors)
        .with_state(state);

    let listner = TcpListener::bind("0.0.0.0:8080").await.unwrap();
    println!("server is listning on http://{}",listner.local_addr().unwrap());

    axum::serve(listner,app).await.unwrap();
}
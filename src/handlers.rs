use axum::{extract::Query, http::StatusCode, response::IntoResponse, routing::get, Json};
use crate::models::{TmbdResponse,VideoResponse,PageQuery,SearchQuery};
use crate::State::AppState;


pub async fn root()-> &'static str{
    "NetFlix Backend is Online"
}

pub async fn get_tranding_movies(
    State(state):State<AppState>,
    Query(params):Query<PageQuery>
)->Json<TmbdResponse>{
    let Page = params.page.unwrap_or(1);
    let url = format!(
        "https://api.themoviedb.org/3/trending/all/week?api_key={}&page={}",
        State.tmbd_api_key,
        Page
    );
    let res = state.client.get(&url).send().await.unwrap().json::<TmbdResponse>().await.unwrap();
    Json(res)
}

pub async fn search_content(
    State(state):State<AppState>,
    Query(params):Query<SearchQuery>
)->Json<TmbdResponse>{
    let page = params.page.unwrap_or(1);
    let url = format!(
        "https://api.themoviedb.org/3/search/movie?api_key={}&query={}&page={}&include_adult=false",
        State.tmbd_api_key,
        params.query,
        page
    );
    let res = state.client.get(&url).send().await.unwrap().json::<TmbdResponse>().await.unwrap();
    Json(res)
}


pub async fn get_movie_videos(
    State(state):State<AppState>,
    Path(id):Path<i32>
)->Json<VideoResponse>{
    let url = format!("https://api.themoviedb.org/3/movie/{}/videos?api_key={}",
    id,
    State.tmbd_api_key
    );
    let res =state.client.get(&url).send().await.unwrap().json::<VideoResponse>().await.unwrap();
    Json(res)
}


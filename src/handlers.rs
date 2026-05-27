use axum::{ extract::{ Path, Query, State }, Json ,http::StatusCode};
use crate::models::{ TmbdResponse, VideoResponse, PageQuery, SearchQuery };
use crate::state::AppState;


pub async fn root()-> &'static str{
    "NetFlix Backend is Online"
}

pub async fn get_tranding_movies(
    State(state):State<AppState>,
    Query(params):Query<PageQuery>
)->Json<TmbdResponse>{
    let page = params.page.unwrap_or(1);
    let url = format!(
        "https://api.themoviedb.org/3/trending/all/week?api_key={}&page={}",
        state.tmbd_api_key,
        page
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
        state.tmbd_api_key,
        params.query,
        page
    );
    let res = state.client.get(&url).send().await.unwrap().json::<TmbdResponse>().await.unwrap();
    Json(res)
}



pub async fn get_movie_videos(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<Json<VideoResponse>, StatusCode> {

    let url = format!(
        "https://api.themoviedb.org/3/movie/{}/videos?api_key={}",
        id,
        state.tmbd_api_key
    );

    let response = state.client
        .get(&url)
        .send()
        .await
        .map_err(|e| {
            println!("request error: {:?}", e);
            StatusCode::BAD_GATEWAY
        })?;

    if !response.status().is_success() {
        println!("tmdb returned error status: {}", response.status());
        return Err(StatusCode::NOT_FOUND);
    }

    let data = response
        .json::<VideoResponse>()
        .await
        .map_err(|e| {
            println!("decode error: {:?}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    Ok(Json(data))
}

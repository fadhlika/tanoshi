use crate::auth::Claims;
use crate::favorites::{favorites::Favorites, FavoriteManga};
use sqlx::postgres::PgPool;
use std::convert::Infallible;
use std::sync::{Arc, Mutex};

pub async fn get_favorites(
    claim: Claims,
    fav: Favorites,
    db: PgPool,
) -> Result<impl warp::Reply, Infallible> {
    let res = fav.get_favorites(claim.sub, db).await;
    Ok(warp::reply::json(&res))
}

pub async fn add_favorites(
    claim: Claims,
    manga: FavoriteManga,
    fav: Favorites,
    db: PgPool,
) -> Result<impl warp::Reply, Infallible> {
    let res = fav.add_favorite(claim.sub, manga, db).await;
    Ok(warp::reply::json(&res))
}

pub async fn remove_favorites(
    source: String,
    title: String,
    claim: Claims,
    fav: Favorites,
    db: PgPool,
) -> Result<impl warp::Reply, Infallible> {
    let title =
        String::from_utf8(base64::decode_config(title, base64::URL_SAFE_NO_PAD).unwrap()).unwrap();
    let res = fav.remove_favorites(claim.sub, source, title, db).await;
    Ok(warp::reply::json(&res))
}
use super::model;
use crate::data::{DataError, DbPool};
use crate::ShortCode;
use sqlx::Row;
use crate::data::model::GetClip;

type Result<T> = std::result::Result<T, DataError>;

pub async fn get_clip<M: Into<GetClip>>(model: M, pool: &DbPool) -> Result<model::Clip> {
    let model = model.into();
    let shortcode = model.shortcode.as_str();
    Ok(sqlx::query_as!(model::Clip, "SELECT * FROM clips WHERE shortcode = ?", shortcode).fetch_one(pool).await?)
}

// using a model as param to avoid having to pass a whole bunch of clip properties as params
pub async fn new_clip<M: Into<model::NewClip>>(model: M, pool: &DbPool) -> Result<model::Clip> {
    let model = model.into();
    let _ = sqlx::query!(
        r#"INSERT INTO clips (
            clip_id,
            shortcode,
            content,
            title,
            posted,
            expires,
            password,
            hits)
        VALUES(?, ?, ?, ?, ?, ?, ?, ?)"#,
        model.clip_id,
        model.shortcode,
        model.content,
        model.title,
        model.posted,
        model.expires,
        model.password,
        0)
        .execute(pool)
        .await?;

    get_clip(model.shortcode, pool).await
}

pub async fn update_clip<M: Into<model::UpdateClip>>(model: M, pool: &DbPool) -> Result<model::Clip> {
    let model = model.into();
    let _ = sqlx::query!(
        r#"UPDATE clips SET
                content = ?,
                expires = ?,
                password = ?,
                title = ?
           WHERE shortcode = ?"#,
        model.content,
        model.expires,
        model.password,
        model.content,
        model.shortcode
        )
        .execute(pool)
        .await?;

    get_clip(model.shortcode, pool).await
}
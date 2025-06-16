use aws_sdk_s3::primitives::ByteStream;
use axum::{
    body::Body,
    extract::{Multipart, Path, State},
    http::{header, HeaderMap, HeaderValue, StatusCode},
    response::IntoResponse,
    Json,
};
use std::sync::Arc;
use tokio_util::io::ReaderStream;
use uuid::Uuid;

use crate::{
    error::Error,
    model::{MediaModel, UpdateMediaSchema},
    AppState, Auth, MEDIA_BUCKET,
};

pub(crate) async fn create_media_handler(
    header: HeaderMap,
    State(data): State<Arc<AppState>>,
    mut multipart: Multipart,
) -> crate::Result<impl IntoResponse> {
    let owner = Auth::decode_header(&data.signing_key, header)?;

    while let Some(field) = multipart
        .next_field()
        .await
        .map_err(|_| Error::new(StatusCode::BAD_REQUEST, "invalid multipart"))?
    {
        if field.name().is_some_and(|n| n == "file") {
            let content_type = field
                .content_type()
                .unwrap_or("application/octet-stream")
                .to_string();
            let bytes = field.bytes().await?;
            let stream = ByteStream::from(bytes);

            let media = MediaModel::create(&data.db, owner).await?;
            let key = media.id.to_string();

            let obj = data
                .s3
                .put_object()
                .bucket(MEDIA_BUCKET)
                .key(&key)
                .body(stream)
                .content_type(&content_type)
                .send()
                .await?;

            let media = media
                .update(
                    &data.db,
                    owner,
                    UpdateMediaSchema {
                        size: obj.size(),
                        content_type: Some(content_type),
                    },
                )
                .await?;
            return Ok(Json(media));
        }
    }

    Err(Error::new(
        StatusCode::INTERNAL_SERVER_ERROR,
        "failed to upload media",
    ))
}

pub(crate) async fn get_media_handler(
    header: HeaderMap,
    Path(media_id): Path<Uuid>,
    State(data): State<Arc<AppState>>,
) -> crate::Result<impl IntoResponse> {
    let owner = Auth::decode_header(&data.signing_key, header)?;
    let media = MediaModel::retrieve(&data.db, owner, media_id).await?;

    let reader = data
        .s3
        .get_object()
        .bucket(MEDIA_BUCKET)
        .key(media.id)
        .send()
        .await?
        .body
        .into_async_read();

    let stream = ReaderStream::new(reader);
    let body = Body::from_stream(stream);

    let mut headers = HeaderMap::new();
    headers.insert(
        header::CONTENT_TYPE,
        HeaderValue::from_str(&media.content_type)?,
    );

    Ok((headers, body))
}

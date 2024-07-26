use axum::{
    http::StatusCode,
    response::{IntoResponse, Response as AxumResponse},
};

#[derive(Debug)]
pub struct AppError(anyhow::Error);

// anyhow::Error => AppError への型変換
impl<E> From<E> for AppError
where
    E: Into<anyhow::Error>,
{
    fn from(err: E) -> Self {
        Self(err.into())
    }
}

// AppError => axum::response::Response への型変換
// 自動的に、500 Internal Server Error になるようにハンドリング
impl IntoResponse for AppError {
    fn into_response(self) -> AxumResponse {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Internal Server Error: {}", self.0),
        )
            .into_response()
    }
}
impl std::fmt::Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

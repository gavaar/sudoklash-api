use actix_web::{HttpResponse, HttpResponseBuilder};
use serde::Serialize;

#[derive(Serialize)]
struct ErrorBody {
  pub status: u16,
  pub message: String,
}

pub enum ErrorResponse {
  // NotFound(String),
  Unauthorized(String),
  BadGateway(String),
}
impl ErrorResponse {
  fn error_builder(&self) -> (u16, HttpResponseBuilder, String) {
    match self {
      ErrorResponse::Unauthorized(message) => (401, HttpResponse::Unauthorized(), message.clone()),
      // ErrorResponse::NotFound(message) => (404, HttpResponse::NotFound(), message.to_owned()),
      ErrorResponse::BadGateway(message) => (502, HttpResponse::BadGateway(), message.to_owned()),
    }
  }

  pub fn throw(&self) -> HttpResponse {
    let (status, mut response_builder, message) = self.error_builder();
    let error_body = ErrorBody { status, message };

    response_builder.json(error_body)
  }
}

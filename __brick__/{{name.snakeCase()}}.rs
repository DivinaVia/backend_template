use chrono::Utc;
use mongodb::Collection;
use salvo::{
    oapi::{endpoint, extract::JsonBody},
    prelude::*,
};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, PgPool};

use crate::{
    config::{api_response::render_response, claims::get_user_claims}, feature::audit::audit::Audit, DB_POOL, MONGO_POOL
};
use salvo::http::{StatusCode, StatusError};

/* -------------------------------------------------------------------------- */
/*                                   Router                                   */
/* -------------------------------------------------------------------------- */

pub fn router() -> Router {
    Router::with_path('{{path.pathCase()}}').{{method}}(execute)
}

/* -------------------------------------------------------------------------- */
/*                                Query Result                                */
/* -------------------------------------------------------------------------- */

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Audit{{name}} {
    pub signal: String,
    pub initial_code: i32,
    pub final_code: i32,
}

/* -------------------------------------------------------------------------- */
/*                               Function Query                               */
/* -------------------------------------------------------------------------- */



/* -------------------------------------------------------------------------- */
/*                                Request Model                               */
/* -------------------------------------------------------------------------- */

#[derive(Debug, Clone, FromRow, Serialize, Deserialize, ToSchema)]
pub struct Request{{name}} {}

/* -------------------------------------------------------------------------- */
/*                               Response Model                               */
/* -------------------------------------------------------------------------- */

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, ToResponse)]
struct Response{{name}} {}

/* -------------------------------------------------------------------------- */
/*                                  Endpoint                                  */
/* -------------------------------------------------------------------------- */

#[endpoint(
  status_codes(200, 500, 401, 404, 400), 
  responses(
      (status_code = 200, response = Response{{name}})
  ),
  tags("{{name}}"), 
  security(["bearer" = ["bearer"]])
)]
pub async fn execute(depot: &Depot, res: &mut Response, request: JsonBody<Request{{name}}>) -> Result<StatusCode, StatusError> {
  let claims = get_user_claims(depot)?;
  let connection_pool = DB_POOL.get().unwrap();
  let mongo_client = MONGO_POOL.get().unwrap();

  render_response(res, json!({}));
  Ok(StatusCode::OK)
}
use salvo::{
    oapi::{endpoint, extract::JsonBody},
    prelude::*,
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use sqlx::FromRow;

use crate::{
    config::{api_response::{render_response, render_response_error}, claims::get_user_claims}, DB_POOL, MONGO_POOL
};
use salvo::http::StatusCode;

/* -------------------------------------------------------------------------- */
/*                                   Router                                   */
/* -------------------------------------------------------------------------- */

pub fn router() -> Router {
    Router::with_path("{{path}}").{{method.lowerCase()}}(execute)
}

/* -------------------------------------------------------------------------- */
/*                               Function Query                               */
/* -------------------------------------------------------------------------- */


/* -------------------------------------------------------------------------- */
/*                               Response Model                               */
/* -------------------------------------------------------------------------- */

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, ToResponse)]
struct Response{{name.pascalCase()}} {}

/* -------------------------------------------------------------------------- */
/*                                  Endpoint                                  */
/* -------------------------------------------------------------------------- */

#[endpoint(
    status_codes(200, 500, 401, 404, 400), 
    responses(
        (status_code = 200, response = Response{{name.pascalCase()}})
    ),
    tags("{{name}}"), 
    security(["bearer" = ["bearer"]])
)]
pub async fn execute(
    depot: &Depot,
    req: &mut Request,
    res: &mut Response,
) -> Result<StatusCode, StatusError> {
    let _claims = match get_user_claims(depot) {
        Ok(claims) => claims,
        Err(err) => {
            render_response_error(res, "{{name.snakeCase()}}_0001".to_string(), err.brief);
            return Err(StatusCode::FORBIDDEN);
        }
    };
    let connection_pool = DB_POOL.get().unwrap();
    let mongo_client = MONGO_POOL.get().unwrap();

    render_response(res, json!({}));
    Ok(StatusCode::OK)
}
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
/*                                Request Model                               */
/* -------------------------------------------------------------------------- */

#[derive(Serialize, Deserialize, Debug, ToSchema, Extractible)]
#[salvo(extract(default_source(from = "body")))]
struct Request{{name.pascalCase()}} {}

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
    // -> Body - JSON
    // request_body=Request{{name.pascalCase()}},
    // -> Parameters
    // parameters(
    //     ("event_id" = i32, Query, description = "O código do evento")
    // ),
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

    // let body: RequestBody = match req
    //     .extract()
    //     .await
    //     .map_err(|_| StatusError::bad_request().brief("Erro ao extrair corpo da requisição"))
    // {
    //     Ok(e) => e,
    //     Err(err) => {
    //         render_response_error(res, "{{name.snakeCase()}}_0002".to_string(), err.brief);
    //         return Err(StatusCode::INTERNAL_SERVER_ERROR);
    //     }
    // };


    // let event_id: i32 = match req
    //     .query::<i32>("event_id")
    //     .ok_or_else(|| StatusError::bad_request().brief("Missing parameter: event_id")) 
    //     {
    //     Ok(e) => e,
    //         Err(err) => {
    //             render_response_error(res, "items_0002".to_string(), err.brief);
    //             return Err(StatusCode::INTERNAL_SERVER_ERROR);
    //         }
    //     };

    render_response(res, json!({}));
    Ok(StatusCode::OK)
}
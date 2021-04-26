use crate::models::user::*;
use crate::validation::Validate;
use actix_web::{get, post, web, HttpResponse, Responder};

#[post("/register")]
/// User registration route
pub async fn register(
    pool: web::Data<sqlx::PgPool>,
    _: web::Data<r2d2::Pool<redis::Client>>,
    body: web::Json<UserInsert>,
) -> impl Responder {
    let user = body.into_inner();
    if let Some(e) = user.validate() {
        return HttpResponse::BadRequest().json(e.to_error());
    }

    HttpResponse::Ok().json(match User::insert(pool.into_inner().as_ref(), user).await {
        Ok(u) => u,
        Err(err) => return HttpResponse::InternalServerError().json(err),
    })
}

use crate::errors::{
    internal_server::{ErrorVariants, InternalServerError},
    Error, ErrorCategory,
};
use crate::models::user::*;
use crate::validation::Validate;
use actix_web::{get, post, web, HttpResponse, Responder};

#[post("/register")]
pub async fn register(
    pool: web::Data<sqlx::PgPool>,
    _: web::Data<r2d2::Pool<redis::Client>>,
    body: web::Json<UserInsert>,
) -> impl Responder {
    let user = body.into_inner();
    if let Some(e) = user.validate() {
        return HttpResponse::BadRequest()
            .json(Error::from_category(ErrorCategory::ValidationError(e)));
    }

    HttpResponse::Ok().json(match User::insert(pool.into_inner().as_ref(), user).await {
        Ok(u) => u,
        Err(_) => {
            return HttpResponse::InternalServerError().json(ErrorCategory::InternalServerError(
                InternalServerError::from_variant(ErrorVariants::DBError),
            ))
        }
    })
}

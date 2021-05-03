use super::helpers::auth::*;
use crate::models::user::*;
use crate::validation::Validate;
use actix_web::{cookie::Cookie, get, post, web, HttpResponse, Responder};
use time::Duration;

#[post("/register")]
/// User registration route
pub async fn register(
    pool: web::Data<sqlx::PgPool>,
    _: web::Data<r2d2::Pool<redis::Client>>,
    body: web::Json<UserInsert>,
) -> impl Responder {
    // Validate request body
    let user = body.into_inner();
    if let Some(e) = user.validate() {
        return HttpResponse::BadRequest().json(e.to_error());
    }
    // Insert the `User` object into the database
    HttpResponse::Ok().json(match User::insert(pool.into_inner().as_ref(), user).await {
        Ok(u) => u,
        Err(err) => return HttpResponse::InternalServerError().json(err),
    })
}

#[post("/login")]
/// User login route
pub async fn login(
    pool: web::Data<sqlx::PgPool>,
    redis_pool: web::Data<r2d2::Pool<redis::Client>>,
    req: web::HttpRequest,
    body: web::Json<UserLogin>,
) -> impl Responder {
    // Validate request body
    let userlogin = body.into_inner();
    if let Some(e) = userlogin.validate() {
        return HttpResponse::BadRequest().json(e.to_error());
    }
    // Get `User` with a username of `userlogin.username`
    let user = User::get_by_username(pool.into_inner().as_ref(), userlogin.username).await;
    if let Err(e) = user {
        return match e.error.kind {
            "AuthError" => HttpResponse::NotFound().json(e),
            "InternalServerError" => HttpResponse::InternalServerError().json(e),
            _ => HttpResponse::InternalServerError().json(()),
        };
    }
    // Check given password against password hash
    let user = user.unwrap();
    if let Some(e) = check_password_hash(userlogin.password, &user.password) {
        return HttpResponse::Unauthorized().json(e);
    }

    let useragent = req.headers().get("User-Agent").unwrap().to_str().unwrap();
    // Construct session (from user's id and useragent) and get the session/refresh token
    let refresh_token =
        UserSession::new(user.id, useragent).set_session(redis_pool.into_inner().as_ref());
    let refresh_token_cookie = Cookie::build("refresh_token", refresh_token)
        .path("/")
        .max_age(Duration::days(1))
        .finish();
    // Generate access token string from `user` object
    let access_token = UserClaims::from_user(user).to_token();
    let access_token_cookie = Cookie::build("access_token", access_token)
        .path("/")
        .max_age(Duration::minutes(20))
        .finish();
    // Return `200` response with both cookies and a success message
    HttpResponse::Ok()
        .cookie(refresh_token_cookie)
        .cookie(access_token_cookie)
        .json(SuccessMessage {
            message: "Successfully Logged in",
        })
}

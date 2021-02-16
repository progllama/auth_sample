use actix_identity::{Identity, CookieIdentityPolicy, IdentityService};
use actix_web::{middleware, App, HttpResponse, web::Form, HttpServer, get, post};
use actix_redis::RedisSession;
use actix_session::Session;
use rand::Rng;

mod structs;
mod consts;
mod utils;

use structs::*;
use utils::*;

#[get("/")]
async fn index(id: Identity, session: Session) -> HttpResponse {

    let auth_token = session.get::<String>(consts::AUTH_TOKEN).ok();
    
    if auth_token == None || id.identity() == None {
        return HttpResponse::Ok().body(consts::INDEX_PAGE.replace("{message}", "plz login or signup!"));
    }

    HttpResponse::Ok()
        .body(consts::INDEX_PAGE.replace(
                "{message}",
                session.get::<String>("USER_NAME").ok().unwrap().unwrap().as_str()
            )
        )
}

#[get("/signup")]
async fn get_signup_page() -> HttpResponse {
    HttpResponse::Ok().body(consts::SIGNUP_PAGE)
}

#[post("/signup")]
async fn signup(form: Form<SignupForm>) -> HttpResponse {
    let form = form.into_inner();

    // check duplication.
    let all_users = get_users();

    let name_duplication = all_users.iter().any(|user| { user.name == form.name });
    let email_duplication = all_users.iter().any(|user| { user.email  == form.email });

    println!("--------------------------\n{:#?}\n{:#?}\n{:#?}", name_duplication, email_duplication, all_users);

    if name_duplication || email_duplication {
        return redirect_to("signup");
    }

    // if no duplication save user.
    save_user(User {
        name: form.name,
        email: form.email,
        hash: hash(form.password),
    });
    redirect_to("/login")
}

#[get("/login")]
async fn get_login_page() -> HttpResponse {
    HttpResponse::Ok().body(consts::LOGIN_PAGE)
}

#[post("/login")]
async fn login(id: Identity, session: Session, form: Form<LoginForm>) -> HttpResponse {
    let form = form.into_inner();

    match find_user(form.email) {
        None => return redirect_to("/signup"),
        Some(user) => {
            if user.hash == hash(form.password) {
                let token = publish_token();
                let _ = session.set(consts::AUTH_TOKEN, token.clone());
                let _ = session.set("USER_NAME", user.name);
                id.remember(token.clone());
            }
        }
    }
    redirect_to("/")
}

#[post("/logout")]
async fn logout(id: Identity, session: Session) -> HttpResponse {
    id.forget();
    session.clear();
    redirect_to("/")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    let private_key = rand::thread_rng().gen::<[u8; 32]>();
    HttpServer::new(move || {
        App::new()
            .wrap(IdentityService::new(
                CookieIdentityPolicy::new(&private_key)
                    .name("auth-exsample")
                    .secure(false),
        ))
        .wrap(middleware::Logger::default())
        .wrap(RedisSession::new("127.0.0.1:6379", &private_key))
        .service(index)
        .service(get_signup_page)
        .service(signup)
        .service(get_login_page)
        .service(login)
        .service(logout)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
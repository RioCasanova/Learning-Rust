use actix_web::{Error, web, App, HttpResponse, HttpServer, Responder};
use actix_web::error::ErrorNotFound;
use serde::{Deserialize as dser, Serialize as ser};
use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

#[derive(dser, ser)]
struct User {
    id: i32,
    name: String,
}

#[derive(ser, dser)]
struct UserResponse {
    id: i32,
    name: String,
}

type UserDB = Arc<Mutex<HashMap<i32, User>>>;

// An actix-web macro that defines a route for a GET request to the /greet path
// this is an endpoint
#[actix_web::get("/users/{id}")]
async fn get_user(user_id: web::Path<i32>, db: web::Data<UserDB>) -> Result<impl Responder, Error> {
    let user_id = user_id.into_inner();
    let db = db.lock().unwrap();

    match db.get(&user_id) {
        Some(user) => Ok(HttpResponse::Ok().json(user)),
        None => Err(ErrorNotFound("User not found")),
    }
}

#[actix_web::post("/create_user")]
async fn create_user(user_data: web::Json<User>, db: web::Data<UserDB>) -> impl Responder {
    let mut db = db.lock().unwrap();
    let user_id = db.keys().max().unwrap_or(&0) + 1;
    let name = user_data.name.clone();
    db.insert(user_id, user_data.into_inner());
    HttpResponse::Created().json(UserResponse { id: user_id, name })
}

// This actix-web attribute macro allows us to use main as an async function
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let port = 8080;
    println!("Server started at http://localhost:{}", port);

    // calling outside the closure so multiple threads or endpoints can access it
    let user_db: UserDB = Arc::new(Mutex::new(HashMap::<i32, User>::new()));

    // in the closure we are configuring our application
    HttpServer::new(move || {
        let app_data = web::Data::new(user_db.clone());
        App::new().app_data(app_data).service(get_user).service(create_user)
    }) // implementation of our endpoint in the httpserver
    .bind(("127.0.0.1", port))? // when we do this we need to update the return type of main to std::io::Result<()>
    .workers(2) // if not specified, will default to the number of logical CPUs
                // 'workers' is equal to the number of OS threads that will be used to accept connections
    .run()
    .await
}

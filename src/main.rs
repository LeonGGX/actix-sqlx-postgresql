// src/main

// import actix_web
use actix_web::{
    web,
    App,
    HttpServer,
    middleware::{Logger},
};

use env_logger;

// les différents modules qui correspondent aux sous-dossiers
mod db;
mod models;
mod handlers;
//mod errors;


///
/// la fonction main
/// avec la macro actix_rt::main
/// qui est le runtime actix
///
#[actix_rt::main]
async fn main() -> std::io::Result<()> {

    // une fonction set_var qui permet de définir ce qui apparaît dans la console
    // ici le journal RUST LOG
    // les infos provenant du serveur, de actix_web et actix_http
    // puis on lance avec env_logger::init()
    std::env::set_var("RUST_LOG", "actix_server=info,actix_web=info,actix_http=trace");
    env_logger::init();

    // initialisation de la connection avec la base de données postgresql
    // codée en dur parce que dotenv n'a pas l'air de fonctionner
    let database_url = "postgres://postgres:1922leon@localhost/persons";
    let new_pool = db::connect(database_url).await.unwrap();
    println!("Connection ouverte !");

    HttpServer::new(  move || {
        App::new()
            .wrap(Logger::default())
            .data(new_pool.clone())
            .route("/", web::get().to(handlers::index))
            .configure(handlers::init) // init persons routes

    })
        .workers(2)
        .bind("127.0.0.1:8088")?
        .run()
        .await
}

///
/// les tests
/// ici test sur la création d'une nouvelle personne dans la DB
///
#[cfg(test)]
mod tests {
    use actix_web::dev::Service;
    use actix_web::{http, test, web, App, Error};

    use crate::models::{Person, InsertablePerson};
    use crate::handlers;
    use crate::handlers::{add_person_hdl, update_person_hdl};

    #[actix_rt::test]
    async fn test_add_person() -> Result<(), Error> {
        let mut app = test::init_service(
            App::new()
                .service(add_person_hdl),
        ).await;

        let req = test::TestRequest::post()
            .uri("/persons")
            .set_json(
                &Person {
                    id:0,
                    last_name: "DOE".to_owned(),
                    first_name: "John".to_owned(),
                }
            )
            .to_request();
        let resp = app.call(req).await.unwrap();

        assert_eq!(resp.status(), http::StatusCode::OK);

        let response_body = match resp.response().body().as_ref() {
            Some(actix_web::body::Body::Bytes(bytes)) => bytes,
            _ => panic!("Response error"),
        };
        println!("reponse : {:?}", response_body);

        //assert_eq!(response_body, r##"{"nom":"my-name","prenom":"my-prenom"}"##);

        Ok(())
    }

    #[actix_rt::test]
    async fn test_update_person() -> Result<(), Error> {

        let mut app = test::init_service(
            App::new()
                .service(update_person_hdl),
        ).await;

        let req = test::TestRequest::put()
            .uri("/person/3")
            .set_json(&InsertablePerson {
                first_name: "VOLNAY".to_owned(),
                last_name: "Jules".to_owned(),
            })
            .to_request();
        let resp = app.call(req).await.unwrap();

        assert_eq!(resp.status(), http::StatusCode::OK);

        let response_body = match resp.response().body().as_ref() {
            Some(actix_web::body::Body::Bytes(bytes)) => bytes,
            _ => panic!("Response error"),
        };
        println!("reponse : {:?}", response_body);

        //assert_eq!(response_body, r##"{"nom":"my-name","prenom":"my-prenom"}"##);

        Ok(())

    }
}




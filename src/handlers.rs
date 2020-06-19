// src/handlers.rs
use crate::models::{Person};
use crate::db;
use actix_web::{/*delete, get, post, put,*/ web, HttpResponse, Responder};
use actix_web_codegen::{get, post, delete, put};
use sqlx::PgPool;

// default / handler
pub async fn index() -> impl Responder {
    HttpResponse::Ok().body(r#"
        Welcome to Actix-web with SQLx Persons example.
        Available routes:
        GET /persons -> list of all persons
        POST /person -> create new person
        GET /person/{id} -> show one person with requested id
        PUT /person/{id} -> update person with requested id
        DELETE /person/{id} -> delete person with requested id
    "#
    )
}


#[get("/persons")]
pub async fn list_persons_hdl(db_pool: web::Data<PgPool>) -> impl Responder {
    let result = db::list_persons(db_pool.get_ref()).await;
    match result {
        Ok(persons) => HttpResponse::Ok().json(persons),
        _ => HttpResponse::BadRequest().body("Error trying to read all persons from database")
    }
}


#[post("/persons")]
pub async fn add_person_hdl(person: web::Json<Person>, db_pool: web::Data<PgPool>) -> impl Responder {
    //let in_pers = InsertablePerson::from_person(person.into_inner());
    let result = db::add_person(db_pool.get_ref(),person.into_inner() ).await;
    match result {
        Ok(todo) => HttpResponse::Ok().json(todo),
        _ => HttpResponse::BadRequest().body("Error trying to create new todo")
    }
}

#[get("/person/{id}")]
pub async fn find_person_by_id_hdl(id: web::Path<i32>, db_pool: web::Data<PgPool>) -> impl Responder {
    let result = db::find_person_by_id(id.into_inner(), db_pool.get_ref()).await;
    match result {
        Ok(person) => HttpResponse::Ok().json(person),
        _ => HttpResponse::BadRequest().body("Person not found")
    }
}

#[put("/person/{id}")]
pub(crate) async fn update_person_hdl(id: web::Path<i32>, person: web::Json<Person>, db_pool: web::Data<PgPool>) -> impl Responder {
    let result = db::update(id.into_inner(), person.into_inner(),db_pool.get_ref()).await;
    match result {
        Ok(person) => HttpResponse::Ok().json(person),
        _ => HttpResponse::BadRequest().body("Person not found")
    }
}

#[delete("/person/{id}")]
async fn delete_person_hdl(id: web::Path<i32>, db_pool: web::Data<PgPool>) -> impl Responder {
    let result = db::delete(id.into_inner(), db_pool.get_ref()).await;
    match result {
        Ok(rows) => {
            if rows > 0 {
                HttpResponse::Ok().body(format!("Successfully deleted {} record(s)", rows))
            } else {
                HttpResponse::BadRequest().body("Person not found")
            }
        },
        _ => HttpResponse::BadRequest().body("Person not found")
    }
}


// function that will be called on new Application to configure routes for this module
pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(list_persons_hdl);
    cfg.service(find_person_by_id_hdl);
    cfg.service(add_person_hdl);
    cfg.service(update_person_hdl);
    cfg.service(delete_person_hdl);
}
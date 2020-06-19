// src/models.rs
use serde::{Serialize, Deserialize};
use sqlx::FromRow;
use actix_web::{Responder, HttpRequest, HttpResponse, Error};
use futures::future::{Ready, ready};

// this struct will be used to represent database record
#[derive(Serialize, Deserialize, FromRow)]
pub struct Person {
    pub id: i32,
    pub first_name: String,
    pub last_name: String,
}

// implementation of Actix Responder for Todo struct so we can return Todo from action handler
impl Responder for Person {
    type Error = Error;
    type Future = Ready<Result<HttpResponse, Error>>;

    fn respond_to(self, _req: &HttpRequest) -> Self::Future {
        let body = serde_json::to_string(&self).unwrap();
        // create response and set content type
        ready(Ok(
            HttpResponse::Ok()
                .content_type("application/json")
                .body(body)
        ))
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, FromRow)]
pub struct InsertablePerson {
    pub first_name : String,
    pub last_name : String,
}

impl InsertablePerson {
    pub fn from_person(person: Person) -> InsertablePerson {
        InsertablePerson {
            first_name: person.first_name,
            last_name: person.last_name,
        }
    }

    pub fn to_string(&self) -> String {
        let mut str = String::new();
        str.push_str(&self.last_name);
        str.push_str(" ");
        str.push_str(&self.first_name);
        str
    }
}

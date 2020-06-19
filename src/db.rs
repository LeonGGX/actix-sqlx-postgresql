// src/db.rs

use sqlx::{PgPool, Row};
use sqlx::postgres::PgRow;

use crate::models::{Person};


/// Open a connection to a database
pub async fn connect(db_url: &str) -> sqlx::Result<PgPool> {
    let pool = PgPool::new(db_url).await?;
    Ok(pool)
}


pub async fn list_persons(pool: &PgPool) -> anyhow::Result<Vec<Person>> {

    let mut persons:Vec<Person> = Vec::new();
    let recs = sqlx::query!(
        r#"
SELECT id, first_name, last_name
FROM persons
ORDER BY id
        "#
    )
        .fetch_all(pool)
        .await?;

    for rec in recs {
        persons.push(
            Person {
                id: rec.id,
                first_name: rec.first_name,
                last_name: rec.last_name
            }
        );
    }

    Ok(persons)
}

pub async fn find_person_by_id(id: i32, pool: &PgPool) -> anyhow::Result<Person> {
    let rec = sqlx::query!(
                r#"
                    SELECT * FROM persons WHERE id = $1
                "#,
                id
            )
        .fetch_one(&*pool)
        .await?;

    Ok(Person {
        id: rec.id,
        first_name: rec.first_name,
        last_name: rec.last_name
    })
}

pub async fn add_person(pool: &PgPool, pers: Person) -> anyhow::Result<Person> {
    let mut tx = pool.begin().await?;
    let rec = sqlx::query(  "INSERT INTO persons (first_name, last_name)
                ²       ²²      VALUES ( $1, $2 )
                                RETURNING id, first_name, last_name;"
    )
        .bind(&pers.first_name)
        .bind(&pers.last_name)
        .map(|row:PgRow| {
            Person {
                id: row.get(0),
                first_name: row.get(1),
                last_name: row.get(2)
            }
        })
        .fetch_one(&mut tx)
        .await?;
    tx.commit().await?;

    Ok(rec)
}


pub async fn update(id: i32, person: Person, pool: &PgPool) -> anyhow::Result<Person> {
    let mut tx = pool.begin().await.unwrap();
    let person = sqlx::query("UPDATE persons \
                                        SET first_name = $1, \
                                        last_name = $2 \
                                        WHERE id = $3 \
                                        RETURNING id, first_name, last_name;")
        .bind(&person.first_name)
        .bind(&person.last_name)
        .bind(id)
        .map(|row: PgRow| {
            Person {
                id: row.get(0),
                first_name: row.get(1),
                last_name: row.get(2)
            }
        })
        .fetch_one(&mut tx)
        .await?;

    tx.commit().await?;
    Ok(person)
}

pub async fn delete(id: i32, pool: &PgPool) -> anyhow::Result<u64> {
    let mut tx = pool.begin().await?;
    let deleted = sqlx::query("DELETE FROM persons WHERE id = $1")
        .bind(id)
        .execute(&mut tx)
        .await?;

    tx.commit().await?;
    Ok(deleted)
}

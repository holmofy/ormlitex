use ormlitex::model::*;
use ormlitex::Connection;
use uuid::Uuid;

#[derive(Model)]
pub struct Person {
    id: Uuid,
    #[ormlitex(primary_key)]
    name: String,
    age: u8,
}


pub static CREATE_TABLE_SQL: &str =
    "CREATE TABLE person (id text PRIMARY KEY, name TEXT, age INTEGER)";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    let mut db = ormlitex::sqlite::SqliteConnection::connect(":memory:").await.unwrap();
    ormlitex::query(CREATE_TABLE_SQL)
        .execute(&mut db)
        .await?;

    let p = Person {
        id: Uuid::new_v4(),
        name: "John".to_string(),
        age: 99,
    }.insert(&mut db).await?;

    let p = p.update_partial()
        .age(100)
        .update(&mut db)
        .await?;

    assert_eq!(p.age, 100);

    Ok(())
}

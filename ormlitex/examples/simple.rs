use ormlitex::model::*;
use ormlitex::Connection;

#[derive(Model, Debug)]
#[ormlitex(insertable = InsertPerson)]
// #[index(col, col2, col3, unique = true, name = "my_index", type="btree")]
pub struct Person {
    pub id: u32,
    pub name: String,
    pub age: u8,
}

pub static CREATE_TABLE_SQL: &str =
    "CREATE TABLE person (id INTEGER PRIMARY KEY, name TEXT, age INTEGER)";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut conn = ormlitex::sqlite::SqliteConnection::connect(":memory:").await.unwrap();
    env_logger::init();

    ormlitex::query(CREATE_TABLE_SQL)
        .execute(&mut conn)
        .await?;

    // You can insert the model directly.
    let mut john = Person {
        id: 1,
        name: "John".to_string(),
        age: 99,
    }
    .insert(&mut conn)
    .await?;
    println!("{:?}", john);

    println!("select");
    let people = Person::select()
        .where_("age > ?")
        .bind(50u32)
        .fetch_all(&mut conn)
        .await?;
    println!("select query builder {:?}", people);

    let r = sqlx::query_as::<_, Person>("select * from person where age > ?")
        .bind(50u32)
        .fetch_all(&mut conn)
        .await?;
    println!("sqlx {:?}", r);

    // After modifying the object, you can update all fields directly.
    john.age = john.age + 1;
    john = john.update_all_fields(&mut conn).await?;
    println!("{:?}", john);

    // Lastly, you can delete the object.
    john.delete(&mut conn).await?;
    // You can get a single user.
    Person::fetch_one(1u32, &mut conn)
        .await
        .expect_err("Should not exist");

    Person {
        id: 1,
        name: "Dan".to_string(),
        age: 28,
    }
    .insert(&mut conn)
    .await?;

    let dan = Person::fetch_one(1u32, &mut conn).await?;
    println!("get_one {:?}", dan);

    let dan2 = dan.update_partial().age(29).update(&mut conn).await?;
    println!("dan1 {:?}", dan);
    println!("dan2 {:?}", dan2);

    InsertPerson {
        name: "Albert Einstein".to_string(),
        age: 60,
    }
    .insert(&mut conn)
    .await?;

    let kurt = Person::builder()
        .name("Kurt".to_string())
        .age(29)
        .insert(&mut conn)
        .await?;
    println!("built {:?}", kurt);
    // // You can create a query builder.
    let people = Person::select()
        .where_("age > ?")
        .bind(50u32)
        .fetch_all(&mut conn)
        .await?;
    println!("select builder {:?}", people);

    let people = Person::query("SELECT * FROM person WHERE age > ?")
        .bind(20u32)
        .fetch_all(&mut conn)
        .await?;
    println!("raw query: {:?}", people);
    Ok(())
}

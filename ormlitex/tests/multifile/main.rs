#[path = "../setup.rs"]
mod setup;

mod user;
mod organization;

pub use user::User;
pub use organization::Organization;
use uuid::Uuid;
use ormlitex::model::*;
use ormlitex::sqlite::SqliteConnection;
use ormlitex::Connection;
use sqlmo::ToSql;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut conn = SqliteConnection::connect(":memory:").await?;
    let migration = setup::migrate_self(&[
        &std::path::Path::new(file!()).parent().unwrap().display().to_string(),
    ]);
    for s in migration.statements {
        let sql = s.to_sql(sqlmo::Dialect::Sqlite);
        ormlitex::query(&sql)
            .execute(&mut conn)
            .await?;
    }

    let org_id = Uuid::new_v4();
    let org = Organization {
        id: org_id,
        name: "Acme".to_string(),
        is_active: true,
    };
    let user = User {
        id: Uuid::new_v4(),
        name: "John".to_string(),
        age: 99,
        organization: Join::new(org),
    };
    let user = user.insert(&mut conn).await?;
    assert_eq!(user.organization.id, org_id);
    Ok(())
}
use ormlitex::Model;
use ormlitex::model::{Insertable, Join, JoinMeta};
use sqlmo::ToSql;

use ormlitex::Connection;
#[path = "../setup.rs"]
mod setup;

#[derive(Debug, Model, Clone)]
pub struct Organization {
    id: i32,
    name: String,
}

#[derive(Model)]
#[ormlitex(insertable = InsertUser)]
pub struct User {
    id: i32,
    name: String,
    #[ormlitex(default)]
    secret: Option<String>,
    #[ormlitex(default_value = "5")]
    number: i32,
    #[ormlitex(column = "type")]
    typ: i32,
    #[ormlitex(join_column = "org_id")]
    organization: Join<Organization>,
}

#[tokio::main]
async fn main() {
    let mut db = ormlitex::sqlite::SqliteConnection::connect(":memory:")
        .await
        .unwrap();
    let migration = setup::migrate_self(&[file!()]);
    for s in migration.statements {
        let sql = s.to_sql(sqlmo::Dialect::Sqlite);
        ormlitex::query(&sql)
            .execute(&mut db)
            .await
            .unwrap();
    }

    let org = Organization {
        id: 12321,
        name: "my org".to_string(),
    };

    let champ = InsertUser {
        name: "Champ".to_string(),
        organization: Join::new(org.clone()),
        typ: 12,
    }.insert(&mut db)
        .await
        .unwrap();

    assert_eq!(champ.id, 1);
    assert_eq!(champ.secret, None);
    assert_eq!(champ.number, 5);
    assert_eq!(champ.organization.id, 12321);
    assert_eq!(champ.organization.name, "my org");

    let millie = InsertUser {
        name: "Millie".to_string(),
        organization: Join::new(org),
        typ: 3,
    }.insert(&mut db)
        .await
        .unwrap();
    assert_eq!(millie.id, 2);
    assert_eq!(millie.secret, None);
    assert_eq!(millie.number, 5);
    assert_eq!(millie.organization.id, 12321);
    assert_eq!(millie.organization.name, "my org");

    let enoki = InsertUser {
        name: "Enoki".to_string(),
        organization: Join::new_with_id(12321),
        typ: 6,
    }.insert(&mut db)
        .await
        .unwrap();
    assert_eq!(enoki.id, 3);
    assert_eq!(enoki.secret, None);
    assert_eq!(enoki.number, 5);
    assert_eq!(enoki.organization.id, 12321);
    assert_eq!(enoki.organization.loaded(), false);

}
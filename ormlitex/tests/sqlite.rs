#![cfg(not(any(feature = "postgres", feature = "mysql")))]
#[path = "./run.rs"]
mod run;

use run::*;

#[test]
fn test_sqlite() {
    set_path_and_run("tests/sqlite/01-table-meta.rs");
    set_path_and_run("tests/sqlite/02-update-partial.rs");
    set_path_and_run("tests/sqlite/03-many-to-one-join.rs");
    set_path_and_run("tests/sqlite/04-allow-clone-primary-key.rs");
    set_path_and_run("tests/sqlite/05-keyword-column.rs");
    set_path_and_run("tests/sqlite/06-insertable.rs");
    // t.pass("tests/03-many-to-many.rs");
    // t.pass("tests/04-one-to-many.rs");
}

#[test]
fn test_multifile() {
    set_dir_and_run("tests/multifile", "main.rs");
}

use sqlx::error::DatabaseError;

pub fn has_code(db_error: impl AsRef<dyn DatabaseError>, expected_code: impl AsRef<str>) -> bool {
    db_error.as_ref().code()
        .map(|code| code.as_ref() == expected_code.as_ref())
        .unwrap_or(false)
}

pub fn db_already_exists(db_error: impl AsRef<dyn DatabaseError>) -> bool {
    has_code(db_error, "42P04")
}

pub fn db_doesnt_exist(db_error: impl AsRef<dyn DatabaseError>) -> bool {
    has_code(db_error, "3D000")
}
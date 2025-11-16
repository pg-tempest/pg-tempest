use std::fmt::{Display, Formatter};

pub struct AdHocDisplay<T>(pub T)
where
    T: Fn(&mut Formatter) -> std::fmt::Result;

impl<T> Display for AdHocDisplay<T>
where
    T: Fn(&mut Formatter) -> std::fmt::Result,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.0(f)
    }
}

#[cfg(test)]
mod tests {
    use crate::utils::adhoc_display::AdHocDisplay;

    #[test]
    fn happy_path() {
        let db_name = "test_db";
        let is_template = false;
        let template_db_name = Some("template_db");

        let sql = format!(
            r#"
            CREATE DATABASE "{db_name}"
                TEMPLATE {}
                IS_TEMPLATE {is_template}
            "#,
            AdHocDisplay(|f| {
                match template_db_name {
                    None => f.write_str("DEFAULT"),
                    Some(db_name) => write!(f, r#""{db_name}""#),
                }
            })
        );

        assert_eq!(
            sql.as_str(),
            r#"
            CREATE DATABASE "test_db"
                TEMPLATE "template_db"
                IS_TEMPLATE false
            "#
        )
    }
}

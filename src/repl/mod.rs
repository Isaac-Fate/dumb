#[cfg(test)]
mod tests {
    use sqlparser::dialect::{ GenericDialect, SQLiteDialect };
    use sqlparser::parser::Parser;

    #[test]
    fn test_sql_parser() {
        let dialect = SQLiteDialect {}; // or AnsiDialect

        let sql = "SELECT name FROM users WHERE age > 30 ORDER BY name DESC";
        let ast = Parser::parse_sql(&dialect, sql).unwrap();

        println!("{:#?}", ast);
    }
}

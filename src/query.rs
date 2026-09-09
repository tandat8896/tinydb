use crate::error::DbError;

#[derive(Debug, PartialEq)]
pub enum Stmt {
    Insert {
        table: String,
        id: i64,
        fields: Vec<String>,
    },
    Select {
        table: String,
        where_id: Option<i64>,
    },
}

pub fn lex(input: &str) -> Vec<String> {
    let mut tokens = Vec::new();
    let mut current = String::new();
    let mut in_quotes = false;

    for ch in input.chars() {
        match ch {
            '"' if !in_quotes => {
                in_quotes = true;
            }
            '"' if in_quotes => {
                in_quotes = false;
            }
            ' ' if !in_quotes => {
                if !current.is_empty() {
                    tokens.push(std::mem::take(&mut current));
                }
            }
            '(' | ')' | ',' if !in_quotes => {
                if !current.is_empty() {
                    tokens.push(std::mem::take(&mut current));
                }
                tokens.push(ch.to_string());
            }
            _ => current.push(ch),
        }
    }
    if !current.is_empty() {
        tokens.push(current);
    }
    tokens
}

pub fn parse(tokens: &[String]) -> Result<Stmt, DbError> {
    if tokens.is_empty() {
        return Err(DbError::InvalidQuery("Empty query".into()));
    }

    match tokens[0].to_uppercase().as_str() {
        "INSERT" => parse_insert(tokens),
        "SELECT" => parse_select(tokens),
        _ => Err(DbError::InvalidQuery(format!(
            "Unknown command: {}",
            tokens[0]
        ))),
    }
}

fn parse_insert(tokens: &[String]) -> Result<Stmt, DbError> {
    // INSERT INTO t (id, name) VALUES (1, "alice")
    if tokens.len() < 9 {
        return Err(DbError::InvalidQuery("Invalid INSERT syntax".into()));
    }

    if tokens[1].to_uppercase() != "INTO" {
        return Err(DbError::InvalidQuery("Expected INTO after INSERT".into()));
    }

    let table = tokens[2].clone();

    // Find opening paren for columns
    let open_paren = tokens.iter().position(|t| t == "(").ok_or_else(|| {
        DbError::InvalidQuery("Expected ( after table name".into())
    })?;

    // Find VALUES keyword
    let values_pos = tokens
        .iter()
        .position(|t| t.to_uppercase() == "VALUES")
        .ok_or_else(|| DbError::InvalidQuery("Expected VALUES keyword".into()))?;

    // Parse column names (between first parens, excluding 'id')
    let mut fields = Vec::new();
    for i in (open_paren + 1)..values_pos {
        if tokens[i] == ")" {
            break;
        }
        if tokens[i] == "," {
            continue;
        }
        let col = tokens[i].to_lowercase();
        if col != "id" {
            fields.push(col);
        }
    }

    // Find opening paren for values
    let values_paren = tokens
        .iter()
        .skip(values_pos)
        .position(|t| t == "(")
        .ok_or_else(|| DbError::InvalidQuery("Expected ( after VALUES".into()))?
        + values_pos;

    // Parse values
    let mut values = Vec::new();
    for i in (values_paren + 1)..tokens.len() {
        if tokens[i] == ")" {
            break;
        }
        if tokens[i] == "," {
            continue;
        }
        values.push(tokens[i].clone());
    }

    // First value should be the id
    if values.is_empty() {
        return Err(DbError::InvalidQuery("No values provided".into()));
    }

    let id: i64 = values[0]
        .parse()
        .map_err(|_| DbError::InvalidQuery(format!("Invalid id: {}", values[0])))?;

    // Remaining values are the fields
    let field_values: Vec<String> = values[1..].to_vec();

    Ok(Stmt::Insert {
        table,
        id,
        fields: field_values,
    })
}

fn parse_select(tokens: &[String]) -> Result<Stmt, DbError> {
    // SELECT * FROM t WHERE id = 1
    // Minimum: SELECT * FROM t (4 tokens)
    if tokens.len() < 4 {
        return Err(DbError::InvalidQuery("Invalid SELECT syntax".into()));
    }

    let from_pos = tokens
        .iter()
        .position(|t| t.to_uppercase() == "FROM")
        .ok_or_else(|| DbError::InvalidQuery("Expected FROM keyword".into()))?;

    let table = tokens[from_pos + 1].clone();

    let where_id = if let Some(where_pos) = tokens
        .iter()
        .position(|t| t.to_uppercase() == "WHERE")
    {
        // WHERE id = value
        let eq_pos = tokens
            .iter()
            .skip(where_pos)
            .position(|t| t == "=")
            .ok_or_else(|| DbError::InvalidQuery("Expected = after WHERE".into()))?
            + where_pos;

        let id_str = &tokens[eq_pos + 1];
        Some(
            id_str
                .parse()
                .map_err(|_| DbError::InvalidQuery(format!("Invalid id: {}", id_str)))?,
        )
    } else {
        None
    };

    Ok(Stmt::Select { table, where_id })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lex_insert() {
        let tokens = lex(r#"INSERT INTO users (id, name) VALUES (1, "alice")"#);
        assert_eq!(
            tokens,
            vec![
                "INSERT", "INTO", "users", "(", "id", ",", "name", ")", "VALUES", "(", "1",
                ",", "alice", ")"
            ]
        );
    }

    #[test]
    fn test_lex_select() {
        let tokens = lex("SELECT * FROM users WHERE id = 1");
        assert_eq!(
            tokens,
            vec!["SELECT", "*", "FROM", "users", "WHERE", "id", "=", "1"]
        );
    }

    #[test]
    fn test_parse_insert() {
        let tokens = lex(r#"INSERT INTO users (id, name) VALUES (1, "alice")"#);
        let stmt = parse(&tokens).unwrap();
        assert_eq!(
            stmt,
            Stmt::Insert {
                table: "users".into(),
                id: 1,
                fields: vec!["alice".into()],
            }
        );
    }

    #[test]
    fn test_parse_insert_multiple_fields() {
        let tokens = lex(r#"INSERT INTO users (id, name, dept) VALUES (1, "alice", "eng")"#);
        let stmt = parse(&tokens).unwrap();
        assert_eq!(
            stmt,
            Stmt::Insert {
                table: "users".into(),
                id: 1,
                fields: vec!["alice".into(), "eng".into()],
            }
        );
    }

    #[test]
    fn test_parse_select() {
        let tokens = lex("SELECT * FROM users WHERE id = 1");
        let stmt = parse(&tokens).unwrap();
        assert_eq!(
            stmt,
            Stmt::Select {
                table: "users".into(),
                where_id: Some(1),
            }
        );
    }

    #[test]
    fn test_parse_select_no_where() {
        let tokens = lex("SELECT * FROM users");
        let stmt = parse(&tokens).unwrap();
        assert_eq!(
            stmt,
            Stmt::Select {
                table: "users".into(),
                where_id: None,
            }
        );
    }

    #[test]
    fn test_parse_error_unknown_command() {
        let tokens = lex("DELETE FROM users");
        assert!(parse(&tokens).is_err());
    }

    #[test]
    fn test_parse_error_empty() {
        let tokens: Vec<String> = vec![];
        assert!(parse(&tokens).is_err());
    }
}

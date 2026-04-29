use ext_php_rs::prelude::*;
use mysql::*;
use mysql::prelude::*;
use std::collections::HashMap;

#[php_class]
pub struct DB {
    pool: Pool,
}

#[php_impl]
impl DB {

    pub fn __construct(host: String, db: String, user: String, pass: String) -> PhpResult<Self> {

    let url = format!("mysql://{}:{}@{}/{}", user, pass, host, db);

    let opts = Opts::from_url(&url)
        .map_err(|e| PhpException::from(e.to_string()))?;

    let pool = Pool::new(opts)
        .map_err(|e| PhpException::from(e.to_string()))?;

    Ok(DB { pool })
}

pub fn query(&self, sql: String) -> PhpResult<Vec<String>> {

        let mut conn = self.pool
            .get_conn()
            .map_err(|e| PhpException::from(e.to_string()))?;

        let result: Vec<Row> = conn
            .query(sql)
            .map_err(|e| PhpException::from(e.to_string()))?;

        Ok(result
            .into_iter()
            .map(|row| format!("{:?}", row))
            .collect())
    }
    
    pub fn create(&self, table: String, columns: Vec<String>) -> PhpResult<String> {

        let cols = columns.join(", ");

        let sql = format!("CREATE TABLE {} ({})", table, cols);

        let mut conn = self.pool.get_conn()
            .map_err(|e| PhpException::from(e.to_string()))?;

        conn.query_drop(sql)
            .map_err(|e| PhpException::from(e.to_string()))?;

        Ok("created".to_string())
    }
    
    pub fn insert(&self, table: String, data: HashMap<String, String>) -> PhpResult<String> {

    let mut conn = self.pool.get_conn()
        .map_err(|e| PhpException::from(e.to_string()))?;

    let columns: Vec<String> = data.keys().cloned().collect();
    let placeholders: Vec<String> = columns.iter().map(|k| format!(":{}", k)).collect();

    let sql = format!(
        "INSERT INTO {} ({}) VALUES ({})",
        table,
        columns.join(", "),
        placeholders.join(", ")
    );

    let params = Params::Named(
        data.into_iter()
            .map(|(k, v)| (k.into(), v.into()))
            .collect()
    );

    conn.exec_drop(sql, params)
        .map_err(|e| PhpException::from(e.to_string()))?;

    Ok("inserted".to_string())
}

pub fn select(
    &self,
    table: String,
    columns: Vec<String>,
    where_data: HashMap<String, String>
) -> PhpResult<Vec<String>> {

    let mut conn = self.pool.get_conn()
        .map_err(|e| PhpException::from(e.to_string()))?;

    let cols = if columns.is_empty() {
        "*".to_string()
    } else {
        columns.join(", ")
    };

    let conditions: Vec<String> = where_data
        .keys()
        .map(|k| format!("{} = :{}", k, k))
        .collect();

    let sql = format!(
        "SELECT {} FROM {} WHERE {}",
        cols,
        table,
        conditions.join(" AND ")
    );

    let params = Params::Named(
        where_data.into_iter()
            .map(|(k, v)| (k.into(), v.into()))
            .collect()
    );

    let result: Vec<Row> = conn.exec(sql, params)
        .map_err(|e| PhpException::from(e.to_string()))?;

    Ok(result.into_iter().map(|r| format!("{:?}", r)).collect())
}

pub fn update(
    &self,
    table: String,
    data: HashMap<String, String>,
    where_data: HashMap<String, String>
) -> PhpResult<String> {

    let mut conn = self.pool.get_conn()
        .map_err(|e| PhpException::from(e.to_string()))?;

    let set_parts: Vec<String> = data
        .keys()
        .map(|k| format!("{} = :set_{}", k, k))
        .collect();

    let where_parts: Vec<String> = where_data
        .keys()
        .map(|k| format!("{} = :where_{}", k, k))
        .collect();

    let mut params_map = std::collections::HashMap::new();

    for (k, v) in data {
        params_map.insert(format!("set_{}", k), v);
    }

    for (k, v) in where_data {
        params_map.insert(format!("where_{}", k), v);
    }

    let sql = format!(
        "UPDATE {} SET {} WHERE {}",
        table,
        set_parts.join(", "),
        where_parts.join(" AND ")
    );

    let params = Params::Named(
        params_map.into_iter()
            .map(|(k, v)| (k.into(), v.into()))
            .collect()
    );

    conn.exec_drop(sql, params)
        .map_err(|e| PhpException::from(e.to_string()))?;

    Ok("updated".to_string())
}

pub fn delete(
    &self,
    table: String,
    where_data: HashMap<String, String>
) -> PhpResult<String> {

    let mut conn = self.pool.get_conn()
        .map_err(|e| PhpException::from(e.to_string()))?;

    let conditions: Vec<String> = where_data
        .keys()
        .map(|k| format!("{} = :{}", k, k))
        .collect();

    let sql = format!(
        "DELETE FROM {} WHERE {}",
        table,
        conditions.join(" AND ")
    );

    let params = Params::Named(
        where_data.into_iter()
            .map(|(k, v)| (k.into(), v.into()))
            .collect()
    );

    conn.exec_drop(sql, params)
        .map_err(|e| PhpException::from(e.to_string()))?;

    Ok("deleted".to_string())
}

}

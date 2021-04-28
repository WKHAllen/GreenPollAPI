use tokio_postgres::{Client, NoTls, Error, Row};
use tokio_postgres::types::ToSql;
use std::marker::Sync;

pub struct DB {
  client: Client,
}

impl DB {
  pub async fn new(db_url: &str) -> Result<DB, Error> {
    let (client, connection) = tokio_postgres::connect(db_url, NoTls).await?;

    tokio::spawn(async move {
      if let Err(e) = connection.await {
        eprintln!("Connection error: {}", e);
      }
    });

    Ok(DB {
      client,
    })
  }

  pub async fn execute(&self, sql: &str, params: &[&(dyn ToSql + Sync)]) -> Result<Vec<Row>, Error> {
    self.client.query(sql, params).await
  }

  pub async fn execute_many(&self, sql: &[&str], params: &[&[&(dyn ToSql + Sync)]]) -> Result<Vec<Vec<Row>>, Error> {
    let mut results: Vec<Vec<Row>> = Vec::new();

    for i in 0..sql.len() {
      let res = self.execute(sql[i], params[i]).await?;
      results.push(res);
    }

    Ok(results)
  }
}

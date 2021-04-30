// use sqlx::postgres::PgPoolOptions;
// use std::fs::read_to_string;

// pub struct DB {
//     pool: sqlx::Pool<sqlx::Postgres>,
// }

// impl DB {
//     pub async fn new(db_url: &str, max_conns: u32) -> Result<Self, sqlx::Error> {
//         // let mut builder = SslConnector::builder(SslMethod::tls())?;
//         // builder.set_verify(SslVerifyMode::NONE);
//         // let connector = MakeTlsConnector::new(builder.build());

//         // let (client, conn) = connect_tls(db_url.parse()?, connector).await?;
//         // spawn(conn);

//         let pool = PgPoolOptions::new()
//             .max_connections(max_conns)
//             .connect(db_url).await?;

//         Ok(DB {
//             pool,
//         })
//     }

//     pub async fn query<T, U>(
//         &self,
//         sql: &str,
//         params: &[U]
//     ) -> std::io::Result<Vec<T>> {
//         sqlx::query_as!(T, sql, ..params)
//             .fetch_all(&self.pool)
//             .await?
//     }

//     // pub async fn execute_many<T>(
//     //     &self,
//     //     sql: &[&T],
//     //     params: &[&[&(dyn ToSql + Sync)]]
//     // ) -> Result<Vec<Vec<Row>>, Error>
//     // where T: ?Sized + ToStatement {
//     //     let mut results: Vec<Vec<Row>> = Vec::new();

//     //     for i in 0..sql.len() {
//     //         let res = self.execute(sql[i], params[i]).await?;
//     //         results.push(res);
//     //     }

//     //     Ok(results)
//     // }

//     pub async fn query_file<T, U>(
//         &self,
//         path: &str,
//         params: &[U]
//     ) -> std::io::Result<Vec<T>> {
//         let sql = read_to_string(path)?;
//         self.query::<T, U>(&sql[..], params).await
//     }

//     pub async fn query_sql_file<T, U>(
//         &self,
//         path: &str,
//         params: &[U]
//     ) -> std::io::Result<Vec<T>> {
//         let file_path = format!("../sql/{}.sql", path);
//         self.query_file::<T, U>(&file_path[..], params).await
//     }

//     pub async fn execute<U>(
//         &self,
//         sql: &str,
//         params: &[U]
//     ) -> std::io::Result<()> {
//         sqlx::query!(sql, ..params)
//             .fetch_all(&self.pool)
//             .await?
//     }

//     pub async fn execute_file<U>(
//         &self,
//         path: &str,
//         params: &[U]
//     ) -> std::io::Result<()> {
//         let sql = read_to_string(path)?;
//         self.execute::<U>(&sql[..], params).await
//     }

//     pub async fn execute_sql_file<U>(
//         &self,
//         path: &str,
//         params: &[U]
//     ) -> std::io::Result<()> {
//         let file_path = format!("../sql/{}.sql", path);
//         self.execute_file::<U>(&file_path[..], params).await
//     }
// }

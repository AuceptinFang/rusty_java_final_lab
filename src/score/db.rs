use anyhow::Result;
use serde::Deserialize;
use sqlx::{Connection, FromRow, SqliteConnection, SqlitePool};
use std::fs::File;
use std::path::Path;
use tracing::{error, info};
#[derive(Debug, Deserialize)]
pub struct ScoreRecord {
    #[serde(rename = "学号")]
    pub id: String,
    #[serde(rename = "姓名")]
    pub name: String,
    #[serde(rename = "平时成绩")]
    pub score_avg: f64,
    #[serde(rename = "期末成绩")]
    pub score_final: f64,
}
#[derive(Debug, FromRow)]
struct DbRow {
    id: String,
    name: String,
    score_usual: f64,
    score_final: f64,
    total: f64,
}

pub(crate) struct Db {
    pool: SqlitePool,
}
impl ScoreRecord {
    pub fn new(id: String, name: String, score_avg: f64, score_final: f64) -> ScoreRecord {
        ScoreRecord {
            id,
            name,
            score_avg,
            score_final,
        }
    }
}

impl Db {
    pub(crate) async fn new() -> Result<Db> {
        let database_url =
            std::env::var("DATABASE_URL").unwrap_or_else(|_| "sqlite://./exam.db".to_string());

        let pool = SqlitePool::connect(&database_url).await?;

        sqlx::query(
            "CREATE TABLE IF NOT EXISTS scores(
                        id TEXT PRIMARY KEY,
                        name text,
                        usual REAL,
                        final REAL,
                        total REAL
            )",
        )
        .execute(&pool)
        .await?;

        Ok(Db { pool })
    }

    pub async fn insert(&mut self, r: ScoreRecord) -> Result<()> {
        let total = 0.4 * r.score_avg + 0.6 * r.score_final;
        sqlx::query(
            "INSERT OR REPLACE INTO scores(
                   id,name,usual,final,total)
                   VALUES (?, ?, ?, ?, ?)
                  ",
        )
        .bind(&r.id)
        .bind(&r.name)
        .bind(&r.score_avg)
        .bind(&r.score_final)
        .bind(&total)
        .execute(&self.pool)
        .await?;
        info!(
            "添加成功 id = {} name = {} 平时成绩{} 期末成绩{} 总分{}",
            r.id, r.name, r.score_avg, r.score_final, total
        );
        Ok(())
    }

    pub async fn search(&mut self, id: &str) -> Result<()> {
        let row = sqlx::query_as::<_, DbRow>(
            "SELECT id, name, usual as score_usual, final as score_final, total 
             FROM scores WHERE id = ?",
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await?;
        
        match row {
            Some(record) => {
                println!(
                    "id: {} name: {} 平时: {:.0} 期末: {:.0} 总分: {:.0}",
                    record.id, record.name, record.score_usual, record.score_final, record.total
                );
            }
            None => {
                println!("未找到学号为 {} 的学生", id);
            }
        }
        Ok(())
    }

    pub async fn show(&mut self, n: i32) -> Result<()> {
        let records = sqlx::query_as::<_, DbRow>(
            "SELECT id, name, usual as score_usual, final as score_final, total
             FROM scores
             ORDER BY total DESC
             LIMIT ?",
        )
        .bind(n)
        .fetch_all(&self.pool)
        .await?;

        for record in records {
            println!(
                "id: {} name: {} 平时: {:.0} 期末: {:.0} 总分: {:.0}",
                record.id, record.name, record.score_usual, record.score_final, record.total
            );
        }

        Ok(())
    }

    pub async fn clear_all(&mut self) -> Result<()> {
        sqlx::query("DELETE FROM scores")
            .execute(&self.pool)
            .await?;
        info!("已清空所有数据");
        Ok(())
    }

    pub async fn import_to_db(&mut self) -> Result<()> {
        let file_path = "./src/score/student.csv";
        let file = File::open(file_path)?;
        let mut rdr = csv::Reader::from_reader(file);
        for result in rdr.deserialize() {
            match result {
                Ok(record) => {
                    // 2. 成功：执行数据库插入
                    if let Err(db_err) = self.insert(record).await {
                        error!("DB Error on insert: {}", db_err);
                    }
                }
                Err(csv_err) => {
                    error!("CSV 格式错误 {}", csv_err);
                }
            }
        }

        Ok(())
    }
}

use mysql_async::{prelude::*, Pool};

pub struct Database {
    pool: Pool,
}

impl Database {
    pub fn new(url: &str) -> Self {
        let pool = Pool::new(url);
        Database { pool }
    }

    pub async fn init(&self) -> Result<(), mysql_async::Error> {
        // Tes koneksi
        let mut conn = self.pool.get_conn().await?;
        conn.ping().await?;
        println!("-----------------------------------------------");
        println!("Koneksi berhasil!");
        println!("-----------------------------------------------");
        Ok(())
    }

    pub async fn shutdown(self) -> Result<(), mysql_async::Error> {
        self.pool.disconnect().await
    }
}

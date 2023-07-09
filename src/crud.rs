pub struct Db(pub sqlx::PgPool);
use crate::schema::{User, History};

pub trait Koneksi {
    async fn new(url: String) -> Db;
    async fn login(&self, user: User) -> Vec<User>;
    async fn reg(&self, user: User) -> ();
    async fn change_pas(&self, user: User) -> ();
    async fn insert_hist(&self, user: User, bc_id:  String) -> ();
    async fn select_hist(&self, user: User) -> Vec<History>;
    async fn select_pass(&self) -> Vec<String>;
    async fn close(&self) -> ();
}

impl Koneksi for Db {
    async fn new(url: String) -> Self {
        Self(
            sqlx::PgPool::connect(url.as_str()).await.unwrap()
        )
    }

    async fn login(&self, user: User) -> Vec<User> {
        sqlx::query_as!(User, "SELECT id, email, passwd FROM bcdl_user WHERE email = $1", user.email).fetch_all(&self.0).await.unwrap()
    }

    async fn reg(&self, user: User) -> () {
        sqlx::query_as!(User, "INSERT INTO bcdl_user (email, passwd) VALUES ($1, $2)", user.email, user.passwd).fetch_all(&self.0).await.unwrap();
    }

    async fn change_pas(&self, user: User) -> () {
        sqlx::query_as!(User, "UPDATE bcdl_user SET passwd = $1 WHERE id = $2", user.passwd, user.id).fetch_all(&self.0).await.unwrap();
    }

    async fn insert_hist(&self, user: User, bc_id: String) -> () {
        sqlx::query_as!(User, "INSERT INTO bcdl_history (user_id, bc_link) VALUES ($1, $2)", user.id, bc_id).fetch_all(&self.0).await.unwrap();
    }

    async fn select_hist(&self, user: User) -> Vec<History> {
        sqlx::query_as!(History, "SELECT * FROM bcdl_history WHERE user_id = $1", user.id).fetch_all(&self.0).await.unwrap()
    }

    async fn select_pass(&self) -> Vec<String> {
        let a = sqlx::query!("SELECT passwd FROM bcdl_user").fetch_all(&self.0).await.unwrap();
        let mut new_a = Vec::<String>::new();
        for i in a {
            new_a.push(i.passwd)
        }
        new_a
    }

    async fn close(&self) -> () {
        self.0.close().await;
    }
}
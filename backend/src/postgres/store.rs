use serde::{Deserialize, Serialize};
use sqlx::{query_as, PgPool};

#[derive(Clone)]
pub(crate) struct Store {
    pool: PgPool,
}

#[derive(Clone, Serialize, Deserialize)]
pub(crate) struct Todo {
    pub id: i32,
    pub title: String,
    pub completed: bool,
}

pub struct ValidUsername(String);

impl From<String> for ValidUsername {
    fn from(username: String) -> Self {
        let clean_username = username
            .chars()
            .filter(|c| !c.is_whitespace())
            .map(|c| c.to_ascii_lowercase())
            .collect();

        Self(clean_username)
    }
}

impl Store {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn add_todo_and_return_all(
        &self,
        user_id: i32,
        title: String,
    ) -> Result<Vec<Todo>, sqlx::Error> {
        query_as!(
            Todo,
            r#"
            with inserted as (
                insert into todos (title, user_id)
                values ($1, $2)
                returning id, title, completed, user_id
            )
            select id "id!", title "title!", completed "completed!"
            from todos where user_id = $2
            union all
            select id "id!", title "title!", completed "completed!" from inserted
            where user_id = $2
            order by "id!"
            "#,
            title,
            user_id
        )
        .fetch_all(&self.pool)
        .await
    }

    pub async fn get_todos(&self, user_id: i32) -> Result<Vec<Todo>, sqlx::Error> {
        query_as!(
            Todo,
            r#"
            select id, title, completed
            from todos
            where user_id = $1
            order by id
            "#,
            user_id
        )
        .fetch_all(&self.pool)
        .await
    }

    pub async fn delete_todo_and_return_all(
        &self,
        user_id: i32,
        id: i32,
    ) -> Result<Vec<Todo>, sqlx::Error> {
        query_as!(
            Todo,
            r#"
            with delete as (
                delete from todos
                where id = $1 and user_id = $2
            )
            select id, title, completed
            from todos where id != $1 and user_id = $2
            order by id
            "#,
            id,
            user_id
        )
        .fetch_all(&self.pool)
        .await
    }

    pub async fn update_todo_and_return_all(
        &self,
        user_id: i32,
        id: i32,
        title: String,
        completed: bool,
    ) -> Result<Vec<Todo>, sqlx::Error> {
        query_as!(
            Todo,
            r#"
            with update_todo as (
                update todos
                set title = $1, completed = $2
                where id = $3 and user_id = $4
                returning id, title, completed, user_id
            )
            select id "id!", title "title!", completed "completed!"
            from todos where id != $3 and user_id = $4
            union all
            select id "id!", title "title!", completed "completed!" from update_todo
            order by "id!"
            "#,
            title,
            completed,
            id,
            user_id
        )
        .fetch_all(&self.pool)
        .await
    }

    pub async fn create_user(
        &self,
        username: ValidUsername,
        password: String,
    ) -> Result<i32, sqlx::Error> {
        let password_hash = password_auth::generate_hash(password);

        let u = sqlx::query!(
            r#"
            insert into users (username, password_hash)
            values ($1, $2)
            returning id
            "#,
            username.0,
            password_hash
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(u.id)
    }

    pub async fn get_authenticated_user_id(
        &self,
        username: ValidUsername,
        password: String,
    ) -> Result<Option<i32>, sqlx::Error> {
        let user = sqlx::query!(
            r#"
            select id, password_hash
            from users
            where username = $1
            "#,
            username.0
        )
        .fetch_optional(&self.pool)
        .await?;

        if let Some(user) = user {
            if password_auth::verify_password(password, &user.password_hash).is_ok() {
                return Ok(Some(user.id));
            }
        }
        Ok(None)
    }
}

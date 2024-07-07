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

impl Store {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn add_todo_and_return_all(&self, title: String) -> Result<Vec<Todo>, sqlx::Error> {
        query_as!(
            Todo,
            r#"--sql
            with inserted as (
                insert into todos (title)
                values ($1)
                returning id, title, completed
            )
            select id "id!", title "title!", completed "completed!"
            from todos
            union all
            select id "id!", title "title!", completed "completed!" from inserted
            order by "id!"
            "#,
            title
        )
        .fetch_all(&self.pool)
        .await
    }

    pub async fn get_todos(&self) -> Result<Vec<Todo>, sqlx::Error> {
        query_as!(
            Todo,
            r#"--sql
            select id, title, completed
            from todos
            order by id
            "#,
        )
        .fetch_all(&self.pool)
        .await
    }

    pub async fn delete_todo_and_return_all(&self, id: i32) -> Result<Vec<Todo>, sqlx::Error> {
        query_as!(
            Todo,
            r#"--sql
            with delete as (
                delete from todos
                where id = $1
            )
            select id, title, completed
            from todos where id != $1
            order by id
            "#,
            id
        )
        .fetch_all(&self.pool)
        .await
    }

    pub async fn update_todo_and_return_all(
        &self,
        id: i32,
        title: String,
        completed: bool,
    ) -> Result<Vec<Todo>, sqlx::Error> {
        query_as!(
            Todo,
            r#"--sql
            with update_todo as (
                update todos
                set title = $1, completed = $2
                where id = $3
                returning id, title, completed
            )
            select id "id!", title "title!", completed "completed!" 
            from todos where id != $3
            union all
            select id "id!", title "title!", completed "completed!" from update_todo
            order by "id!"
            "#,
            title,
            completed,
            id
        )
        .fetch_all(&self.pool)
        .await
    }
}

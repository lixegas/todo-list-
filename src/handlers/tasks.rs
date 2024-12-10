use crate::models::status::Status;
use crate::models::tasks::{CreateTask,Task, UpdateTask};
use crate::db;
use axum::{http::StatusCode, Json};
use rusqlite::params;
use tokio::task;
use axum::extract::Path;
use rusqlite::OptionalExtension;
use chrono::{NaiveDateTime, Utc};

pub async fn list_tasks() -> Result<Json<Vec<Task>>, (StatusCode, String)> {
    let result: Result<Vec<Task>, String> = task::block_in_place(|| {
        let conn = db::connection_db().unwrap(); 

        let query = "SELECT id, description, status, created_at, updated_at FROM todoList;"; 

        let mut stmt = conn.prepare(query).unwrap(); 

       
        let task_iter = stmt.query_map(params![], |row| {
            Ok(Task {
                id: row.get(0)?,  
                description: row.get(1)?,  
                status: match row.get::<_, String>(2)?.as_str() {
                    "UNCOMPLETED" => Status::UNCOMPLETED,
                    "COMPLETED" => Status::COMPLETED,
                    "UPDATED" => Status::UPDATED,
                    "CANCELED" => Status::CANCELED,
                    "POSTPONE" => Status::POSTPONE,
                    _ => Status::UNCOMPLETED, 
                },
                created_at: row.get::<_, String>(3)?
                    .parse::<NaiveDateTime>().unwrap_or_else(|_| {
                        Utc::now().naive_utc()  
                    }),
                updated_at: row.get::<_, Option<String>>(4)?
                    .map(|s| NaiveDateTime::parse_from_str(&s, "%+").unwrap_or_else(|_| {
                        Utc::now().naive_utc()  
                    })),
            })
        }).unwrap();  

       
        let tasks: Vec<Task> = task_iter.filter_map(|task| task.ok()).collect();
        Ok(tasks) 
    });

    match result {
        Ok(tasks) => Ok(Json(tasks)), 
        Err(_) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            "Failed to retrieve tasks.".to_string(), 
        )),
    }
}

pub async fn create_task(Json(payload): Json<CreateTask>) -> Result<Json<Task>, (StatusCode, String)> {
    let status = Status::UNCOMPLETED;

    let result: Result<Task, String> = task::block_in_place(|| {
        let conn = db::connection_db().unwrap();  

        
        let query_check_exists = "SELECT COUNT(*) FROM todoList WHERE description = ?1;";
        let count: i64 = conn.query_row(query_check_exists, params![&payload.description], |row| row.get(0)).unwrap();

        if count > 0 {
            return Err("Task with the same description already exists.".to_string());
        }

       
        let query = "
            INSERT INTO todoList (description, status, created_at, updated_at)
            VALUES (?1, ?2, ?3, ?4);
        ";

        let created_at = Utc::now().to_string();
        let updated_at = "NULL".to_string(); 

        let status_str = match status {
            Status::UNCOMPLETED => "UNCOMPLETED",
            Status::COMPLETED => "COMPLETED",
            Status::UPDATED => "UPDATED",
            Status::CANCELED => "CANCELED",
            Status::POSTPONE => "POSTPONE",
        };

        conn.execute(
            query,
            params![&payload.description, &status_str, &created_at, &updated_at],
        ).unwrap();  

        let last_inserted_id = conn.last_insert_rowid();  

        
        Ok(Task {
            id: last_inserted_id as u64, 
            description: payload.description,
            status: status, 
            created_at: NaiveDateTime::parse_from_str(&created_at, "%+").unwrap(),
            updated_at: None,  
        })
    });

    match result {
        Ok(task) => Ok(Json(task)),  
        Err(e) => Err((
            StatusCode::BAD_REQUEST,  
            format!("Failed to create task: {}", e),
        )),
    }
}

#[warn(unused_variables)]
pub async fn update_task(Path(id): Path<u64>, Json(payload): Json<UpdateTask>) -> Result<Json<Task>, (StatusCode, String)> {
    let result: Result<Task, String> = task::block_in_place(|| {
        let conn = db::connection_db().unwrap();  

       
        let query_check_exists = "SELECT id, description, status, created_at, updated_at FROM todoList WHERE id = ?1;";
        let mut stmt = conn.prepare(query_check_exists).unwrap();
        
        let task_row = stmt.query_row(params![id], |row| {
            Ok((
                row.get::<_, u64>(0)?,
                row.get::<_, String>(1)?,
                row.get::<_, String>(2)?,
                row.get::<_, String>(3)?,
                row.get::<_, Option<String>>(4)?,  
            ))
        });

        let (existing_id, existing_description, _existing_status, created_at, _existing_updated_at) = match task_row {
            Ok(row) => row,
            Err(_) => {
                return Err("Task not found".to_string());
            }
        };

        
        let new_description = payload.description.unwrap_or(existing_description);
        let new_status = payload.status.unwrap_or_else(|| Status::UNCOMPLETED);  
        let updated_at = Utc::now().to_string();

        
        let status_str = match new_status {
            Status::UNCOMPLETED => "UNCOMPLETED",
            Status::COMPLETED => "COMPLETED",
            Status::UPDATED => "UPDATED",
            Status::CANCELED => "CANCELED",
            Status::POSTPONE => "POSTPONE",
        };

        
        let query_update = "
            UPDATE todoList 
            SET description = ?1, status = ?2, updated_at = ?3
            WHERE id = ?4;
        ";

        conn.execute(
            query_update,
            params![new_description, status_str, updated_at, id],
        ).unwrap(); 

        
        Ok(Task {
            id: existing_id,
            description: new_description,
            status: new_status,
            created_at: NaiveDateTime::parse_from_str(&created_at, "%+").unwrap(),
            updated_at: Some(NaiveDateTime::parse_from_str(&updated_at, "%+").unwrap()),  // Imposta l'updated_at
        })
    });

    match result {
        Ok(task) => Ok(Json(task)),  
        Err(e) => Err((
            StatusCode::NOT_FOUND,
            format!("Failed to update task: {}", e),
        )),
    }
}



pub async fn delete_task(Path(id): Path<u64>) -> Result<Json<String>, (StatusCode, String)> {
    let result = task::block_in_place(|| {
        let conn = db::connection_db().unwrap(); 

        
        let query_check = "SELECT id FROM todoList WHERE id = ?1";
        let mut stmt_check = conn.prepare(query_check).unwrap();
        let task_exists = stmt_check.query_row(params![id], |row| row.get::<_, u64>(0)).optional();

        match task_exists {
            Ok(Some(_)) => {
                
                let query_delete = "DELETE FROM todoList WHERE id = ?1";
                conn.execute(query_delete, params![id]).unwrap();
                Ok("Task deleted successfully.".to_string())
            }
            Ok(None) => Err("Task not found.".to_string()),  
            Err(_) => Err("Failed to check task.".to_string()), 
        }
    });

    match result {
        Ok(message) => Ok(Json(message)),
        Err(e) => Err((
            StatusCode::NOT_FOUND,
            format!("Failed to delete task: {}", e),
        )),
    }
}


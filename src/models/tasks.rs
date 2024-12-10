use serde::{Deserialize, Serialize};
use chrono::NaiveDateTime;
use crate::models::status::Status;

#[derive(Deserialize, Serialize, Debug)]
pub struct Task {
    pub id: u64,               
    pub description: String,
    pub status: Status,         
    pub created_at: NaiveDateTime,  
    pub updated_at: Option<NaiveDateTime>,  
}

#[derive(Deserialize, Serialize, Debug)]
pub struct CreateTask {
    pub description: String,  
}

#[derive(Deserialize, Serialize, Debug)]
pub struct UpdateTask {
    pub description: Option<String>,  
    pub status: Option<Status>,       
}

#[derive(Deserialize, Serialize, Debug)]
pub struct DeleteTask {
    pub id: u64,
}

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum Status {
    UNCOMPLETED,
    COMPLETED,
    UPDATED,
    CANCELED,
    POSTPONE,
}


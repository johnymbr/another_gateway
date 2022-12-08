use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Pagination {
    pub page: i32,
    #[serde(rename = "pageSize")]
    pub page_size: i32,
}

impl Pagination {
    pub fn offset(&self) -> i32 {
        self.page * self.page_size
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PaginationResponse<T> {
    pub page: i32,
    pub page_size: i32,
    pub total: i32,
    pub elements: Vec<T>,
}

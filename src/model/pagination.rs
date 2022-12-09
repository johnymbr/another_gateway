use serde::{Deserialize, Serialize};

use crate::exception::{ApiError, PG_ERR_PAGE_REQUIRED, PG_ERR_PAGE_SIZE_REQUIRED};

#[derive(Serialize, Deserialize, Debug)]
pub struct Pagination {
    pub page: Option<i64>,
    #[serde(rename = "pageSize")]
    pub page_size: Option<i64>,
}

impl Pagination {
    pub fn offset(&self) -> i64 {
        self.page.unwrap_or(0) * self.page_size.unwrap_or(0)
    }

    pub fn validate(&self) -> Result<(), ApiError> {
        match self.page {
            None => {
                return Err(ApiError::new(PG_ERR_PAGE_REQUIRED));
            },
            _ => {}
        }

        match self.page_size {
            None => {
                return Err(ApiError::new(PG_ERR_PAGE_SIZE_REQUIRED));
            },
            _ => {}
        }

        Ok(())
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PaginationResponse<T> {
    pub page: i64,
    pub page_size: i64,
    pub total: i64,
    pub elements: Vec<T>,
}

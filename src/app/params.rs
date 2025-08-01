use serde::{Deserialize, Serialize};
use serde_with::{DisplayFromStr, serde_as};
use validator::Validate;

#[serde_as]
#[derive(Debug, Deserialize, Validate)]
pub struct QueryParams {
    #[validate(range(min = 1, message = "Page must be greater than 0"))]
    #[serde_as(as = "DisplayFromStr")]
    #[serde(default = "one")]
    pub page: u64,
    #[validate(range(min = 1, max = 100, message = "Page size must be between 1 and 100"))]
    #[serde_as(as = "DisplayFromStr")]
    #[serde(default = "ten")]
    pub page_size: u64,
}

const fn one() -> u64 {
    1
}
const fn ten() -> u64 {
    10
}

#[derive(Debug, Serialize)]
pub struct Page<T> {
    pub page: u64,
    pub page_size: u64,
    pub total: u64,
    pub items: Vec<T>,
}

impl<T> Page<T> {
    pub fn new(page: u64, page_size: u64, total: u64, items: Vec<T>) -> Self {
        Self {
            page,
            page_size,
            total,
            items,
        }
    }

    pub fn from_pagination(
        QueryParams { page, page_size }: QueryParams,
        total: u64,
        items: Vec<T>,
    ) -> Self {
        Self::new(page, page_size, total, items)
    }
}

use serde::Serialize;

pub mod member;

const DEFAULT_PAGE_SIZE: u32 = 30;
#[derive(Serialize)]
pub struct Paginate<T: Serialize> {
    pub total: u32,
    pub total_page: u32,
    pub page: u32,
    pub page_size: u32,
    pub data: T,
}

impl<T: Serialize> Paginate<T> {
    pub fn new(total: u32, page: u32, data: T) -> Self {
        let total_page = f64::ceil(total as f64 / DEFAULT_PAGE_SIZE as f64) as u32;
        Self {
            total,
            page,
            total_page,
            page_size: DEFAULT_PAGE_SIZE,
            data,
        }
    }
    pub fn has_prev(&self) -> bool {
        self.page > 0
    }
    pub fn has_next(&self) -> bool {
        self.page < self.last_page()
    }
    pub fn last_page(&self) -> u32 {
        self.total_page - 1
    }
}

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ListPageResp<T> {
    pub items: Vec<T>,
    pub total: u64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ListPageReq {
    pub page_num: usize,
    pub page_size: usize,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OptionItem<T> {
    pub label: String,
    pub value: T,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OptionsResp<T> {
    pub items: Vec<OptionItem<T>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ArrayItemsResp<T> {
    pub items: Vec<T>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct IDReq {
    pub id: u64,
}

impl Into<u64> for IDReq {
    fn into(self) -> u64 {
        self.id
    }
}

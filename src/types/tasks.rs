use serde::Deserialize;
use serde::Serialize;
use serde_json::Value;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetAllTasks {
    pub results: Vec<Result>,
    pub pagination: Option<Pagination>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Result {
    #[serde(rename = "_id")]
    pub id: String,
    pub tasks: Vec<TaskResponse>,
    #[serde(rename = "total_time")]
    pub total_time: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TaskResponse {
    #[serde(rename = "_id")]
    pub id: String,
    pub name: String,
    #[serde(rename = "initial_time")]
    pub initial_time: String,
    #[serde(rename = "end_time")]
    pub end_time: String,
    pub project: String,
    #[serde(rename = "project_color")]
    pub project_color: String,
    pub client: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Pagination {
    pub previous: Value,
    pub next: String,
    #[serde(rename = "next_page")]
    pub next_page: i64,
    #[serde(rename = "previous_page")]
    pub previous_page: Value,
    #[serde(rename = "total_pages")]
    pub total_pages: i64,
    #[serde(rename = "total_items")]
    pub total_items: i64,
    pub size: i64,
    pub start: i64,
}

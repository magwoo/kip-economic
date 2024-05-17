use axum::http::StatusCode;
use axum::routing::{get, post};
use axum::{extract::State, response::IntoResponse};
use axum::{Form, Router};
use serde::{Deserialize, Serialize};

use crate::database::Database;
use crate::Employee;

pub fn get_nest() -> Router<Database> {
    Router::new()
        .route("/employee", post(add_employee).delete(remove_employee))
        .route("/employees/raw", get(employees_raw))
}

async fn add_employee(
    State(db): State<Database>,
    Form(employee): Form<Employee>,
) -> Result<impl IntoResponse, StatusCode> {
    let headers = [("hx-trigger", "EmployeesRefresh, EmployeeFormClear")];

    db.add(employee)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok((headers, StatusCode::CREATED))
}

#[derive(Deserialize, Serialize)]
struct RemoveEmployeeForm {
    id: u32,
}

async fn remove_employee(
    State(db): State<Database>,
    Form(RemoveEmployeeForm { id }): Form<RemoveEmployeeForm>,
) -> Result<impl IntoResponse, StatusCode> {
    db.remove(id)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(StatusCode::OK)
}

async fn employees_raw(State(db): State<Database>) -> Result<impl IntoResponse, StatusCode> {
    let employees = db
        .get_all()
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(serde_json::to_string(&employees).unwrap())
}

use axum::extract::State;
use axum::http::StatusCode;
use axum::response::{Html, IntoResponse};
use axum::routing::get;
use axum::Router;
use cercis::prelude::*;

use crate::components::*;
use crate::database::Database;

pub fn get_nest() -> Router<Database> {
    Router::new()
        .route("/", get(root_page))
        .route("/employees", get(employees))
        .route("/employee-form", get(employee_form))
}

async fn root_page() -> impl IntoResponse {
    let content = rsx!(Page {
        title: "Экономика",

        MainBlock {
            left: left_block(),
            right: right_block()
        }
        div {
            class: "flex w-[80vw] gap-1 mx-auto xl:w-[48rem] text-stone-400 my-2 justify-center xl:justify-end",

            span { "Выполнил" }
            a {
                class: "text-blue-400 hover:text-blue-500",
                href: "https://github.com/magwoo",
                target: "_blank",

                "Бутин Роман"
            }
        }
    });

    Html(content.render())
}

async fn employees(State(db): State<Database>) -> Result<impl IntoResponse, StatusCode> {
    let employees = db
        .get_all()
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let content = rsx!(div {
        id: "employees-history",
        class: "flex flex-col gap-3",
        hx_get: "/employees",
        hx_swap: "outerHTML",
        hx_trigger: "EmployeesRefresh from:body, every 3s",

        for (id, employee) in employees {
            HistoryCard {
                name: employee.full_name,
                id: id,
                salary: employee.salary,
                bonus: employee.bonus,
                mult: employee.rating,
            }
        }
    });

    Ok(Html(content.render()))
}

async fn employee_form() -> impl IntoResponse {
    Html(form().render())
}

fn left_block() -> Element {
    const BUTTON_TW: &str = "px-3 py-2 xl:py-1 text-lg xl:text-base rounded-md text-stone-100 transition-colors w-full xl:w-min";
    let form = form();

    rsx!(InnerBlock {
        title: "Сотрудник",
        class: "flex flex-col justify-between",

        form
        div {
            class: "flex justify-end items-center gap-3 mt-3 xl:mt-0",

            Icon {
                id: "form-indicator",
                class: "hidden xl:flex text-stone-300 font-bold animate-spin htmx-indicator",

                "progress_activity"
            }
            button {
                class: "{BUTTON_TW} bg-gray-500 hover:bg-gray-600",
                hx_target: "#employee-form",
                hx_indicator: "#form-indicator",
                hx_get: "/employee-form",
                hx_swap: "outerHTML transition:true",

                "Очистить"
            }
            button {
                id: "employee-done",
                class: "{BUTTON_TW} bg-blue-500 hover:bg-blue-600",

                "Готово"
            }
        }
    })
}

fn right_block() -> Element {
    rsx!(InnerBlock {
        title: "История",
        class: "flex flex-col gap-3 overflow-y-auto rounded-md",
        buttons: rsx!(a {
            class: "hidden xl:flex text-stone-500 h-0 hover:text-blue-500",
            download: "employees.yml",
            href: "/employees/raw",

            Icon { "download" }
        }),

        div {
            hx_get: "/employees",
            hx_trigger: "load",
            hx_swap: "outerHTML",

            div { class: "bg-stone-300 mx-auto mt-12 w-4 h-4 rounded-full animate-ping" }
        }
    })
}

fn form() -> Element {
    const INPUT_TW: &str = "px-2 py-1.5 grow rounded-md bg-transparent text-stone-600 outline-none invalid:text-red-500";
    const LABEL_ANIM_TW: &str =
        "absolute text-stone-400 peer-placeholder-shown:left-12 peer-placeholder-shown:text-base
            peer-placeholder-shown:top-[0.35rem] left-3 text-sm top-[-0.75rem] peer-focus:text-sm
            peer-focus:left-3 peer-focus:top-[-0.75rem] transition-all pointer-events-none";

    rsx!(form {
        id: "employee-form",
        class: "flex flex-col gap-3",
        hx_post: "/employee",
        hx_swap: "none",
        hx_trigger: "click from:#employee-done",
        hx_indicator: "#form-indicator",

        div {
            class: "hidden",
            hx_get: "/employee-form",
            hx_trigger: "EmployeeFormClear from:body",
            hx_target: "#employee-form",
            hx_swap: "outerHTML transition:true",
        }

        WithIcon {
            icon: "id_card",
            class: "relative",

            input {
                class: "{INPUT_TW} peer",
                name: "full_name",
                "type": "text",
                placeholder: " "
            }
            label { class: "{LABEL_ANIM_TW}", "ФИО сотрудника" }
        }
        WithIcon {
            icon: "currency_ruble",
            class: "relative",

            input {
                class: "{INPUT_TW} peer out-of-range:text-red-500",
                name: "salary",
                "type": "number",
                placeholder: " ",
                step: 1,
                min: 0,
                max: 2000000000,
            }
            label { class: "{LABEL_ANIM_TW}", "Зарплата" }
        }
        WithIcon {
            icon: "add",
            class: "relative",

            input {
                class: "{INPUT_TW} peer out-of-range:text-red-500",
                name: "bonus",
                "type": "number",
                placeholder: " ",
                step: 1,
                min: 0,
                max: 2000000000,
            }
            label { class: "{LABEL_ANIM_TW}", "Премия" }
        }
        WithIcon {
            icon: "trending_up",
            class: "relative",

            input {
                class: "{INPUT_TW} peer out-of-range:text-red-500",
                name: "rating",
                "type": "number",
                placeholder: " ",
                step: 0.01,
                min: 0,
                max: 1
            }
            label { class: "{LABEL_ANIM_TW}", "Рейтинг" }
        }
    })
}

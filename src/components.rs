use cercis::prelude::*;
use num_format::{Locale, ToFormattedString};

#[component]
pub fn Page<'a>(title: &'a str, head: Element, children: Element) -> Element {
    const META_CONTENT: &str = "witdh=device-width, initial-scale=1.0";
    const GOOGLE_ICONS_URL: &str = 
        "https://fonts.googleapis.com/css2?family=Material+Symbols+Outlined:opsz,wght,FILL,GRAD@20..48,100..700,0..1,-50..200";

    rsx!(
        doctype {}
        html {
            head {
                meta { charset: "UTF-8" }
                meta {
                    name: "viewport",
                    content: "{META_CONTENT}",
                }
                script { src: "https://unpkg.com/htmx.org@1.9.12" }
                script { src: "https://cdn.tailwindcss.com" }
                link { rel: "stylesheet", href: "{GOOGLE_ICONS_URL}" }
                head
                title { "{title}" }
            }
            body {
                class: "bg-stone-200",

                children
            }
        }

    )
}

#[component]
pub fn EmployeeForm() -> Element {
    const INPUT_TW: &str = "px-2 py-1.5 grow rounded-md bg-transparent text-stone-600 outline-none";
    const LABEL_ANIM_TW: &str =
        "absolute text-stone-400 peer-placeholder-shown:left-12 peer-placeholder-shown:text-base
            peer-placeholder-shown:top-[0.35rem] left-3 text-sm top-[-0.75rem] peer-focus:text-sm
            peer-focus:left-3 peer-focus:top-[-0.75rem] transition-all pointer-events-none";

    rsx!(form {
        id: "employee-form",
        class: "flex flex-col gap-3",
        hx_post: "/employee",
        hx_trigger: "click from:#employee-done",

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

#[component]
pub fn MainBlock(left: Element, right: Element) -> Element {
    const DIV_TW: &str = "flex justify-between flex-col xl:flex-row gap-3 mx-auto
        xl:mt-[20vh] bg-stone-100 w-[95vw] mt-[2.5vw] xl:w-[54rem]
        xl:h-96 rounded-xl shadow p-3";
    
    rsx!(div {
        class: "{DIV_TW}",

        left
        div { class: "bg-stone-300/25 h-1 w-full xl:w-2 xl:h-full rounded-full" }
        right
    })
}

#[component]
pub fn InnerBlock<'a>(title: &'a str, class: Option<&'a str>, buttons: Element, children: Element) -> Element {
    let class = class.unwrap_or_default();

    rsx!(div {
        class: "flex flex-col gap-5 w-full",

        div {
            class: "flex justify-center align-center",
            
            h1 {
                class: "text-stone-600 font-bold text-center text-xl grow",

                "{title}"
            }
            buttons
        }
        div {
            class: "h-full {class}",

            children
        }

    })
}

#[component]
pub fn HistoryCard(name: String, id: u32, salary: u32, bonus: u32, mult: f64) -> Element {
    let sum = (*salary as f64 + *bonus as f64 * mult).floor() as u32;
    let sum = sum.to_formatted_string(&Locale::ru);
    let salary = salary.to_formatted_string(&Locale::ru);
    let bonus = bonus.to_formatted_string(&Locale::ru);
    let delete_id = format!("delete-employee-{id}");

    rsx!(div {
        id: "{delete_id}-card",
        class: "flex flex-col gap-2 w-full bg-stone-200 p-2 rounded-md text-stone-600",

        div {
            class: "flex justify-between",

            div {
                class: "flex gap-2",
                
                Icon {
                    class: "text-stone-500",

                    "person"
                }
                span {
                    class: "truncate max-w-64",
                    
                    "{name}"
                }
            }

            form {
                class: "hidden",
                hx_delete: "/employee",
                hx_trigger: "click from:#{delete_id}",
                hx_target: "#{delete_id}-card",
                hx_swap: "delete",

                input { name: "id", value: "{id}" }
            }
            button {
                id: "{delete_id}",
                class: "h-0 text-stone-400 hover:text-red-500 transition-colors",
                
                Icon { "delete" }
            }
        }
        div {
            class: "flex gap-1 text-stone-400",

            Icon {
                class: "font-light text-stone-500",
                
                "functions"
            }

            span { 
                class: "text-blue-500 cursor-pointer hover:text-blue-600",
                
                "{sum}"
            }
            span { "=" }
            span { "{salary}" }
            span { "+" }
            span { "{bonus}" }
            span { "×" }
            span { "{((mult * 100.0) as u32) as f64 / 100.0}" }
        }
    })
}

#[component]
pub fn WithIcon<'a>(class: Option<&'a str>, icon: &'a str, children: Element) -> Element {
    let class = class.unwrap_or_default();

    rsx!(div {
        class: "flex items-center gap-2 {class} bg-stone-200 rounded-md px-3 hover:bg-stone-300/60 transition-colors",

        Icon {
            class: "text-stone-500",
            
            "{icon}" 
        }
        children
    })
}

#[component]
pub fn Icon<'a>(id: Option<&'a str>, class: Option<&'a str>, children: Element) -> Element {
    let id = id.unwrap_or_default();
    let class = class.unwrap_or_default();

    rsx!(span {
        id: "{id}",
        class: "material-symbols-outlined select-none {class}",

        children
    })
}

use axum::{
    http::{header, StatusCode},
    response::{IntoResponse, Response},
    Json,
};
use maud::{html, Markup, PreEscaped};
use serde::Deserialize;

use crate::{
    endpoints::page,
    render::{self, Entry, Note, Timesheet, WorkingArea},
};

pub async fn get() -> Markup {
    page(
        html! {
            style { (PreEscaped(include_str!("tsg.css"))) }
            script type="module" { (PreEscaped(include_str!("tsg.js"))) }
        },
        html! {
            form #form {
                h1 { "Arbeitszeitdokumentationsgenerator" }

                p {
                    "Du kannst deine Daten auch in einem "
                    a href=".." { "coolen Formular" }
                    " eingeben."
                }

                textarea name="global" placeholder="Global.json" {}
                textarea name="month" placeholder="Month.json" {}

                button #submit type="button" { "Arbeitszeitdokumentation generieren" }

                pre #info {}
            }
        },
    )
}

fn default_vacation() -> bool {
    false
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GlobalJson {
    name: String,
    staff_id: i64,
    department: String,
    working_time: String,
    wage: f64,
    working_area: String,
}

#[derive(Debug, Deserialize)]
pub struct EntryJson {
    action: String,
    day: u32,
    start: String,
    end: String,
    pause: Option<String>,
    #[serde(default = "default_vacation")]
    vacation: bool,
}

#[derive(Debug, Deserialize)]
pub struct MonthJson {
    year: u32,
    month: u32,
    pred_transfer: Option<String>,
    entries: Vec<EntryJson>,
}

#[derive(Debug, Deserialize)]
pub struct PostJson {
    global: GlobalJson,
    month: MonthJson,
}

fn error_response<S: ToString>(msg: S) -> Response {
    (StatusCode::BAD_REQUEST, msg.to_string()).into_response()
}

fn parse_span(span_str: &str) -> Option<u32> {
    let mut parts = span_str.split(':');

    let hours = parts.next()?.parse::<u32>().ok()?;
    let minutes = parts.next()?.parse::<u32>().ok()?;

    if parts.next().is_some() {
        return None;
    }

    if minutes != 0 {
        return None;
    }

    Some(hours)
}

pub async fn post(json: Json<PostJson>) -> Response {
    let json = json.0;

    // Parse working area
    let working_area = match &json.global.working_area as &str {
        "gf" => WorkingArea::Großforschung,
        "ub" => WorkingArea::Unibereich,
        _ => {
            return error_response(format!(
                "invalid working area: {:?}",
                json.global.working_area
            ))
        }
    };

    // Parse working time
    let Some(monthly_hours) = parse_span(&json.global.working_time) else {
        return error_response(format!(
            "invalid working_time: {:?}",
            json.global.working_time
        ));
    };

    let entries = json
        .month
        .entries
        .into_iter()
        .map(|e| Entry {
            task: e.action,
            day: e.day,
            start: e.start,
            end: e.end,
            rest: e.pause,
            note: if e.vacation { Some(Note::Urlaub) } else { None },
        })
        .collect::<Vec<_>>();

    let timesheet = Timesheet {
        name: json.global.name,
        staff_id: json.global.staff_id.to_string(),
        department: json.global.department,
        working_area,
        monthly_hours,
        hourly_wage: json.global.wage.to_string(),
        validate: true,
        carry_prev_month: json.month.pred_transfer,
        year: json.month.year,
        month: json.month.month,
        entries,
    };

    match render::render(timesheet) {
        Ok(pdf) => ([(header::CONTENT_TYPE, "application/pdf")], pdf).into_response(),
        Err(errors) => error_response(errors.join("\n")),
    }
}

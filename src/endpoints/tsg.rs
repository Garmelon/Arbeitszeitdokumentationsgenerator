use axum::{
    http::{header, StatusCode},
    response::{IntoResponse, Response},
    Json,
};
use el::{html::*, Document};
use serde::Deserialize;

use crate::{
    endpoints::page,
    render::{self, Entry, Note, Timesheet, WorkingArea},
};

const LINK_SOURCE: &str = "https://github.com/Garmelon/Arbeitszeitdokumentationsgenerator";

pub async fn get() -> Document {
    let head = (
        style(include_str!("tsg.css")),
        script((attr::TypeScript::Module, include_str!("tsg.js"))),
    );

    let body = form((
        attr::id("form"),
        h1((
            "Arbeitszeitdokumentationsgenerator ",
            a((attr::id("source"), attr::href(LINK_SOURCE), "(source)")),
        )),
        p((
            "Du kannst deine Daten auch in einem ",
            a((attr::href(".."), "coolen Formular")),
            " eingeben.",
        )),
        p((
            label((attr::r#for("i-global"), "Global.json")),
            textarea((
                attr::id("i-global"),
                attr::name("global"),
                attr::placeholder("{}"),
            )),
        )),
        p((
            label((attr::r#for("i-month"), "Month.json")),
            textarea((
                attr::id("i-month"),
                attr::name("month"),
                attr::placeholder("{}"),
            )),
        )),
        p((
            label((
                attr::title(concat!(
                    "Die Einträge werden chronologisch sortiert,",
                    " anstatt dass ihre Reihenfolge beibehalten wird."
                )),
                input((
                    attr::name("sort"),
                    attr::TypeInput::Checkbox,
                    attr::checked(),
                )),
                " Einträge sortieren",
            )),
            label((
                attr::title(concat!(
                    "Die Einträge werden auf Konsistenz und Korrektheit überprüft,",
                    " bevor das Dokument generiert wird."
                )),
                input((
                    attr::name("validate"),
                    attr::TypeInput::Checkbox,
                    attr::checked(),
                )),
                " Einträge validieren",
            )),
        )),
        button((
            attr::id("submit"),
            attr::TypeButton::Button,
            "Arbeitszeitdokumentation generieren",
        )),
        pre(attr::id("info")),
    ));

    page(head, body)
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
    sort: bool,
    validate: bool,
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
        validate: json.validate,
        sort: json.sort,
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

use std::iter;

use axum::{
    http::{header, StatusCode},
    response::{IntoResponse, Response},
};
use axum_extra::extract::Form;
use el::{html::*, Document};
use jiff::{ToSpan, Zoned};
use serde::Deserialize;

use crate::{
    endpoints::page,
    render::{self, Entry, Note, Timesheet, WorkingArea},
};

const LINK_SOURCE: &str = "https://github.com/Garmelon/Arbeitszeitdokumentationsgenerator";
const LINK_TSG: &str = "https://github.com/kit-sdq/TimeSheetGenerator";
const LINK_TEMPLATE: &str =
    "https://github.com/Garmelon/Arbeitszeitdokumentationsgenerator/blob/master/kit_timesheet.md";

pub async fn get() -> Document {
    // We assume that people still want to fill out the previous month's time
    // sheet during the first two weeks of the following month.
    let month = Zoned::now()
        .checked_sub(2.weeks())
        .unwrap()
        .strftime("%Y-%m");

    let head = (
        style(include_str!("index.css")),
        script((attr::TypeScript::Module, include_str!("index.js"))),
    );

    let body = form((
        attr::id("form"),
        h1((
            "Arbeitszeitdokumentationsgenerator ",
            a((attr::id("source"), attr::href(LINK_SOURCE), "(source)")),
        )),
        p((
            "Du kannst auch ",
            a((attr::href("tsg/"), "JSON eingeben")),
            ", das kompatibel mit dem ",
            a((attr::href(LINK_TSG), "TimeSheetGenerator")),
            " ist, oder das dem Generator zugrunde liegende ",
            a((attr::href(LINK_TEMPLATE), "Typst-Template")),
            " direkt benutzen.",
        )),
        div((
            attr::id("header"),
            label((attr::id("l-month"), attr::r#for("i-month"), "Monat / Jahr:")),
            input((
                attr::id("i-month"),
                attr::name("month"),
                attr::TypeInput::Month,
                attr::placeholder(&month),
                attr::value(&month),
            )),
            label((
                attr::id("l-name"),
                attr::r#for("i-name"),
                "Name, Vorname des/r Beschäftigten:",
            )),
            input((
                attr::id("i-name"),
                attr::class("twocol"),
                attr::name("name"),
                attr::TypeInput::Text,
                attr::placeholder("McStudentface, Student"),
            )),
            label((
                attr::id("l-staffid"),
                attr::r#for("i-staffid"),
                "Personalnummer:",
            )),
            input((
                attr::id("i-staffid"),
                attr::name("staff_id"),
                attr::TypeInput::Text,
                attr::placeholder("1337420"),
            )),
            div((
                attr::id("gfub"),
                label((
                    attr::id("l-gf"),
                    attr::title("Großforschung"),
                    "GF: ",
                    input((
                        attr::id("i-gf"),
                        attr::name("working_area"),
                        attr::TypeInput::Radio,
                        attr::value("GF"),
                    )),
                )),
                label((
                    attr::id("l-ub"),
                    attr::title("Unibereich"),
                    "UB: ",
                    input((
                        attr::id("i-ub"),
                        attr::name("working_area"),
                        attr::TypeInput::Radio,
                        attr::value("UB"),
                        attr::checked(),
                    )),
                )),
            )),
            label((
                attr::id("l-department"),
                attr::r#for("i-department"),
                attr::title("Institut/Organisationseinheit"),
                "OE:",
            )),
            input((
                attr::id("i-department"),
                attr::class("twocol"),
                attr::name("department"),
                attr::TypeInput::Text,
                attr::placeholder("Institut für Informatik"),
                attr::value("Institut für Informatik"),
            )),
            label((
                attr::id("l-monthlyhours"),
                attr::r#for("i-monthlyhours"),
                "Vertraglich vereinbarte Arbeitszeit:",
            )),
            div((
                attr::id("mhhr"),
                attr::class("twocol"),
                span((
                    input((
                        attr::id("i-monthlyhours"),
                        attr::name("monthly_hours"),
                        attr::TypeInput::Number,
                        attr::value(40),
                        attr::min(0),
                    )),
                    " Std.",
                )),
                span((
                    label((
                        attr::id("l-hourlywage"),
                        attr::r#for("i-hourlywage"),
                        "Stundensatz: ",
                    )),
                    input((
                        attr::id("i-hourlywage"),
                        attr::name("hourly_wage"),
                        attr::TypeInput::Number,
                        attr::step(0.01),
                        attr::value(14.09),
                    )),
                    " €",
                )),
            )),
            div((
                attr::id("carry"),
                attr::class("twocol"),
                span((
                    label((
                        attr::id("l-carry"),
                        attr::r#for("i-carry"),
                        "Übertrag vom Vormonat: ",
                    )),
                    input((
                        attr::id("i-carry"),
                        attr::class("i-dur"),
                        attr::name("carry_prev_month"),
                        attr::TypeInput::Text,
                        attr::placeholder("00:00"),
                    )),
                )),
            )),
            label((
                attr::id("check"),
                attr::title(concat!(
                    "Die Tabelleneinträge werden chronologisch sortiert,",
                    " anstatt dass ihre Reihenfolge beibehalten wird."
                )),
                "Einträge sortieren ",
                input((
                    attr::name("sort"),
                    attr::TypeInput::Checkbox,
                    attr::value(true),
                    attr::checked(),
                )),
            )),
            label((
                attr::id("validate"),
                attr::title(concat!(
                    "Die Tabelleneinträge werden auf Konsistenz und Korrektheit überprüft,",
                    " bevor das Dokument generiert wird."
                )),
                "Einträge validieren ",
                input((
                    attr::name("validate"),
                    attr::TypeInput::Checkbox,
                    attr::value(true),
                    attr::checked(),
                )),
            )),
        )),
        div((
            attr::id("table"),
            div((
                attr::id("task"),
                "Tätigkeit",
                br(()),
                "(Stichwort, Projekt)",
            )),
            div("Tag"),
            div("Beginn"),
            div("Ende"),
            div("Pause"),
            div("Arbeitszeit"),
            div(()),
            div("(hh:mm)"),
            div("(hh:mm)"),
            div("(hh:mm)"),
            div(()),
            iter::repeat_n(
                (
                    div(input((
                        attr::class("i-task"),
                        attr::name("task"),
                        attr::TypeInput::Text,
                    ))),
                    div(input((
                        attr::class("i-day"),
                        attr::name("day"),
                        attr::TypeInput::Number,
                        attr::placeholder(1),
                        attr::min(1),
                        attr::max(31),
                    ))),
                    div(input((
                        attr::class("i-dur"),
                        attr::name("start"),
                        attr::TypeInput::Text,
                        attr::placeholder("12:34"),
                    ))),
                    div(input((
                        attr::class("i-dur"),
                        attr::name("end"),
                        attr::TypeInput::Text,
                        attr::placeholder("12:34"),
                    ))),
                    div(input((
                        attr::class("i-dur"),
                        attr::name("rest"),
                        attr::TypeInput::Text,
                        attr::placeholder("00:00"),
                    ))),
                    div(select((
                        attr::name("note"),
                        attr::value(""),
                        option((attr::value(""), "Normal")),
                        option((attr::value("U"), "Urlaub")),
                        option((attr::value("K"), "Krankheit")),
                        option((attr::value("F"), "Feiertag")),
                        option((attr::value("S"), "Sonstiges")),
                    ))),
                ),
                22,
            )
            .collect::<Vec<_>>(),
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

#[derive(Debug, Deserialize)]
pub struct PostForm {
    month: String,
    name: String,
    staff_id: String,
    working_area: String,
    department: String,
    monthly_hours: u32,
    hourly_wage: String,
    carry_prev_month: String,
    #[serde(default)]
    sort: bool,
    #[serde(default)]
    validate: bool,
    task: Vec<String>,
    day: Vec<Option<u32>>,
    start: Vec<String>,
    end: Vec<String>,
    rest: Vec<String>,
    note: Vec<String>,
}

fn error_response<S: ToString>(msg: S) -> Response {
    (StatusCode::BAD_REQUEST, msg.to_string()).into_response()
}

fn parse_month(month_str: &str) -> Option<(u32, u32)> {
    let mut parts = month_str.split('-');

    let year = parts.next()?.parse::<u32>().ok()?;
    let month = parts.next()?.parse::<u32>().ok()?;

    if parts.next().is_some() {
        return None;
    }

    Some((year, month))
}

pub async fn post(form: Form<PostForm>) -> Response {
    let form = form.0;

    // Parse working area
    let working_area = match &form.working_area as &str {
        "GF" => WorkingArea::Großforschung,
        "UB" => WorkingArea::Unibereich,
        _ => return error_response(format!("invalid working area: {:?}", form.working_area)),
    };

    // Parse month
    let Some((year, month)) = parse_month(&form.month) else {
        return error_response(format!("invalid month: {:?}", form.month));
    };

    // Parse rests
    let rests = form
        .rest
        .into_iter()
        .map(|r| if r.is_empty() { None } else { Some(r) })
        .collect::<Vec<_>>();

    // Parse notes
    let mut notes = vec![];
    for note in form.note {
        let note = match &note as &str {
            "" => None,
            "U" => Some(Note::Urlaub),
            "K" => Some(Note::Krankheit),
            "F" => Some(Note::Feiertag),
            "S" => Some(Note::Sonstiges),
            _ => return error_response(format!("invalid note: {note:?}")),
        };
        notes.push(note)
    }

    // Parse carry
    let carry_prev_month = if form.carry_prev_month.is_empty() {
        None
    } else {
        Some(form.carry_prev_month)
    };

    let entries = (form.task.into_iter())
        .zip(form.day.into_iter())
        .zip(form.start.into_iter())
        .zip(form.end.into_iter())
        .zip(rests.into_iter())
        .zip(notes.into_iter())
        .filter_map(|(((((task, day), start), end), rest), note)| {
            if task.is_empty() || start.is_empty() || end.is_empty() {
                return None;
            };
            Some(Entry {
                task,
                day: day?,
                start,
                end,
                rest,
                note,
            })
        })
        .collect::<Vec<_>>();

    let timesheet = Timesheet {
        name: form.name,
        staff_id: form.staff_id,
        department: form.department,
        working_area,
        monthly_hours: form.monthly_hours,
        hourly_wage: form.hourly_wage,
        validate: form.validate,
        sort: form.sort,
        carry_prev_month,
        year,
        month,
        entries,
    };

    match render::render(timesheet) {
        Ok(pdf) => ([(header::CONTENT_TYPE, "application/pdf")], pdf).into_response(),
        Err(errors) => error_response(errors.join("\n")),
    }
}

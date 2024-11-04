use axum::{
    http::{header, StatusCode},
    response::{IntoResponse, Response},
};
use axum_extra::extract::Form;
use jiff::{ToSpan, Zoned};
use maud::{html, Markup, PreEscaped};
use serde::Deserialize;

use crate::{
    endpoints::page,
    render::{self, Entry, Note, Timesheet, WorkingArea},
};

pub async fn get() -> Markup {
    // We assume that people still want to fill out the previous month's time
    // sheet during the first two weeks of the following month.
    let month = Zoned::now()
        .checked_sub(2.weeks())
        .unwrap()
        .strftime("%Y-%m");

    page(
        html! {
            style { (PreEscaped(include_str!("index.css"))) }
            script type="module" { (PreEscaped(include_str!("index.js"))) }
        },
        html! {
            form #form {
                h1 {
                    "Arbeitszeitdokumentationsgenerator "
                    a #source href="https://github.com/Garmelon/Arbeitszeitdokumentationsgenerator" { "(source)" }
                }

                p {
                    "Du kannst auch "
                    a href="tsg/" { "JSON eingeben" }
                    ", das kompatibel mit dem "
                    a href="https://github.com/kit-sdq/TimeSheetGenerator" { "TimeSheetGenerator" }
                    " ist, oder das dem Generator zugrunde liegende "
                    a href="https://github.com/Garmelon/Arbeitszeitdokumentationsgenerator/blob/master/kit_timesheet.md" { "Typst-Template" }
                    " direkt benutzen."
                }

                div #header {
                    label #l-month for="i-month" { "Monat / Jahr:" }
                    input #i-month name="month" type="month" placeholder=(month) value=(month) {}

                    label #l-name for="i-name" { "Name, Vorname des/r Beschäftigten:" }
                    input #i-name .twocol name="name" type="text" placeholder="McStudentface, Student" {}

                    label #l-staffid for="i-staffid" { "Personalnummer:" }
                    input #i-staffid name="staff_id" type="text" placeholder="1337420" {}

                    div #gfub {
                        label #l-gf title="Großforschung" { "GF: "
                            input #i-gf name="working_area" type="radio" value="GF" {}
                        }

                        label #l-ub for="i-ub" title="Unibereich" { "UB: "
                            input #i-ub name="working_area" type="radio" value="UB" checked {}
                        }
                    }

                    label #l-department for="i-department" title="Organisationseinheit" { "OE:" }
                    input #i-department .twocol name="department" type="text" placeholder="Institut für Informatik" value="Institut für Informatik" {}

                    label #l-monthlyhours for="i-monthlyhours" { "Vertraglich vereinbarte Arbeitszeit:" }
                    div #mhhr .twocol {
                        span {
                            input #i-monthlyhours name="monthly_hours" type="number" value="40" min="0" {}
                            " Std."
                        }
                        span {
                            label #l-hourlywage for="i-hourlywage" { "Stundensatz: " }
                            input #i-hourlywage name="hourly_wage" type="number" step="0.01" value="14.09" {}
                            " €"
                        }
                    }

                    div #carry .twocol {
                        span {
                            label #l-carry for="i-carry" { "Übertrag vom Vormonat: " }
                            input #i-carry .i-dur name="carry_prev_month" type="text" placeholder="00:00" {}
                        }
                    }

                    label #check title="Die Tabelleneinträge werden chronologisch sortiert, anstatt dass ihre Reihenfolge beibehalten wird." {
                        "Einträge sortieren "
                        input name="sort" type="checkbox" value="true" checked {}
                    }

                    label #validate title="Die Tabelleneinträge werden auf Konsistenz und Korrektheit überprüft, bevor das Dokument generiert wird." {
                        "Einträge validieren "
                        input name="validate" type="checkbox" value="true" checked {}
                    }
                }

                div #table {
                    div #task { "Tätigkeit" br; "(Stichwort, Projekt)" }
                    div { "Tag" }
                    div { "Beginn" }
                    div { "Ende" }
                    div { "Pause" }
                    div { "Arbeitszeit" }
                    div { }
                    div { "(hh:mm)" }
                    div { "(hh:mm)" }
                    div { "(hh:mm)" }
                    div { }

                    @for _ in 0..22 {
                        div { input .i-task name="task" type="text" {} }
                        div { input .i-day name="day" type="number" placeholder="1" min="1" max="31" {} }
                        div { input .i-dur name="start" type="text" placeholder="12:34" {} }
                        div { input .i-dur name="end" type="text" placeholder="12:34" {} }
                        div { input .i-dur name="rest" type="text" placeholder="00:00" {} }
                        div { select name="note" value="" {
                            option value="" { "Normal" }
                            option value="U" { "Urlaub" }
                            option value="K" { "Krankheit" }
                            option value="F" { "Feiertag" }
                            option value="S" { "Sonstiges" }
                        } }
                    }
                }

                button #submit type="button" { "Arbeitszeitdokumentation generieren" }

                pre #info {}
            }
        },
    )
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

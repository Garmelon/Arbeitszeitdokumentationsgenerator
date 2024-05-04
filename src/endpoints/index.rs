use maud::{html, Markup, PreEscaped};
use time::{macros::format_description, Date, OffsetDateTime};

use crate::endpoints::page;

pub async fn get() -> Markup {
    let now = OffsetDateTime::now_local().unwrap_or_else(|_| OffsetDateTime::now_utc());
    let month = now.date().replace_day(1).unwrap().previous_day().unwrap();
    let month = month.format(format_description!("[year]-[month]")).unwrap();

    page(
        html! {
            style { (PreEscaped(include_str!("base.css"))) }
        },
        html! {
            form {
                h1 { "Arbeitszeitdokumentationsgenerator" }

                div #header {
                    label #l-month for="i-month" { "Monat / Jahr:" }
                    input #i-month name="month" type="month" placeholder=(month) value=(month) {}

                    label #l-name for="i-name" { "Name, Vorname des/r Beschäftigten:" }
                    input #i-name name="name" type="text" placeholder="McStudentface, Student" {}

                    label #l-staffid for="i-staffid" { "Personalnummer:" }
                    input #i-staffid name="staff_id" type="number" placeholder="1337420" {}

                    div #gfub {
                        label #l-gf title="Großforschung" { "GF: "
                            input #i-gf name="working_area" type="radio" value="gf" {}
                        }

                        label #l-ub for="i-ub" title="Unibereich" { "UB: "
                            input #i-ub name="working_area" type="radio" value="ub" {}
                        }
                    }

                    label #l-department for="i-department" title="Organisationseinheit" { "OE:" }
                    input #i-department name="department" type="text" placeholder="Institut für Informatik" value="Institut für Informatik" {}

                    label #l-monthlyhours for="i-monthlyhours" { "Vertraglich vereinbarte Arbeitszeit:" }
                    div #mhhr {
                        span {
                            input #i-monthlyhours name="monthly_hours" type="number" value="40" {}
                            " Std."
                        }
                        label #l-hourlyrate for="i-hourlyrate" { "Stundensatz:" }
                        span {
                            input #i-hourlyrate name="hourly_rate" type="number" step="0.01" placeholder="14.09" {}
                            " €"
                        }
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
                        div { input .i-task name="task[]" type="text" {} }
                        div { input .i-day name="day[]" type="number" value="1" {} }
                        div { input .i-dur name="start[]" type="text" placeholder="12:34" {} }
                        div { input .i-dur name="end[]" type="text" placeholder="12:34" {} }
                        div { input .i-dur name="pause[]" type="text" placeholder="01:23" value="00:00" {} }
                        div { select name="note[]" value="" {
                            option value="" { "Normal" }
                            option value="U" { "Urlaub" }
                            option value="K" { "Krankheit" }
                            option value="F" { "Feiertag" }
                            option value="S" { "Sonstiges" }
                        } }
                    }
                }
            }
        },
    )
}

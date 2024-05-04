use maud::{html, Markup};

use crate::endpoints::page;

pub async fn get() -> Markup {
    page(
        html! {
            style { (include_str!("base.css")) }
        },
        html! {
            form {
                h1 { "Arbeitszeitdokumentationsgenerator" }

                div #header {
                    label #l-month for="i-month" { "Monat / Jahr:" }
                    input #i-month name="month" type="month" placeholder="2024-04" {} // TODO Fill in previous month by default

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
            }
        },
    )
}

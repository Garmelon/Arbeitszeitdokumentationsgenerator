use maud::{html, Markup};

pub mod index;
pub mod tsg;

fn page(head: Markup, body: Markup) -> Markup {
    html! {
        (maud::DOCTYPE)
        html lang="en" {
            head {
                meta charset="utf-8";
                meta name="viewport" content="width=device-width, initial-scale=1";
                title { "AbzDokGen" }
                (head)
            }
            body { (body) }
        }
    }
}

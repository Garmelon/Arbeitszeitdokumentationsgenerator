use el::{html::*, Document, ElementComponent};

pub mod index;
pub mod tsg;

fn page(head: impl ElementComponent, body: impl ElementComponent) -> Document {
    html((
        el::html::head((
            meta((
                attr::name("viewport"),
                attr::content("width=device-width, initial-scale=1"),
            )),
            title("AbzDokGen"),
            head,
        )),
        el::html::body(body),
    ))
    .into_document()
}

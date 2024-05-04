use comemo::Prehashed;
use typst::{
    diag::{FileError, FileResult, SourceResult},
    eval::Tracer,
    foundations::{Bytes, Datetime, Smart},
    model::Document,
    syntax::{FileId, Source},
    text::{Font, FontBook},
    Library, World,
};

const LOGO: &str = include_str!("../kit_logo.svg");
const LOGO_NAME: &str = "kit_logo.svg";

const TEMPLATE: &str = include_str!("../kit_timesheet.typ");
const TEMPLATE_NAME: &str = "kit_timesheet.typ";

const ALIAS: &str = "ts";

//////////
// Data //
//////////

pub enum WorkingArea {
    Großforschung,
    Unibereich,
}

pub enum Note {
    Urlaub,
    Krankheit,
    Feiertag,
    Sonstiges,
}

pub struct Entry {
    pub task: String,
    pub day: u32,
    pub start: String,
    pub end: String,
    pub rest: Option<String>,
    pub note: Option<Note>,
}

pub struct Timesheet {
    pub name: String,
    pub staff_id: String,
    pub department: String,
    pub working_area: WorkingArea,
    pub monthly_hours: u32,
    pub hourly_wage: String,
    pub validate: bool,
    pub year: u32,
    pub month: u32,
    pub entries: Vec<Entry>,
}

///////////////////////
// Convert to source //
///////////////////////

fn fmt_str(s: &str) -> String {
    // https://github.com/typst/typst/blob/v0.11.0/crates/typst-syntax/src/lexer.rs#L699-L713
    // https://github.com/typst/typst/blob/v0.11.0/crates/typst-syntax/src/ast.rs#L1041-L1082
    let quoted = s
        .chars()
        .map(|c| match c {
            '"' => "\\\"".to_string(),
            '\\' => "\\\\".to_string(),
            c => c.to_string(),
        })
        .collect::<String>();

    format!("\"{}\"", quoted)
}

fn fmt_int(n: u32) -> String {
    n.to_string()
}

fn fmt_bool(b: bool) -> String {
    match b {
        true => "true",
        false => "false",
    }
    .to_string()
}

fn fmt_area(area: WorkingArea) -> String {
    let name = match area {
        WorkingArea::Großforschung => "Großforschung",
        WorkingArea::Unibereich => "Unibereich",
    };
    format!("{ALIAS}.areas.{name}")
}

fn fmt_note(note: Note) -> String {
    let name = match note {
        Note::Urlaub => "Urlaub",
        Note::Krankheit => "Krankheit",
        Note::Feiertag => "Feiertag",
        Note::Sonstiges => "Sonstiges",
    };
    format!("{ALIAS}.notes.{name}")
}

fn fmt_entry(entry: Entry) -> String {
    let mut args = vec![
        fmt_str(&entry.task),
        fmt_int(entry.day),
        fmt_str(&entry.start),
        fmt_str(&entry.end),
    ];

    if let Some(rest) = entry.rest {
        args.push(format!("rest: {}", fmt_str(&rest)));
    }

    if let Some(note) = entry.note {
        args.push(format!("note: {}", fmt_note(note)));
    }

    format!("{ALIAS}.entry({})", args.join(", "))
}

fn fmt_timesheet(ts: Timesheet) -> String {
    let mut lines = vec![];

    lines.push(format!("#import {} as {ALIAS}", fmt_str(TEMPLATE_NAME)));
    lines.push(format!("#{ALIAS}.timesheet("));
    lines.push(format!("  name: {},", fmt_str(&ts.name)));
    lines.push(format!("  staff_id: {},", fmt_str(&ts.staff_id)));
    lines.push(format!("  department: {},", fmt_str(&ts.department)));
    lines.push(format!("  working_area: {},", fmt_area(ts.working_area)));
    lines.push(format!("  monthly_hours: {},", fmt_int(ts.monthly_hours)));
    lines.push(format!("  hourly_wage: {},", fmt_str(&ts.hourly_wage)));
    lines.push(format!("  validate: {},", fmt_bool(ts.validate)));
    lines.push(format!("  year: {},", fmt_int(ts.year)));
    lines.push(format!("  month: {},", fmt_int(ts.month)));
    for entry in ts.entries {
        lines.push(format!("  {},", fmt_entry(entry)));
    }
    lines.push(")".to_string());
    lines.join("\n")
}

/////////////////////
// Evaluate source //
/////////////////////

struct DummyWorld {
    library: Prehashed<Library>,
    book: Prehashed<FontBook>,
    main: Source,
}

impl DummyWorld {
    fn new(main: String) -> Self {
        Self {
            library: Prehashed::new(Library::builder().build()),
            book: Prehashed::new(FontBook::new()),
            main: Source::detached(main),
        }
    }
}

impl World for DummyWorld {
    fn library(&self) -> &Prehashed<Library> {
        &self.library
    }

    fn book(&self) -> &Prehashed<FontBook> {
        &self.book
    }

    fn main(&self) -> Source {
        self.main.clone()
    }

    fn source(&self, id: FileId) -> FileResult<Source> {
        let path = id.vpath().as_rootless_path();
        match path.to_string_lossy().as_ref() {
            TEMPLATE_NAME => Ok(Source::new(id, TEMPLATE.to_string())),
            _ => Err(FileError::NotFound(path.to_path_buf())),
        }
    }

    fn file(&self, id: FileId) -> FileResult<Bytes> {
        let path = id.vpath().as_rootless_path();
        match path.to_string_lossy().as_ref() {
            LOGO_NAME => Ok(Bytes::from_static(LOGO.as_bytes())),
            _ => Err(FileError::NotFound(path.to_path_buf())),
        }
    }

    fn font(&self, _index: usize) -> Option<Font> {
        panic!("this should never be called")
    }

    fn today(&self, _offset: Option<i64>) -> Option<Datetime> {
        panic!("this should never be called")
    }
}

fn compile_timesheet(ts: Timesheet) -> SourceResult<Document> {
    let world = DummyWorld::new(fmt_timesheet(ts));
    let mut tracer = Tracer::new();
    typst::compile(&world, &mut tracer)
}

pub fn render(ts: Timesheet) -> Result<Vec<u8>, Vec<String>> {
    let document = compile_timesheet(ts)
        .map_err(|es| es.iter().map(|e| e.message.to_string()).collect::<Vec<_>>())?;

    Ok(typst_pdf::pdf(&document, Smart::Auto, None))
}

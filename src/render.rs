use std::{fs, path::PathBuf, sync::OnceLock};

use typst::{
    diag::{FileError, FileResult, SourceResult},
    foundations::{Bytes, Datetime},
    syntax::{FileId, Source},
    text::{Font, FontBook, FontInfo},
    utils::LazyHash,
    Library, World,
};
use typst_pdf::PdfOptions;

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
    pub carry_prev_month: Option<String>,
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
    if let Some(carry) = ts.carry_prev_month {
        lines.push(format!("  carry_prev_month: {},", fmt_str(&carry)));
    }
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

// The logic for detecting and loading fonts was ripped straight from:
// https://github.com/typst/typst/blob/69dcc89d84176838c293b2d59747cd65e28843ad/crates/typst-cli/src/fonts.rs
// https://github.com/typst/typst/blob/69dcc89d84176838c293b2d59747cd65e28843ad/crates/typst-cli/src/world.rs#L193-L195

struct FontSlot {
    path: PathBuf,
    index: u32,
    font: OnceLock<Option<Font>>,
}

impl FontSlot {
    pub fn get(&self) -> Option<Font> {
        self.font
            .get_or_init(|| {
                let data = fs::read(&self.path).ok()?.into();
                Font::new(data, self.index)
            })
            .clone()
    }
}

fn load_system_fonts() -> (FontBook, Vec<FontSlot>) {
    let mut book = FontBook::new();
    let mut fonts = vec![];

    let mut db = fontdb::Database::new();
    db.load_system_fonts();

    for face in db.faces() {
        let path = match &face.source {
            fontdb::Source::File(path) | fontdb::Source::SharedFile(path, _) => path,
            fontdb::Source::Binary(_) => continue,
        };

        if let Some(info) = db.with_face_data(face.id, FontInfo::new).unwrap() {
            book.push(info);
            fonts.push(FontSlot {
                path: path.clone(),
                index: face.index,
                font: OnceLock::new(),
            })
        }
    }

    (book, fonts)
}

struct DummyWorld {
    library: LazyHash<Library>,
    book: LazyHash<FontBook>,
    main: Source,
    fonts: Vec<FontSlot>,
}

impl DummyWorld {
    fn new(main: String) -> Self {
        let (book, fonts) = load_system_fonts();
        Self {
            library: LazyHash::new(Library::builder().build()),
            book: LazyHash::new(book),
            main: Source::detached(main),
            fonts,
        }
    }
}

impl World for DummyWorld {
    fn library(&self) -> &LazyHash<Library> {
        &self.library
    }

    fn book(&self) -> &LazyHash<FontBook> {
        &self.book
    }

    fn main(&self) -> FileId {
        self.main.id()
    }

    fn source(&self, id: FileId) -> FileResult<Source> {
        if id == self.main.id() {
            return Ok(self.main.clone());
        }

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

    fn font(&self, index: usize) -> Option<Font> {
        self.fonts[index].get()
    }

    fn today(&self, _offset: Option<i64>) -> Option<Datetime> {
        panic!("datetimes are not supported")
    }
}

fn render_pdf(ts: Timesheet) -> SourceResult<Vec<u8>> {
    let world = DummyWorld::new(fmt_timesheet(ts));
    let document = typst::compile(&world).output?;
    let options = PdfOptions::default();
    typst_pdf::pdf(&document, &options)
}

pub fn render(ts: Timesheet) -> Result<Vec<u8>, Vec<String>> {
    render_pdf(ts).map_err(|es| es.iter().map(|e| e.message.to_string()).collect::<Vec<_>>())
}

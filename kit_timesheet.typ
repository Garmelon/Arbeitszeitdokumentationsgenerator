/////////////
// "Enums" //
/////////////

#let areas = (Großforschung: "GF", Unibereich: "UB")
#let notes = (Urlaub: "U", Krankheit: "K", Feiertag: "F", Sonstiges: "S")

////////////
// Layout //
////////////

#let _kit_green = rgb("#009682")
#let _kit_stroke = 0.2mm

#let _frame(body) = {
  set text(lang: "de", font: "Liberation Sans")
  set page(margin: (top: 9.5mm, bottom: 12mm, left: 15mm, right: 10mm))
  set block(spacing: 1mm)
  set par(leading: 5pt)

  // Weird vertical text in the bottom left
  place(bottom + left, dx: -6mm, dy: -1mm,
    rotate(-90deg, origin: bottom + left,
      text(size: 6pt, tracking: 3.1pt,
        "K_PSE_PB_AZDoku_01_04-20"
      )
    )
  )

  // Main box
  rect(
    width: 100%,
    height: 100%,
    stroke: _kit_stroke,
    radius: (top-right: 4.5mm, bottom-left: 4.5mm),
    inset: 0mm
  )[
    // Logo
    #place(top + left, dx: 9.5mm, dy: 3.5mm,
      image("kit_logo.svg", width: 29mm)
    )

    // Heading
    #place(top + left, dx: 78mm, dy: 9mm,
      text(weight: "bold", size: 14pt, fill: _kit_green,
        [Arbeitszeitdokumentation]
      )
    )

    // Page number
    #place(bottom + right, dx: -15mm, dy: -1.5mm,
      text(size: 9pt, [Seite 1 von 1])
    )

    // Main content
    #block(
      inset: (top: 24.5mm, left: 7.5mm, right: 13mm),
      width: 100%,
      height: 100%,
      body
    )
  ]
}

#let _underlined(body) = box(
  baseline: 0%,
  stroke: (bottom: _kit_stroke),
  outset: (bottom: 1.5mm),
  inset: (x: 1mm),
  body,
)

#let _checkbox(checked) = box(
  width: 3.5mm,
  outset: (y: 0.4mm),
  stroke: _kit_stroke,
  align(center + horizon, if checked { "X" } else { " " })
)

#let _header(
  year: " ",
  month: " ",
  name: " ",
  staff_id: " ",
  working_area: " ",
  department: " ",
  monthly_hours: " ",
  hourly_wage: " ",
) = [
  #set text(size: 11pt)
  #pad(left: 2.5mm, grid(
    columns: (1fr, 92mm),
    rows: 6mm,

    [],
    align(right)[
      *Monat / Jahr:* #h(6mm)
      #_underlined(align(center)[
        #box(width: 21mm)[#month] / #box(width: 21mm)[#year]
      ])
    ],

    [*Name, Vorname des/r Beschäftigten:*],
    _underlined(box(width: 100%, name)),

    [*Personalnummer:*],
    _underlined[
      #box(width: 1fr)[#staff_id]
      #h(8mm)
      *GF:* #h(1mm)
      #_checkbox(working_area == areas.Großforschung)
      #h(8mm)
      *UB:* #h(1mm)
      #_checkbox(working_area == areas.Unibereich)
      #h(8mm)
    ],

    [*OE:*], // Institut / Organisationseinheit
    _underlined(box(width: 100%)[#department]),

    [*Vertraglich vereinbarte Arbeitszeit:*],
    [
      #_underlined(align(center)[
        #h(4mm)
        #box(width: 10mm)[#monthly_hours]
        Std.
        #h(4mm)
      ])
      #h(1fr)
      *Stundensatz:*
      #h(4mm)
      #_underlined(align(center)[
        #box(width: 18mm)[#hourly_wage]
        *€*
      ])
    ]
  ))
]

#let _log(..entries) = {
  set text(size: 10pt)
  table(
    columns: (1fr, 23.3mm, 23.3mm, 23.3mm, 23.3mm, 23.3mm),
    rows: array.range(24).map(_ => 5.05mm),
    align: center + horizon,
    stroke: _kit_stroke,
    inset: 1mm,
    table.header(
      table.cell(rowspan: 2)[
        *Tätigkeit* \
        *(Stichwort, Projekt)*
      ],
      [*Datum*],
      [*Beginn*],
      [*Ende*],
      [*Pause*],
      [*Arbeitszeit#super[1]*],
      [*(tt.mm.jj)*],
      [*(hh:mm)*],
      [*(hh:mm)*],
      [*(hh:mm)*],
      [*(hh:mm)*],
    ),
    ..entries.pos()
  )
}

#let _summary(
  holiday: [],
  total: [],
  monthly_hours: [],
  carry_prev_month: [],
  carry_next_month: [],
) = {
  set text(size: 10pt)
  align(right, table(
    columns: (54mm, 23.3mm),
    rows: 5.05mm,
    align: center + horizon,
    stroke: _kit_stroke,
    inset: 1mm,
    [*Urlaub anteilig:*], [#holiday],
    [*Summe:*], [#total],
    [*monatliche Soll-Arbeitszeit:*], [#monthly_hours],
    [*Übertrag vom Vormonat:*], [#carry_prev_month],
    [*Übertrag in den Folgemonat:*], [#carry_next_month],
  ))
}

#let _footer() = pad(left: 2.5mm)[
  #v(3.5mm)
  #grid(
    columns: (1fr, 77.5mm),
    column-gutter: 6.5mm,
    row-gutter: (12mm, 3mm),
    [Ich bestätige die Richtigkeit der Angaben:],
    [Geprüft:],
    pad(left: -2.5mm, line(length: 100%, stroke: stroke(thickness: _kit_stroke, dash: "densely-dotted"))),
    pad(left: -2.5mm, line(length: 100%, stroke: stroke(thickness: _kit_stroke, dash: "densely-dotted"))),
    [Datum, Unterschrift Beschäftigte/r],
    [Datum, Unterschrift Dienstvorgesetzte/r],
  )


  #v(5.5mm)

  #set text(size: 10pt)
  Nach *§ 17 Mindestlohngesetz (MiLoG)* müssen für geringfügig entlohnte und kurzfristig beschäftigte
  Arbeitnehmer/innen u.a. Beginn, Ende und Dauer der täglichen Arbeitszeit aufgezeichnet und für Kon-
  trollzwecke mindestens zwei Jahre am Ort der Beschäftigung aufbewahrt werden.

  #v(12.5mm)
  #line(length: 51mm, stroke: _kit_stroke)
  #v(1mm)

  #set text(size: 9pt)
  #super[1] Summe in vollen Stunden und Minuten ohne Pause (Std:Min); bei Abwesenheit können auch folgende Kürzel
  eingetragen werden: U=Urlaub, K=Krankheit, F=Feiertag, S=Sonstiges
]

//////////
// Util //
//////////

#let _pad_int(n, char: "0", width: 2) = {
  let s = str(n)
  for _ in array.range(width - s.clusters().len()) { char }
  s
}

#let _parse_duration(s) = {
  let matched = s.match(regex("^(-?)([0-9]+):([0-5][0-9])$"))
  assert(matched != none, message: "invalid duration or time: " + s)
  let groups = matched.captures
  let sign = if groups.at(0) == "-" { -1 } else { 1 }
  let h = int(groups.at(1))
  let m = int(groups.at(2))
  sign * (h * 60 + m)
}

#let _fmt_duration(mins) = {
  if mins < 0 {
    "-"
    mins = -mins
  }
  let h = int(mins / 60)
  let m = mins - h * 60
  _pad_int(h, char: "0", width: 2)
  ":"
  _pad_int(m, char: "0", width: 2)
}

////////////////
// Validation //
////////////////

#let _assert_entry(row, entry, condition, message) = {
  message = "row " + str(row) + " (day " + str(entry.day) + "): " + message
  assert(condition, message: message)
}

#let _check_entry(row, entry) = {
  let e = entry
  _assert_entry(row, e, e.start <= e.end, "start must be before end")
  _assert_entry(row, e, e.rest <= e.end - e.start, "rest too long")

  // I think the previous two checks should make it impossible for this assert
  // to fail, but just to be careful...
  _assert_entry(row, e, e.duration >= 0, "duration must be positive")
}

//////////////////
// Entry points //
//////////////////

#let timesheet_empty() = _frame[
  #_header()
  #_log()
  #_summary()
  #_footer()
]

#let entry(
  task,
  day,
  start,
  end,
  rest: "0:00",
  note: none,
) = {
  assert(type(day) == int)
  assert(note == none or notes.values().contains(note))

  start = _parse_duration(start)
  end = _parse_duration(end)
  rest = _parse_duration(rest)

  (
    task: task,
    day: day,
    start: start,
    end: end,
    rest: rest,
    duration: end - start - rest,
    note: note,
  )
}

#let timesheet(
  name: "Name, Vorname",
  staff_id: 1234567,
  department: "Institut für Informatik",
  working_area: none,
  monthly_hours: 40,
  hourly_wage: [14.09],
  validate: true,
  carry_prev_month: "00:00",
  year: 2024,
  month: 1,
  ..entries,
) = {
  assert(working_area == none or areas.values().contains(working_area))
  assert(type(monthly_hours) == int)
  assert(type(year) == int)
  assert(type(month) == int)

  carry_prev_month = _parse_duration(carry_prev_month)
  entries = entries.pos()

  let monthly = monthly_hours * 60
  let holiday = entries.filter(e => e.note == notes.Urlaub).map(e => e.duration).sum(default: 0)
  let total = entries.map(e => e.duration).sum(default: 0)
  let carry_next_month = carry_prev_month + total - monthly

  if validate {
    for (row, entry) in entries.enumerate(start: 1) {
      _check_entry(row, entry)
    }
  }

  let rows = entries.map(e => (
    e.task,
    datetime(year: year, month: month, day: e.day).display("[day].[month].[year]"),
    _fmt_duration(e.start),
    _fmt_duration(e.end),
    _fmt_duration(e.rest),
    {
      _fmt_duration(e.duration)
      if e.note != none { " "; e.note }
    },
  ))

  _frame[
    #_header(
      year: year,
      month: month,
      name: name,
      staff_id: staff_id,
      working_area: working_area,
      department: department,
      monthly_hours: monthly_hours,
      hourly_wage: hourly_wage,
    )
    #_log(..rows.flatten())
    #_summary(
      holiday: _fmt_duration(holiday),
      total: _fmt_duration(total),
      monthly_hours: _fmt_duration(monthly),
      carry_prev_month: _fmt_duration(carry_prev_month),
      carry_next_month: _fmt_duration(carry_next_month),
    )
    #_footer()
  ]
}

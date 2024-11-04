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
#let _kit_rows = 22

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
    rows: array.range(_kit_rows + 2).map(_ => 5.05mm),
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

#let _divides(divident, divisor) = calc.rem(divident, divisor) == 0
#let _is_leap_year(year) = _divides(year, 4) and not (_divides(year, 100) and not _divides(year, 400))

#let _month_length(year, month) = {
  assert(1 <= month and month <= 12)
  if (1, 3, 5, 7, 8, 10, 12).contains(month) { 31 }
  else if (4, 6, 9, 11).contains(month) { 30 }
  else if _is_leap_year(year) { 29 }
  else { 28 }
}

#let _next_day(date) = {
  let year = date.year()
  let month = date.month()
  let day = date.day()

  if day < _month_length(year, month) {
    day += 1
  } else if month < 12 {
    month += 1
    day = 1
  } else {
    year += 1
    month = 1
    day = 1
  }

  datetime(year: year, month: month, day: day)
}

#let _prev_day(date) = {
  let year = date.year()
  let month = date.month()
  let day = date.day()

  if day > 1 {
    day -= 1
  } else if month > 1 {
    month -= 1
    day = _month_length(year, month)
  } else {
    year -= 1
    month = 12
    day = 31
  }

  datetime(year: year, month: month, day: day)
}

#let _move_by(date, days) = {
  while days > 0 {
    date = _next_day(date)
    days -= 1
  }

  while days < 0 {
    date = _prev_day(date)
    days += 1
  }

  date
}

#let _computus(year) = {
  // https://en.wikipedia.org/wiki/Date_of_Easter#Anonymous_Gregorian_algorithm
  let Y = year
  let a = calc.rem(Y, 19)
  let b = calc.quo(Y, 100)
  let c = calc.rem(Y, 100)
  let d = calc.quo(b, 4)
  let e = calc.rem(b, 4)
  // let f = calc.quo(b + 8, 25)
  // let g = calc.quo(b - f + 1, 3)
  let g = calc.quo(8*b + 13, 25)
  let h = calc.rem(19*a + b - d - g + 15, 30)
  let i = calc.quo(c, 4)
  let k = calc.rem(c, 4)
  let l = calc.rem(32 + 2*e + 2*i - h - k, 7)
  // let m = calc.quo(a + 11*h + 22*l, 451)
  let m = calc.quo(a + 11*h + 19*l, 433)
  // let n = calc.quo(h + l - 7*m + 114, 31)
  let n = calc.quo(h + l - 7*m + 90, 25)
  // let o = calc.rem(h + l - 7*m + 114, 31)
  let p = calc.rem(h + l - 7*m + 33*n + 19, 32)
  let month = n
  // let day = o + 1
  let day = p
  datetime(year: year, month: month, day: day)
}

#let _public_holidays_germany_bw(year) = {
  let easter = _computus(year)
  (
    (name: "Neujahr", date: datetime(year: year, month: 1, day: 1)),
    (name: "Heilige Drei Könige", date: datetime(year: year, month: 1, day: 6)),
    (name: "Karfreitag", date: _move_by(easter, -2)),
    (name: "Ostermontag", date: _move_by(easter, 1)),
    (name: "Tag der Arbeit", date: datetime(year: year, month: 5, day: 1)),
    (name: "Christi Himmelfahrt", date: _move_by(easter, 39)),
    (name: "Pfingstmontag", date: _move_by(easter, 50)),
    (name: "Fronleichnam", date: _move_by(easter, 60)),
    (name: "Tag der Deutschen Einheit", date: datetime(year: year, month: 10, day: 3)),
    (name: "Allerheiligen", date: datetime(year: year, month: 11, day: 1)),
    (name: "Erster Weihnachtsfeiertag", date: datetime(year: year, month: 12, day: 25)),
    (name: "Zweiter Weihnachtsfeiertag", date: datetime(year: year, month: 12, day: 26)),
  )
}

////////////////
// Validation //
////////////////

#let _assert_entry(row, entry, condition, message) = {
  message = "row " + str(row) + " (day " + str(entry.day) + "): " + message
  assert(condition, message: message)
}

#let _check_entries(year, month, entries) = {
  for (row, e) in entries.enumerate(start: 1) {
    _assert_entry(row, e, e.start <= e.end, "start must be before end")
    _assert_entry(row, e, e.rest <= e.end - e.start, "rest too long")

    // I think the previous two checks should make it impossible for this assert
    // to fail, but just to be careful...
    _assert_entry(row, e, e.duration >= 0, "duration must be positive")

    // Date checks
    let date = datetime(year: year, month: month, day: e.day)
    _assert_entry(row, e, date.weekday() != 6, "day is a Saturday")
    _assert_entry(row, e, date.weekday() != 7, "day is a Sunday")
    for holiday in _public_holidays_germany_bw(year) {
      _assert_entry(row, e, date != holiday.date, "day is a holiday (" + holiday.name + ")")
    }

    // Time range checks
    // https://github.com/kit-sdq/TimeSheetGenerator/blob/2e80a56483832fb96087b8145c6cf311ec417c60/src/main/java/checker/MiLoGChecker.java#L30-L31
    let earliest = _parse_duration("06:00")
    let latest = _parse_duration("22:00")
    _assert_entry(row, e, e.start >= earliest, "must not work before 06:00")
    _assert_entry(row, e, e.end <= latest, "must not work after 22:00")
  }
}

#let _assert_day(day, condition, message) = {
  message = "day " + str(day) + ": " + message
  assert(condition, message: message)
}

#let _check_days(entries) = {
  let by_day = (:)
  for entry in entries {
    let key = str(entry.day)
    let info = by_day.at(key, default: (duration: 0, rest: 0))
    info.duration += entry.duration
    info.rest += entry.rest
    by_day.insert(key, info)
  }

  for (day, info) in by_day.pairs() {
    // According to the TimeSheetGenerator, working hours *must* not exceed 10
    // hours per day:
    // https://github.com/kit-sdq/TimeSheetGenerator/blob/2e80a56483832fb96087b8145c6cf311ec417c60/src/main/java/checker/MiLoGChecker.java#L32
    //
    // According to "Merkblatt für Studentische/Wissenschaftliche Hilfskräfte"
    // (Stand 2024-03-15), working hours *should* not exceed 8 hours per day,
    // though the wording suggests it is allowed in theory.
    //
    // §3 of the Arbeitszeitgesetz (ArbZG) reads: "Die werktägliche Arbeitszeit
    // der Arbeitnehmer darf acht Stunden nicht überschreiten. Sie kann auf bis
    // zu zehn Stunden nur verlängert werden, wenn innerhalb von sechs
    // Kalendermonaten oder innerhalb von 24 Wochen im Durchschnitt acht Stunden
    // werktäglich nicht überschritten werden."
    //
    // Conclusion: A hard limit of 8 working hours a day will likely cause the
    // least headaches in the long run.
    let max_duration = _parse_duration("08:00")
    _assert_day(day, info.duration <= max_duration, "must not work more than 8 hours per day (see comment in typst template for more details)")

    // The TimeSheetGenerator requires 30 minutes rest after more than 6 hours
    // of work, and 45 minutes rest after more than 9 hours of work:
    // https://github.com/kit-sdq/TimeSheetGenerator/blob/2e80a56483832fb96087b8145c6cf311ec417c60/src/main/java/checker/MiLoGChecker.java#L35
    //
    // §4 of the Arbeitszeitgesetz (ArbZG) reads: "Die Arbeit ist durch im
    // voraus feststehende Ruhepausen von mindestens 30 Minuten bei einer
    // Arbeitszeit von mehr als sechs bis zu neun Stunden und 45 Minuten bei
    // einer Arbeitszeit von mehr als neun Stunden insgesamt zu unterbrechen.
    // [...]"
    //
    // Since we already have a hard limit of 8 working hours a day, only the 30
    // minute case will ever happen. However, the 45 minute case is kept for
    // completeness. This should prevent correctness bugs if the working hour
    // limit is ever increased again.
    if info.duration > _parse_duration("09:00") {
      _assert_day(day,
        info.rest >= _parse_duration("00:45"),
        "at least 45 minutes rest required after more than 9 hours of work",
      )
    } else if info.duration > _parse_duration("06:00") {
      _assert_day(day,
        info.rest >= _parse_duration("00:30"),
        "30 minutes rest required after more than 6 hours of work",
      )
    }
  }
}

#let _check_total(total) = {
  let max_total = _parse_duration("85:00")
  assert(total <= max_total, message: "must not work more than 85 hours per month")
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
  sort: true,
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
  assert(entries.len() <= _kit_rows, message: "at most " + str(_kit_rows) + " entries allowed")

  if sort {
    entries = entries.sorted(key: entry => (entry.day, entry.end, entry.start))
  }

  let monthly = monthly_hours * 60
  let holiday = entries.filter(e => e.note == notes.Urlaub).map(e => e.duration).sum(default: 0)
  let total = entries.map(e => e.duration).sum(default: 0)
  let carry_next_month = carry_prev_month + total - monthly

  if validate {
    _check_entries(year, month, entries)
    _check_days(entries)
    _check_total(total)
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

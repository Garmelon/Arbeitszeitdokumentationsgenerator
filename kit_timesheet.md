# The `kit_timesheet` typst module

This file exhaustively documents the `kit_timesheet` typst module and its
exported values.

## Quickstart

```typst
#import "kit_timesheet.typ" as ts

#ts.timesheet(
  name: [McStudentface, Student],
  staff_id: 1337420,
  department: [Institut für Informatik],
  working_area: ts.areas.Großforschung,
  monthly_hours: 40,
  hourly_wage: [14.09],
  carry_prev_month: "02:30",
  year: 2024,
  month: 1,
  ts.entry("Urlaub", 2, "12:00", "14:00", note: ts.notes.Urlaub),
  ts.entry("Mate trinken", 3, "14:00", "15:30"),
  ts.entry("Im Bett liegen", 4, "10:00", "12:00"),
  ts.entry("Stundenzettel ausfüllen", 31, "14:00", "15:30", rest: "01:00"),
)
```

## Times and durations

Some function arguments represent a time of day or a duration. They expect a
string of the form `HH:MM` or `-HH:MM`, e.g. `12:34`. More specifically, the
string must match the regex `^-?[0-9]+:[0-5][0-9]$`.

This makes it possible to specify negative durations, which can be useful if you
didn't work enough hours last month and want to specify a negative
`carry_last_month` value.

A time of day is just a duration starting from midnight (00:00).

## Members

### `areas` (dictionary)

The `areas` dict contains the enum-like string values accepted by the
`working_area` parameter of the `timesheet` function. See its documentation for
more detail.

Available entries are:

- `areas.Großforschung` (value: `"GF"`)
- `areas.Unibereich` (value: `"UB"`)

### `notes` (dictionary)

The `notes` dict contains the enum-like string values accepted by the `note`
parameter of the `entry` function. See its documentation as well as footnote 1
in the generated document for more detail.

Available entries are:

- `notes.Urlaub` (value: `"U"`)
- `notes.Krankheit` (value: `"K"`)
- `notes.Feiertag` (value: `"F"`)
- `notes.Sonstiges` (value: `"S"`)

### `entry` (function)

Create a single entry for the `entries` parameter of the `timesheet` function.
An entry corresponds exactly to a row in the resulting document.

Positional arguments:

- `task`:
  What you worked on. Corresponds to the _Tätigkeit_ column of the table.
- `day`:
  The day of month. Corresponds to the _Datum_ column of the table.
- `start`:
  When you started working. Corresponds to the _Beginn_ column of the table.
- `end`:
  When you stopped working. Corresponds to the _Ende_ column of the table.

Named arguments:

- `rest` (default: `"00:00"`):
  Break time. Corresponds to the _Pause_ column of the table. If it wasn't a
  keyword, I'd have called it `break` :D
- `note` (default: `none`):
  Additional note for the _Arbeitszeit_ column (whose value is automatically
  calculated). Entries with a note of `notes.Urlaub` are used to calculate the
  _Urlaub anteilig_ field in the summary table. See the `notes` dictionary for
  all available values.

### `timesheet` (function)

Generate and validate a full timesheet.

Positional arguments:

- `..entries`:
  All positional arguments are entries for the big table, created using the
  `entry` function.

Named arguments:

- `name`:
  Your name (Name, Vorname).
- `staff_id`:
  Your staff id (Personalnummer).
- `department`:
  Your department (Organisationseinheit/OE).
- `working_area`:
  Your working area (Großforschung/GF or Unibereich/UB). Corresponds to the _GF_
  and _UB_ checkboxes on the form. See the `areas` dictionary for all available
  values.
- `monthly_hours`:
  How many hours per month your contract says you should work (Vertraglich
  vereinbarte Arbeitszeit).
- `hourly_wage`:
  Your hourly wage (Stundensatz).
- `validate` (default: `true`):
  Whether the template should try to validate the data you entered (check if
  values look wrong, if you worked on a holiday, ...). If you turn this off, you
  can do funky things like work a negative amount of time or on Sundays.
- `year`:
  The year this time sheet is being generated for.
- `month`:
  The month this time sheet is being generated for.

### `timesheet_empty` (function)

Generate an empty timesheet. Useful if you want to fill it out by hand. Not sure
why anyone would want to do this though :P

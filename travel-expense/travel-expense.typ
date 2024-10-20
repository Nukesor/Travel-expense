#import "@preview/oxifmt:0.2.1": strfmt
#set page(margin: (
  x: 2cm,
))
#set text(
  font: "Noto Sans",
  size: 10pt,
)

#let euro(cents) = {
  let euro = calc.round(cents / 100, digits: 2)
  [#strfmt("€{:.2}", euro)]
}

#let details = yaml("computed.yml")

= Reisekostenabrechnung
#v(1.5em)

Name: #details.author, Firma: #details.company \

Für den Monat *#details.month*

#set text(size: 8pt)
#table(
  columns: (3em, 5em, 8em, 1fr, 3em, 7em, 9em),
  align: horizon,
  stroke: 1pt + gray,
  table.header(
    table.cell([*Ifd. Nr.*]),
    [*Monats- Tag*],
    [*Reisebeginn/-ende*],
    [*Reiseanlass; \ Reiseweg (Ziel und Zweck der Reise)*],
    [*Std.*],
    [*Dienstlich gefahrene km*],
    [*Verpflegung- pauschalbetrag*],
  ),
  ..for (index, entry) in details.entries.enumerate() {
    (
      [#index],
      [#entry.day],
      [Start: #entry.start_time \
        Ende: #entry.end_time],
      entry.subject,
      [#entry.calculated.hours],
      [$#entry.traveled_km "km" * #euro(details.cent_per_km)$],
      [#euro(entry.calculated.catering_money)],
    )
  },
  table.cell(
    colspan: 5,
    "",
  ),
  [
    $#details.totals.travel_distance_km "km" * #euro(details.cent_per_km)$ \
    $= #euro(details.totals.travel_money)$
  ],
  [#euro(details.totals.catering_money)]
)
#set text(size: 10pt)

*Gesamtsumme der Reisekosten: #euro(details.totals.money)*
#v(1.5em)

Hamburg, den #details.document_date

#image(details.signature_image, width: 10em)

#details.author

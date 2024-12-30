# TODO

Style guide
look at [dark table]( <https://www.darktable.org/>)
Is the level of compactness ok?
How to I display portrait and landscape side by side.

## Known bugs

 Pannel is spelt panel

## Snags : -

  ImageGallery - "No Description" need padding ... left look tight.
  top and bottom need different padding.

  FileLister -- text element must be 100% of width -- (small by default)
  Button is on a the line below right aligned.

  Search bar has alignment issues, needs mag icon .. white slightly inset.

  No description should be greyed out. description of "" should be converted to no description.

  M,F buttons -- should become black on hover ... leaving the white to look like an inset.

  Define an ease in transition for metadata panel.

  Text in the about page looks weak, should at least mention my name.

## Functionality

  Demo "directory" button change the current selection.

  Demo active selection changing the indexer

  Pressing the full screen button starts a model dialog
  ( with a associated open close buttons )

## Meta data issues

## Style side tray

 Need to sanitize the input
 Consider using a FileDirectory selector.

## Performance

No I am using cantor pairing things seem slow..
network waterfall is still fast .. but client processing time is high.

Memoise the indexer .. as per the leptos framework, on startup the indexer will be called twice.

## Conditions for PWA

to be FULLY compliant add a PNG icon ( 512x512 ).
Add a maskable icon for android.

## Leptos 0.7

I want to port to leptos 0.7 as soon a possible.

One sticking point is that "leptos_routes" is not implemented for Actix. ( as of 29June alpha release )

[Release Notes](https://github.com/leptos-rs/leptos/releases/tag/0.7.0-alpha)

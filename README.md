# Photo Indexer

Rust 2021 Edition.

Provides a HTTP Server which provides searching of a images hold in local storage.

* A simple web interface allow search based on an images EXIF data.

* A root directory is specified and the indexer collects data from all images from the root downwards.

## Development

```bash
cd photo_indexer
npx tailwindcss -i ./input.css -o ./style/output.css --watch
```

and

```bash
cargo-leptos watch
```

## Production

TODO

* How to select the root images directory.
* Currently it is hard coded.

```bash
cd photo_indexer
cargo-leptos build
cargo-leptos serve
```

then,visit

[localhost](http://localhost:3000/)

Outline of test strategy

* Invalid root directory
* No matching file in root, with a text file ignored.
* Invalid exif data.

# Photo Indexer

Rust 2021 Edition.

Provides a HTTP Server which provides searching of a images hold in local storage.

* A simple web interface allow search based on an images EXIF data.

* An optional root directory is specified and the indexer collects data from all images from the root downwards. If no root directory is specified the current working directory will be used.



## Development

```bash
cd photo_indexer
./tailwind.sh
```

and

Here for example ../exif-samples is the root directory.

```bash
cargo-leptos watch -- ../exif-samples
```

## Production

TODO

```bash
cd photo_indexer
cargo-leptos serve --release -- ../exif-samples
```

then, visit

[localhost](http://localhost:3000/)

Outline of test strategy

* Invalid root directory
* No matching file in root, with a text file ignored.
* Invalid exif data.

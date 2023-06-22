# Photo Indexer

Rust 2021 Edition.

Provides a HTTP Server which provides searching of a images hold in local storage.

* A simple web interface allow search based on an images EXIF data.

* A root directory is specified and the indexer collects data from all images from the root downwards.

## Quick Start

```console
$ cargo run ./folder/ 8080
$ brave http://localhost:8080/
```

brave is a web-browser, other browsers are availble!

Outline of test stratergy

* Invalid root directory
* No matching file in root, with a text file ignored.
* Invalid exif data.


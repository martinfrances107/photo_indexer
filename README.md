# Photo Indexer

Rust 2021 Edition.

Provides a HTTP Server which provides searching of a images held in local storage.

* A simple web interface allow search based on an images EXIF data.

* The search is based on [tf–idf](https://en.wikipedia.org/wiki/Tf%E2%80%93idf) indexer run a selected directory.

![Alt SearchPage](https://raw.githubusercontent.com/martinfrances107/photo_indexer/main/images/SearchPage.png "Search Page")

## Development

The have been testing against a set of images
with a varied set of exif meta data.

Download here.
<https://github.com/ianare/exif-samples>

Watch the code base and update the tailwindcss file.

```bash
cd photo_indexer
./tailwind.sh
```

Finally run the development web server, and specify the /exif-sample image gallery as the root directory.

```bash
cargo-leptos watch -- ../exif-samples
```

 If no root directory is specified the current working directory will be used.

## Production

```bash
cd photo_indexer
cargo-leptos serve --release -- ../exif-samples
```

then, visit

[localhost:3000](http://localhost:3000/)

## Outline of test strategy

* Invalid root directory
* No matching file in root, with a text file ignored.
* Invalid exif data.

## Known issues

../exif-samples -- search for sony.

* One of the descriptions looks like -- "", "", "" --
* metadata look like 2 string are captured -- "DSC-D700", "" --

## Credits

The tf-idf indexer, is based on a original idea by [tsoding](https://github.com/tsoding/seroost)

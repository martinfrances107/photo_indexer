# Photo Indexer

Rust 2021 Edition.

Provides a HTTP Server which provides searching of a images held in local storage.

* A simple web interface allow search based on an images EXIF data.

* The search is based on [tf–idf](https://en.wikipedia.org/wiki/Tf%E2%80%93idf) indexer run a selected directory.

<image title="Search Page" alt="SearchPage" src="https://raw.githubusercontent.com/martinfrances107/photo_indexer/main/images/SearchPage.png">

## Development

The have been testing against a set of images
with a varied set of exif meta data.

Download here.
https://github.com/ianare/exif-samples

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

## Migration to leptos 0.7

This modules used leptos 0.6. leptos 0.7-alpha is availble but the following features are yet to be implemented/ported

* ACTIX support
* create_local_reource.

I will be migrating to 0.7 as soon as possible.

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

## Credits

The tf-idf indexer, is based on a original idea by [tsoding](https://github.com/tsoding/seroost)

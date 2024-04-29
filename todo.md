# TODO

## Known bugs

1) Images are sticky
  steps to reproduce ...

    type hdr - get 6 images
    type chinook - get one image -- but the first from hdr is incorrectly displayed.
    type "" to clear the images
    type chinook - to display the correct image.

## Snags : -

  Image card need to be rounded at the bottom.

  Search Button - Not vissible

## meta data issues

## style side tray

## Performance

  CSS is not compressed. A large drag on the waterfall.
  Add compression to HTTP server.

  ```rustlang
  .wrap(middleware::Compress::default())
  ```

## Conditions for PWA

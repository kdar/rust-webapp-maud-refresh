rust-webapp-maud-refresh
========================

Example of using [maud](https://github.com/lfairy/maud) compile-time templates in rust, and being able to reload the templates when they change via a shared library.

You could use a livereload proxy to auto refresh the browser (this example doesn't refresh the browser, it will just update the cached template when the template changes such that next time you visit the webpage, the new template will display).

## Motivation

This stuff is normally easy when using templates that reside on the filesystem, but I haven't seen anything like this when using compile-time templates. Not saying this is a good idea, but it's here if anyone is interested.

You could also just have a livereload proxy in front of a rust webserver, and just have your main webserver recompile on source change. However, I wanted to use a shared library for quicker compiles.

## Notes

Only tested on rustc 1.14.0-nightly and Windows 7 x64 but should work on nix if you change the shared library path.
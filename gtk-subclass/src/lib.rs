
extern crate gdk;
extern crate gio;
extern crate gtk;
#[macro_use]
extern crate glib;

extern crate cairo;
extern crate cairo_sys as cairo_ffi;
extern crate gdk_sys as gdk_ffi;
extern crate glib_sys as glib_ffi;
extern crate gobject_sys as gobject_ffi;
extern crate gtk_sys as gtk_ffi;
extern crate gio_sys as gio_ffi;
extern crate pango;

extern crate libc;

#[macro_use]
extern crate gobject_subclass;

#[macro_use]
extern crate gio_subclass;

mod auto;
pub use auto::application;
pub mod cell_renderer;

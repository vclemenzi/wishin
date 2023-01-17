#![deny(clippy::all)]

#[macro_use]
extern crate napi_derive;

#[napi(constructor)]
pub struct Application {}
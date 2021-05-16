# Changelog

All notable changes to this project will be documented in this file.

The format is roughly based on [Keep a
Changelog](http://keepachangelog.com/en/1.0.0/).

This project intends to inhere to [Semantic
Versioning](http://semver.org/spec/v2.0.0.html), but has not yet reached 1.0 so
all APIs might be changed.

## Unreleased - yyyy-mm-dd

### Bug Fixes

- Fixes some inefficiencies when decoding (particularly in the `FieldDecoder`).

## v0.5.0 - 2020-07-06

### New Features

- `integer` & `unsigned_integer` types now use TryInto instead of Into,
  allowing them to be used with types smaller than i64 & u64.  Attempting to
  decode an integer larger than a type can handle will result in a decode
  error.

## v0.4.1 - 2020-06-12

### Bug Fixes

- `DecodeError` now implements `std::error::Error`

## v0.4.0 - 2020-05-24

### New Features

- Added `and_then` decoder function

## v0.3.0 - 2020-05-24

### New Features

- Added map functions up to map50.

## v0.2.0 - 2020-02-09

### New Features

- Added Error::Other so other crates can provide cutom error messages

## v0.1.0 - 2020-02-03

- Initial release with support for most obvious types

//  SPDX-FileCopyrightText: Copyright 2022-2023 James M. Putnam (putnamjm.design@gmail.com)
//  SPDX-License-Identifier: MIT

//! core module
pub mod classes; // needs to be public for API
mod coerce;
mod compile;
pub mod exception; // needs to be public for API
pub mod frame; // needs to be public for mu native functions
mod functions;
mod image;
pub mod mu; // core API interfaces
pub mod namespaces; // needs to be public for function printing
pub mod read;
mod readtable; // needs to be public for type readers

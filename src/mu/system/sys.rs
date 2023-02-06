//  SPDX-FileCopyrightText: Copyright 2022-2023 James M. Putnam (putnamjm.design@gmail.com)
//  SPDX-License-Identifier: MIT

//! system interface
use crate::system::{stream, stream::Stream};

pub struct System {
    pub streams: Stream,
}

impl Default for System {
    fn default() -> Self {
        Self::new()
    }
}

impl System {
    pub fn new() -> Self {
        System {
            streams: stream::Stream::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::system::sys::System;

    #[test]
    fn system() {
        match System::new() {
            _ => assert_eq!(true, true),
        }
    }
}

//======---------------------------------------------------------------======//
//                                                                           //
// Copyright 2022-2023 Evan Cox <evanacox00@gmail.com>. All rights reserved. //
//                                                                           //
// Use of this source code is governed by a BSD-style license that can be    //
// found in the LICENSE.txt file at the root of this project, or at the      //
// following link: https://opensource.org/licenses/BSD-3-Clause              //
//                                                                           //
//======---------------------------------------------------------------======//

use crate::drivers::kserial;
use core::fmt;
use core::ops::DerefMut;
use ksupport::sync::BasicMutex;
use log::{Level, LevelFilter, Log, Metadata, Record};

/// A logger that outputs exclusively to the [`kserial`] serial backend.
///
/// This is the initial logger before the kernel has been able to set anything
/// up, it's meant to catch *anything* after the bootloader passes control for
/// the most part.
///
/// Flushing does nothing, as this logger is not buffered.
#[repr(transparent)]
#[derive(Copy, Clone, Debug)]
pub struct KSerialLogger;

static SERIAL_LOGGER: KSerialLogger = KSerialLogger;

impl Log for KSerialLogger {
    fn enabled(&self, _: &Metadata) -> bool {
        true
    }

    fn log(&self, record: &Record) {
        let port = kserial::serial();

        {
            let mut serial = port.lock();
            let level = match record.level() {
                Level::Error => "[error!]",
                Level::Warn => "[ warn ]",
                Level::Info => "[ info ]",
                Level::Debug => "[debug!]",
                Level::Trace => "[trace!]",
            };

            let _ = fmt::write(
                serial.deref_mut(),
                format_args!(
                    "{level} [{} at {}:{}]: {} \n",
                    record.target(),
                    record.file().unwrap_or("<unknown>"),
                    record.line().unwrap_or(0),
                    record.args(),
                ),
            );
        }
    }

    fn flush(&self) {}
}

/// Initialize the kernel-level serial logger.
pub fn logger_init(max: LevelFilter) {
    log::set_max_level(max);

    let _ = log::set_logger(&SERIAL_LOGGER);
}

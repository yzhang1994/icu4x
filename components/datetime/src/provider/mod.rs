// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

// Provider structs must be stable
#![allow(clippy::exhaustive_structs)]

//! Data provider struct definitions for this ICU4X component.
//!
//! Read more about data providers: [`icu_provider`]

/// Data providers for the Gregorian Calendar.
pub mod calendar;

/// Data providers for time zones.
pub mod time_zones;

/// Provider for week data.
pub mod week_data;

/// Traits for managing data needed by [`DateTimeFormat`](crate::DateTimeFormat).
pub(crate) mod date_time;

// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::provider::time_zones::{MetaZoneId, TimeZoneBcp47Id};
use tinystr::TinyStr8;

use crate::date::*;
use core::str::FromStr;

/// A temporary struct that implements [`TimeZoneInput`]
/// and is used in tests, benchmarks and examples of this component.
///
/// *Notice:* Rust at the moment does not have a canonical way to represent time zones. We are introducing
/// [`MockTimeZone`](crate::mock::time_zone::MockTimeZone) as an example of the data necessary for
/// ICU [`TimeZoneFormat`](crate::TimeZoneFormat) to work, and [we hope to work with the community](
/// https://github.com/unicode-org/icu4x/blob/main/docs/research/datetime.md) to develop core date and time
/// APIs that will work as an input for this component.
///
/// # Examples
///
/// ```
/// use icu::datetime::date::GmtOffset;
/// use icu::datetime::mock::time_zone::MockTimeZone;
///
/// let tz1 = MockTimeZone::new(
///     GmtOffset::default(),
///     /* time_zone_id */ None,
///     /* metazone_id */ None,
///     /* time_variaint */ None,
/// );
///
/// let tz2: MockTimeZone = "+05:00".parse().expect("Failed to parse a time zone.");
/// ```
#[derive(Debug, Default)]
#[allow(clippy::exhaustive_structs)] // this type will not add fields (it is largely an example type)
pub struct MockTimeZone {
    /// The GMT offset in seconds.
    pub gmt_offset: GmtOffset,
    /// The IANA time-zone identifier
    pub time_zone_id: Option<TimeZoneBcp47Id>,
    /// The CLDR metazone identifier
    pub metazone_id: Option<MetaZoneId>,
    /// The time variant e.g. "daylight" or "standard"
    pub time_variant: Option<TinyStr8>,
}

impl MockTimeZone {
    /// Creates a new [`MockTimeZone`].
    /// A GMT offset is required, as it is used as a final fallback for formatting.
    /// The other arguments optionally allow access to more robust formats.
    pub const fn new(
        gmt_offset: GmtOffset,
        time_zone_id: Option<TimeZoneBcp47Id>,
        metazone_id: Option<MetaZoneId>,
        time_variant: Option<TinyStr8>,
    ) -> Self {
        Self {
            gmt_offset,
            time_zone_id,
            metazone_id,
            time_variant,
        }
    }
}

impl FromStr for MockTimeZone {
    type Err = DateTimeError;

    /// Parse a [`MockTimeZone`] from a string.
    ///
    /// This utility is for easily creating time zones, not a complete robust solution.
    ///
    /// The offset must range from GMT-12 to GMT+14.
    /// The string must be an ISO-8601 time zone designator:
    /// e.g. Z
    /// e.g. +05
    /// e.g. +0500
    /// e.g. +05:00
    ///
    /// # Examples
    ///
    /// ```
    /// use icu::datetime::mock::time_zone::MockTimeZone;
    ///
    /// let tz0: MockTimeZone = "Z".parse().expect("Failed to parse a time zone.");
    /// let tz1: MockTimeZone = "+02".parse().expect("Failed to parse a time zone.");
    /// let tz2: MockTimeZone = "-0230".parse().expect("Failed to parse a time zone.");
    /// let tz3: MockTimeZone = "+02:30".parse().expect("Failed to parse a time zone.");
    /// ```
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let gmt_offset = GmtOffset::from_str(input)?;
        Ok(Self {
            gmt_offset,
            time_zone_id: None,
            metazone_id: None,
            time_variant: None,
        })
    }
}

impl TimeZoneInput for MockTimeZone {
    fn gmt_offset(&self) -> GmtOffset {
        self.gmt_offset
    }

    fn time_zone_id(&self) -> Option<&TimeZoneBcp47Id> {
        self.time_zone_id.as_ref()
    }

    fn metazone_id(&self) -> Option<&MetaZoneId> {
        self.metazone_id.as_ref()
    }

    fn time_variant(&self) -> Option<&TinyStr8> {
        self.time_variant.as_ref()
    }
}

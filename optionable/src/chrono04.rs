use crate::optionable::impl_optional_self;
use crate::{Error, Optionable, OptionableConvert};
use chrono04::{DateTime, Days, Months, NaiveDate, NaiveDateTime, NaiveTime, TimeDelta, TimeZone};

impl<Tz: TimeZone> Optionable for DateTime<Tz> {
    type Optioned = Self;
}

impl<Tz: TimeZone> OptionableConvert for DateTime<Tz> {
    fn into_optioned(self) -> Self::Optioned {
        self
    }

    fn try_from_optioned(value: Self::Optioned) -> Result<Self, Error> {
        Ok(value)
    }

    fn merge(&mut self, other: Self::Optioned) -> Result<(), Error> {
        *self = other;
        Ok(())
    }
}

impl_optional_self!(Days, Months, NaiveDate, NaiveDateTime, NaiveTime, TimeDelta);

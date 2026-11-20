use crate::optionable::impl_optional_self;
use jiff::{SignedDuration, Span, SpanFieldwise, Timestamp, ZonedDateTime};

impl_optional_self!(
    SignedDuration,
    Span,
    SpanFieldwise,
    Timestamp,
    ZonedDateTime
);

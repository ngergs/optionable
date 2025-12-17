use crate::optionable::impl_optional_self;
use jiff::{SignedDuration, Span, SpanFieldwise, SpanRelativeTo, Timestamp, Zoned};

impl_optional_self!(SignedDuration, Span, SpanFieldwise, Timestamp, Zoned);

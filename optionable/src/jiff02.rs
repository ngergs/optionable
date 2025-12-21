use crate::optionable::impl_optional_self;
use jiff02::{SignedDuration, Span, SpanFieldwise, Timestamp, Zoned};

impl_optional_self!(SignedDuration, Span, SpanFieldwise, Timestamp, Zoned);

use crate::optionable::impl_optional_self;
use serde_json::Value;


#[cfg_attr(docsrs, doc(cfg(feature = "serde_json")))]
impl_optional_self!(Value);

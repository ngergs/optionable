#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
/// ShardInfo describes the shard selector that was applied to produce a list response. Its presence on a list response indicates the list is a filtered subset.
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ShardInfoAc {
    /// selector is the shard selector string from the request, echoed back so clients can verify which shard they received and merge responses from multiple shards.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selector: Option<std::string::String>,
}
#[automatically_derived]
impl crate::Optionable for k8s_openapi028::apimachinery::pkg::apis::meta::v1::ShardInfo {
    type Optioned = ShardInfoAc;
}
#[automatically_derived]
impl crate::Optionable for ShardInfoAc {
    type Optioned = ShardInfoAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert
for k8s_openapi028::apimachinery::pkg::apis::meta::v1::ShardInfo {
    fn into_optioned(self) -> ShardInfoAc {
        ShardInfoAc {
            selector: Some(self.selector),
        }
    }
    fn try_from_optioned(value: ShardInfoAc) -> Result<Self, crate::Error> {
        Ok(Self {
            selector: value
                .selector
                .ok_or(crate::Error {
                    missing_field: "selector",
                })?,
        })
    }
    fn merge(&mut self, other: ShardInfoAc) -> Result<(), crate::Error> {
        if let Some(other_value) = other.selector {
            self.selector = crate::OptionableConvert::try_from_optioned(other_value)?;
        }
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<k8s_openapi028::apimachinery::pkg::apis::meta::v1::ShardInfo>
for ShardInfoAc {
    fn from_optionable(
        value: k8s_openapi028::apimachinery::pkg::apis::meta::v1::ShardInfo,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<
        k8s_openapi028::apimachinery::pkg::apis::meta::v1::ShardInfo,
        crate::Error,
    > {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi028::apimachinery::pkg::apis::meta::v1::ShardInfo,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
impl k8s_openapi028::DeepMerge for ShardInfoAc {
    fn merge_from(&mut self, other: Self) {
        k8s_openapi028::DeepMerge::merge_from(&mut self.selector, other.selector);
    }
}

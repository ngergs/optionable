#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ListMetaAc {
    #[serde(rename = "continue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub continue_: Option<std::string::String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remaining_item_count: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_version: Option<std::string::String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub self_link: Option<std::string::String>,
}
#[automatically_derived]
impl crate::Optionable for k8s_openapi027::apimachinery::pkg::apis::meta::v1::ListMeta {
    type Optioned = ListMetaAc;
}
#[automatically_derived]
impl crate::Optionable for ListMetaAc {
    type Optioned = ListMetaAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert
for k8s_openapi027::apimachinery::pkg::apis::meta::v1::ListMeta {
    fn into_optioned(self) -> ListMetaAc {
        ListMetaAc {
            continue_: self.continue_,
            remaining_item_count: self.remaining_item_count,
            resource_version: self.resource_version,
            self_link: self.self_link,
        }
    }
    fn try_from_optioned(value: ListMetaAc) -> Result<Self, crate::Error> {
        Ok(Self {
            continue_: value.continue_,
            remaining_item_count: value.remaining_item_count,
            resource_version: value.resource_version,
            self_link: value.self_link,
        })
    }
    fn merge(&mut self, other: ListMetaAc) -> Result<(), crate::Error> {
        self.continue_ = other.continue_;
        self.remaining_item_count = other.remaining_item_count;
        self.resource_version = other.resource_version;
        self.self_link = other.self_link;
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<k8s_openapi027::apimachinery::pkg::apis::meta::v1::ListMeta>
for ListMetaAc {
    fn from_optionable(
        value: k8s_openapi027::apimachinery::pkg::apis::meta::v1::ListMeta,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<
        k8s_openapi027::apimachinery::pkg::apis::meta::v1::ListMeta,
        crate::Error,
    > {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::apimachinery::pkg::apis::meta::v1::ListMeta,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}

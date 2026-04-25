#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
/// ListMeta describes metadata that synthetic resources must have, including lists and various status objects. A resource may have only one of {ObjectMeta, ListMeta}.
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ListMetaAc {
    /// continue may be set if the user set a limit on the number of items returned, and indicates that the server has more data available. The value is opaque and may be used to issue another request to the endpoint that served this list to retrieve the next set of available objects. Continuing a consistent list may not be possible if the server configuration has changed or more than a few minutes have passed. The resourceVersion field returned when using this continue value will be identical to the value in the first response, unless you have received this token from an error message.
    #[serde(rename = "continue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub continue_: Option<std::string::String>,
    /// remainingItemCount is the number of subsequent items in the list which are not included in this list response. If the list request contained label or field selectors, then the number of remaining items is unknown and the field will be left unset and omitted during serialization. If the list is complete (either because it is not chunking or because this is the last chunk), then there are no more remaining items and this field will be left unset and omitted during serialization. Servers older than v1.15 do not set this field. The intended use of the remainingItemCount is *estimating* the size of a collection. Clients should not rely on the remainingItemCount to be set or to be exact.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remaining_item_count: Option<i64>,
    /// String that identifies the server's internal version of this object that can be used by clients to determine when objects have changed. Value must be treated as opaque by clients and passed unmodified back to the server. Populated by the system. Read-only. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#concurrency-control-and-consistency
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_version: Option<std::string::String>,
    /// Deprecated: selfLink is a legacy read-only field that is no longer populated by the system.
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
        if self.continue_.is_none() {
            self.continue_ = crate::OptionableConvert::try_from_optioned(
                other.continue_,
            )?;
        } else if let Some(self_value) = self.continue_.as_mut()
            && let Some(other_value) = other.continue_
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        if self.remaining_item_count.is_none() {
            self.remaining_item_count = crate::OptionableConvert::try_from_optioned(
                other.remaining_item_count,
            )?;
        } else if let Some(self_value) = self.remaining_item_count.as_mut()
            && let Some(other_value) = other.remaining_item_count
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        if self.resource_version.is_none() {
            self.resource_version = crate::OptionableConvert::try_from_optioned(
                other.resource_version,
            )?;
        } else if let Some(self_value) = self.resource_version.as_mut()
            && let Some(other_value) = other.resource_version
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        if self.self_link.is_none() {
            self.self_link = crate::OptionableConvert::try_from_optioned(
                other.self_link,
            )?;
        } else if let Some(self_value) = self.self_link.as_mut()
            && let Some(other_value) = other.self_link
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
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
impl k8s_openapi027::DeepMerge for ListMetaAc {
    fn merge_from(&mut self, other: Self) {
        k8s_openapi027::DeepMerge::merge_from(&mut self.continue_, other.continue_);
        k8s_openapi027::DeepMerge::merge_from(
            &mut self.remaining_item_count,
            other.remaining_item_count,
        );
        k8s_openapi027::DeepMerge::merge_from(
            &mut self.resource_version,
            other.resource_version,
        );
        k8s_openapi027::DeepMerge::merge_from(&mut self.self_link, other.self_link);
    }
}

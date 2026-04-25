#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
/// GroupResource specifies a Group and a Resource, but does not force a version.  This is useful for identifying concepts during lookup stages without having partially valid types
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct GroupResourceAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group: Option<std::string::String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource: Option<std::string::String>,
}
#[automatically_derived]
impl crate::Optionable
for k8s_openapi027::apimachinery::pkg::apis::meta::v1::GroupResource {
    type Optioned = GroupResourceAc;
}
#[automatically_derived]
impl crate::Optionable for GroupResourceAc {
    type Optioned = GroupResourceAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert
for k8s_openapi027::apimachinery::pkg::apis::meta::v1::GroupResource {
    fn into_optioned(self) -> GroupResourceAc {
        GroupResourceAc {
            group: Some(self.group),
            resource: Some(self.resource),
        }
    }
    fn try_from_optioned(value: GroupResourceAc) -> Result<Self, crate::Error> {
        Ok(Self {
            group: value
                .group
                .ok_or(crate::Error {
                    missing_field: "group",
                })?,
            resource: value
                .resource
                .ok_or(crate::Error {
                    missing_field: "resource",
                })?,
        })
    }
    fn merge(&mut self, other: GroupResourceAc) -> Result<(), crate::Error> {
        if let Some(other_value) = other.group {
            self.group = crate::OptionableConvert::try_from_optioned(other_value)?;
        }
        if let Some(other_value) = other.resource {
            self.resource = crate::OptionableConvert::try_from_optioned(other_value)?;
        }
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<
    k8s_openapi027::apimachinery::pkg::apis::meta::v1::GroupResource,
> for GroupResourceAc {
    fn from_optionable(
        value: k8s_openapi027::apimachinery::pkg::apis::meta::v1::GroupResource,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<
        k8s_openapi027::apimachinery::pkg::apis::meta::v1::GroupResource,
        crate::Error,
    > {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::apimachinery::pkg::apis::meta::v1::GroupResource,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
impl k8s_openapi027::DeepMerge for GroupResourceAc {
    fn merge_from(&mut self, other: Self) {
        k8s_openapi027::DeepMerge::merge_from(&mut self.group, other.group);
        k8s_openapi027::DeepMerge::merge_from(&mut self.resource, other.resource);
    }
}

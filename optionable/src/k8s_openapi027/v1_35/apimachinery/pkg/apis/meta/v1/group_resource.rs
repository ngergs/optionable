#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct GroupResourceAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group: Option<<std::string::String as crate::Optionable>::Optioned>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource: Option<<std::string::String as crate::Optionable>::Optioned>,
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
            group: Some(crate::OptionableConvert::into_optioned(self.group)),
            resource: Some(crate::OptionableConvert::into_optioned(self.resource)),
        }
    }
    fn try_from_optioned(value: GroupResourceAc) -> Result<Self, crate::Error> {
        Ok(Self {
            group: crate::OptionableConvert::try_from_optioned(
                value
                    .group
                    .ok_or(crate::Error {
                        missing_field: "group",
                    })?,
            )?,
            resource: crate::OptionableConvert::try_from_optioned(
                value
                    .resource
                    .ok_or(crate::Error {
                        missing_field: "resource",
                    })?,
            )?,
        })
    }
    fn merge(&mut self, other: GroupResourceAc) -> Result<(), crate::Error> {
        if let Some(other_value) = other.group {
            crate::OptionableConvert::merge(&mut self.group, other_value)?;
        }
        if let Some(other_value) = other.resource {
            crate::OptionableConvert::merge(&mut self.resource, other_value)?;
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

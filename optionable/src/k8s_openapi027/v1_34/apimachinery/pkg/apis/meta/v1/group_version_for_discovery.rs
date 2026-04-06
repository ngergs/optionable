#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
/// GroupVersion contains the "group/version" and "version" string of a version. It is made a struct to keep extensibility.
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct GroupVersionForDiscoveryAc {
    /// groupVersion specifies the API group and version in the form "group/version"
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_version: Option<std::string::String>,
    /// version specifies the version in the form of "version". This is to save the clients the trouble of splitting the GroupVersion.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<std::string::String>,
}
#[automatically_derived]
impl crate::Optionable
for k8s_openapi027::apimachinery::pkg::apis::meta::v1::GroupVersionForDiscovery {
    type Optioned = GroupVersionForDiscoveryAc;
}
#[automatically_derived]
impl crate::Optionable for GroupVersionForDiscoveryAc {
    type Optioned = GroupVersionForDiscoveryAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert
for k8s_openapi027::apimachinery::pkg::apis::meta::v1::GroupVersionForDiscovery {
    fn into_optioned(self) -> GroupVersionForDiscoveryAc {
        GroupVersionForDiscoveryAc {
            group_version: Some(self.group_version),
            version: Some(self.version),
        }
    }
    fn try_from_optioned(
        value: GroupVersionForDiscoveryAc,
    ) -> Result<Self, crate::Error> {
        Ok(Self {
            group_version: value
                .group_version
                .ok_or(crate::Error {
                    missing_field: "group_version",
                })?,
            version: value
                .version
                .ok_or(crate::Error {
                    missing_field: "version",
                })?,
        })
    }
    fn merge(&mut self, other: GroupVersionForDiscoveryAc) -> Result<(), crate::Error> {
        if let Some(other_value) = other.group_version {
            self.group_version = other_value;
        }
        if let Some(other_value) = other.version {
            self.version = other_value;
        }
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<
    k8s_openapi027::apimachinery::pkg::apis::meta::v1::GroupVersionForDiscovery,
> for GroupVersionForDiscoveryAc {
    fn from_optionable(
        value: k8s_openapi027::apimachinery::pkg::apis::meta::v1::GroupVersionForDiscovery,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<
        k8s_openapi027::apimachinery::pkg::apis::meta::v1::GroupVersionForDiscovery,
        crate::Error,
    > {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::apimachinery::pkg::apis::meta::v1::GroupVersionForDiscovery,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}

pub struct GroupVersionForDiscoveryOpt {
    pub group_version: Option<<std::string::String as crate::Optionable>::Optioned>,
    pub version: Option<<std::string::String as crate::Optionable>::Optioned>,
}
#[automatically_derived]
impl crate::Optionable
for ::k8s_openapi::apimachinery::pkg::apis::meta::v1::GroupVersionForDiscovery {
    type Optioned = GroupVersionForDiscoveryOpt;
}
#[automatically_derived]
impl crate::Optionable for GroupVersionForDiscoveryOpt {
    type Optioned = GroupVersionForDiscoveryOpt;
}
#[automatically_derived]
impl crate::OptionableConvert
for ::k8s_openapi::apimachinery::pkg::apis::meta::v1::GroupVersionForDiscovery {
    fn into_optioned(self) -> GroupVersionForDiscoveryOpt {
        GroupVersionForDiscoveryOpt {
            group_version: Some(
                crate::OptionableConvert::into_optioned(self.group_version),
            ),
            version: Some(crate::OptionableConvert::into_optioned(self.version)),
        }
    }
    fn try_from_optioned(
        value: GroupVersionForDiscoveryOpt,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            group_version: crate::OptionableConvert::try_from_optioned(
                value
                    .group_version
                    .ok_or(crate::optionable::Error {
                        missing_field: "group_version",
                    })?,
            )?,
            version: crate::OptionableConvert::try_from_optioned(
                value
                    .version
                    .ok_or(crate::optionable::Error {
                        missing_field: "version",
                    })?,
            )?,
        })
    }
    fn merge(
        &mut self,
        other: GroupVersionForDiscoveryOpt,
    ) -> Result<(), crate::optionable::Error> {
        if let Some(other_value) = other.group_version {
            crate::OptionableConvert::merge(&mut self.group_version, other_value)?;
        }
        if let Some(other_value) = other.version {
            crate::OptionableConvert::merge(&mut self.version, other_value)?;
        }
        Ok(())
    }
}

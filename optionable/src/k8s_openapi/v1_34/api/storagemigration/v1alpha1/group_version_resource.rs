pub struct GroupVersionResourceOpt {
    pub group: <Option<std::string::String> as crate::Optionable>::Optioned,
    pub resource: <Option<std::string::String> as crate::Optionable>::Optioned,
    pub version: <Option<std::string::String> as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable
for ::k8s_openapi::api::storagemigration::v1alpha1::group_version_resource::GroupVersionResource {
    type Optioned = GroupVersionResourceOpt;
}
#[automatically_derived]
impl crate::Optionable for GroupVersionResourceOpt {
    type Optioned = GroupVersionResourceOpt;
}
#[automatically_derived]
impl crate::OptionableConvert
for ::k8s_openapi::api::storagemigration::v1alpha1::group_version_resource::GroupVersionResource {
    fn into_optioned(self) -> GroupVersionResourceOpt {
        GroupVersionResourceOpt {
            group: <Option<
                std::string::String,
            > as crate::OptionableConvert>::into_optioned(self.group),
            resource: <Option<
                std::string::String,
            > as crate::OptionableConvert>::into_optioned(self.resource),
            version: <Option<
                std::string::String,
            > as crate::OptionableConvert>::into_optioned(self.version),
        }
    }
    fn try_from_optioned(
        value: GroupVersionResourceOpt,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            group: <Option<
                std::string::String,
            > as crate::OptionableConvert>::try_from_optioned(value.group)?,
            resource: <Option<
                std::string::String,
            > as crate::OptionableConvert>::try_from_optioned(value.resource)?,
            version: <Option<
                std::string::String,
            > as crate::OptionableConvert>::try_from_optioned(value.version)?,
        })
    }
    fn merge(
        &mut self,
        other: GroupVersionResourceOpt,
    ) -> Result<(), crate::optionable::Error> {
        <Option<
            std::string::String,
        > as crate::OptionableConvert>::merge(&mut self.group, other.group)?;
        <Option<
            std::string::String,
        > as crate::OptionableConvert>::merge(&mut self.resource, other.resource)?;
        <Option<
            std::string::String,
        > as crate::OptionableConvert>::merge(&mut self.version, other.version)?;
        Ok(())
    }
}

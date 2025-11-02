#[derive(Clone, std::fmt::Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct GroupVersionResourceAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group: <Option<std::string::String> as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource: <Option<std::string::String> as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: <Option<std::string::String> as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable
for ::k8s_openapi::api::storagemigration::v1alpha1::GroupVersionResource {
    type Optioned = GroupVersionResourceAc;
}
#[automatically_derived]
impl crate::Optionable for GroupVersionResourceAc {
    type Optioned = GroupVersionResourceAc;
}
#[automatically_derived]
impl crate::OptionableConvert
for ::k8s_openapi::api::storagemigration::v1alpha1::GroupVersionResource {
    fn into_optioned(self) -> GroupVersionResourceAc {
        GroupVersionResourceAc {
            group: crate::OptionableConvert::into_optioned(self.group),
            resource: crate::OptionableConvert::into_optioned(self.resource),
            version: crate::OptionableConvert::into_optioned(self.version),
        }
    }
    fn try_from_optioned(
        value: GroupVersionResourceAc,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            group: crate::OptionableConvert::try_from_optioned(value.group)?,
            resource: crate::OptionableConvert::try_from_optioned(value.resource)?,
            version: crate::OptionableConvert::try_from_optioned(value.version)?,
        })
    }
    fn merge(
        &mut self,
        other: GroupVersionResourceAc,
    ) -> Result<(), crate::optionable::Error> {
        crate::OptionableConvert::merge(&mut self.group, other.group)?;
        crate::OptionableConvert::merge(&mut self.resource, other.resource)?;
        crate::OptionableConvert::merge(&mut self.version, other.version)?;
        Ok(())
    }
}

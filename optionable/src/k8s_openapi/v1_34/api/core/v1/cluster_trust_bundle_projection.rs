pub struct ClusterTrustBundleProjectionAc {
    pub label_selector: <Option<
        ::k8s_openapi::apimachinery::pkg::apis::meta::v1::LabelSelector,
    > as crate::Optionable>::Optioned,
    pub name: <Option<std::string::String> as crate::Optionable>::Optioned,
    pub optional: <Option<bool> as crate::Optionable>::Optioned,
    pub path: Option<<std::string::String as crate::Optionable>::Optioned>,
    pub signer_name: <Option<std::string::String> as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::core::v1::ClusterTrustBundleProjection {
    type Optioned = ClusterTrustBundleProjectionAc;
}
#[automatically_derived]
impl crate::Optionable for ClusterTrustBundleProjectionAc {
    type Optioned = ClusterTrustBundleProjectionAc;
}
#[automatically_derived]
impl crate::OptionableConvert
for ::k8s_openapi::api::core::v1::ClusterTrustBundleProjection {
    fn into_optioned(self) -> ClusterTrustBundleProjectionAc {
        ClusterTrustBundleProjectionAc {
            label_selector: crate::OptionableConvert::into_optioned(self.label_selector),
            name: crate::OptionableConvert::into_optioned(self.name),
            optional: crate::OptionableConvert::into_optioned(self.optional),
            path: Some(crate::OptionableConvert::into_optioned(self.path)),
            signer_name: crate::OptionableConvert::into_optioned(self.signer_name),
        }
    }
    fn try_from_optioned(
        value: ClusterTrustBundleProjectionAc,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            label_selector: crate::OptionableConvert::try_from_optioned(
                value.label_selector,
            )?,
            name: crate::OptionableConvert::try_from_optioned(value.name)?,
            optional: crate::OptionableConvert::try_from_optioned(value.optional)?,
            path: crate::OptionableConvert::try_from_optioned(
                value
                    .path
                    .ok_or(crate::optionable::Error {
                        missing_field: "path",
                    })?,
            )?,
            signer_name: crate::OptionableConvert::try_from_optioned(value.signer_name)?,
        })
    }
    fn merge(
        &mut self,
        other: ClusterTrustBundleProjectionAc,
    ) -> Result<(), crate::optionable::Error> {
        crate::OptionableConvert::merge(&mut self.label_selector, other.label_selector)?;
        crate::OptionableConvert::merge(&mut self.name, other.name)?;
        crate::OptionableConvert::merge(&mut self.optional, other.optional)?;
        if let Some(other_value) = other.path {
            crate::OptionableConvert::merge(&mut self.path, other_value)?;
        }
        crate::OptionableConvert::merge(&mut self.signer_name, other.signer_name)?;
        Ok(())
    }
}

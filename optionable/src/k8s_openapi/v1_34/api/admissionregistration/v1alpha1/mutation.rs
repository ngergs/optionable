pub struct MutationOpt {
    pub apply_configuration: <Option<
        ::k8s_openapi::api::admissionregistration::v1alpha1::ApplyConfiguration,
    > as crate::Optionable>::Optioned,
    pub json_patch: <Option<
        ::k8s_openapi::api::admissionregistration::v1alpha1::JSONPatch,
    > as crate::Optionable>::Optioned,
    pub patch_type: Option<<std::string::String as crate::Optionable>::Optioned>,
}
#[automatically_derived]
impl crate::Optionable
for ::k8s_openapi::api::admissionregistration::v1alpha1::Mutation {
    type Optioned = MutationOpt;
}
#[automatically_derived]
impl crate::Optionable for MutationOpt {
    type Optioned = MutationOpt;
}
#[automatically_derived]
impl crate::OptionableConvert
for ::k8s_openapi::api::admissionregistration::v1alpha1::Mutation {
    fn into_optioned(self) -> MutationOpt {
        MutationOpt {
            apply_configuration: crate::OptionableConvert::into_optioned(
                self.apply_configuration,
            ),
            json_patch: crate::OptionableConvert::into_optioned(self.json_patch),
            patch_type: Some(crate::OptionableConvert::into_optioned(self.patch_type)),
        }
    }
    fn try_from_optioned(value: MutationOpt) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            apply_configuration: crate::OptionableConvert::try_from_optioned(
                value.apply_configuration,
            )?,
            json_patch: crate::OptionableConvert::try_from_optioned(value.json_patch)?,
            patch_type: crate::OptionableConvert::try_from_optioned(
                value
                    .patch_type
                    .ok_or(crate::optionable::Error {
                        missing_field: "patch_type",
                    })?,
            )?,
        })
    }
    fn merge(&mut self, other: MutationOpt) -> Result<(), crate::optionable::Error> {
        crate::OptionableConvert::merge(
            &mut self.apply_configuration,
            other.apply_configuration,
        )?;
        crate::OptionableConvert::merge(&mut self.json_patch, other.json_patch)?;
        if let Some(other_value) = other.patch_type {
            crate::OptionableConvert::merge(&mut self.patch_type, other_value)?;
        }
        Ok(())
    }
}

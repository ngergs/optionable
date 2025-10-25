pub struct PersistentVolumeClaimTemplateOpt {
    pub metadata: <Option<
        ::k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta,
    > as crate::Optionable>::Optioned,
    pub spec: Option<
        <::k8s_openapi::api::core::v1::PersistentVolumeClaimSpec as crate::Optionable>::Optioned,
    >,
}
#[automatically_derived]
impl crate::Optionable
for ::k8s_openapi::api::core::v1::persistent_volume_claim_template::PersistentVolumeClaimTemplate {
    type Optioned = PersistentVolumeClaimTemplateOpt;
}
#[automatically_derived]
impl crate::Optionable for PersistentVolumeClaimTemplateOpt {
    type Optioned = PersistentVolumeClaimTemplateOpt;
}
#[automatically_derived]
impl crate::OptionableConvert
for ::k8s_openapi::api::core::v1::persistent_volume_claim_template::PersistentVolumeClaimTemplate {
    fn into_optioned(self) -> PersistentVolumeClaimTemplateOpt {
        PersistentVolumeClaimTemplateOpt {
            metadata: <Option<
                ::k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta,
            > as crate::OptionableConvert>::into_optioned(self.metadata),
            spec: Some(
                <::k8s_openapi::api::core::v1::PersistentVolumeClaimSpec as crate::OptionableConvert>::into_optioned(
                    self.spec,
                ),
            ),
        }
    }
    fn try_from_optioned(
        value: PersistentVolumeClaimTemplateOpt,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            metadata: <Option<
                ::k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta,
            > as crate::OptionableConvert>::try_from_optioned(value.metadata)?,
            spec: <::k8s_openapi::api::core::v1::PersistentVolumeClaimSpec as crate::OptionableConvert>::try_from_optioned(
                value
                    .spec
                    .ok_or(crate::optionable::Error {
                        missing_field: "spec",
                    })?,
            )?,
        })
    }
    fn merge(
        &mut self,
        other: PersistentVolumeClaimTemplateOpt,
    ) -> Result<(), crate::optionable::Error> {
        <Option<
            ::k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta,
        > as crate::OptionableConvert>::merge(&mut self.metadata, other.metadata)?;
        if let Some(other_value) = other.spec {
            <::k8s_openapi::api::core::v1::PersistentVolumeClaimSpec as crate::OptionableConvert>::merge(
                &mut self.spec,
                other_value,
            )?;
        }
        Ok(())
    }
}

pub struct ResourceClaimTemplateSpecOpt {
    pub metadata: <Option<
        ::k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta,
    > as crate::Optionable>::Optioned,
    pub spec: Option<
        <::k8s_openapi::api::resource::v1beta2::ResourceClaimSpec as crate::Optionable>::Optioned,
    >,
}
#[automatically_derived]
impl crate::Optionable
for ::k8s_openapi::api::resource::v1beta2::ResourceClaimTemplateSpec {
    type Optioned = ResourceClaimTemplateSpecOpt;
}
#[automatically_derived]
impl crate::Optionable for ResourceClaimTemplateSpecOpt {
    type Optioned = ResourceClaimTemplateSpecOpt;
}
#[automatically_derived]
impl crate::OptionableConvert
for ::k8s_openapi::api::resource::v1beta2::ResourceClaimTemplateSpec {
    fn into_optioned(self) -> ResourceClaimTemplateSpecOpt {
        ResourceClaimTemplateSpecOpt {
            metadata: <Option<
                ::k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta,
            > as crate::OptionableConvert>::into_optioned(self.metadata),
            spec: Some(
                <::k8s_openapi::api::resource::v1beta2::ResourceClaimSpec as crate::OptionableConvert>::into_optioned(
                    self.spec,
                ),
            ),
        }
    }
    fn try_from_optioned(
        value: ResourceClaimTemplateSpecOpt,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            metadata: <Option<
                ::k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta,
            > as crate::OptionableConvert>::try_from_optioned(value.metadata)?,
            spec: <::k8s_openapi::api::resource::v1beta2::ResourceClaimSpec as crate::OptionableConvert>::try_from_optioned(
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
        other: ResourceClaimTemplateSpecOpt,
    ) -> Result<(), crate::optionable::Error> {
        <Option<
            ::k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta,
        > as crate::OptionableConvert>::merge(&mut self.metadata, other.metadata)?;
        if let Some(other_value) = other.spec {
            <::k8s_openapi::api::resource::v1beta2::ResourceClaimSpec as crate::OptionableConvert>::merge(
                &mut self.spec,
                other_value,
            )?;
        }
        Ok(())
    }
}

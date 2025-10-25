pub struct BindingOpt {
    pub metadata: Option<
        <::k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta as crate::Optionable>::Optioned,
    >,
    pub target: Option<
        <::k8s_openapi::api::core::v1::ObjectReference as crate::Optionable>::Optioned,
    >,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::core::v1::binding::Binding {
    type Optioned = BindingOpt;
}
#[automatically_derived]
impl crate::Optionable for BindingOpt {
    type Optioned = BindingOpt;
}
#[automatically_derived]
impl crate::OptionableConvert for ::k8s_openapi::api::core::v1::binding::Binding {
    fn into_optioned(self) -> BindingOpt {
        BindingOpt {
            metadata: Some(
                <::k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta as crate::OptionableConvert>::into_optioned(
                    self.metadata,
                ),
            ),
            target: Some(
                <::k8s_openapi::api::core::v1::ObjectReference as crate::OptionableConvert>::into_optioned(
                    self.target,
                ),
            ),
        }
    }
    fn try_from_optioned(value: BindingOpt) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            metadata: <::k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta as crate::OptionableConvert>::try_from_optioned(
                value
                    .metadata
                    .ok_or(crate::optionable::Error {
                        missing_field: "metadata",
                    })?,
            )?,
            target: <::k8s_openapi::api::core::v1::ObjectReference as crate::OptionableConvert>::try_from_optioned(
                value
                    .target
                    .ok_or(crate::optionable::Error {
                        missing_field: "target",
                    })?,
            )?,
        })
    }
    fn merge(&mut self, other: BindingOpt) -> Result<(), crate::optionable::Error> {
        if let Some(other_value) = other.metadata {
            <::k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta as crate::OptionableConvert>::merge(
                &mut self.metadata,
                other_value,
            )?;
        }
        if let Some(other_value) = other.target {
            <::k8s_openapi::api::core::v1::ObjectReference as crate::OptionableConvert>::merge(
                &mut self.target,
                other_value,
            )?;
        }
        Ok(())
    }
}

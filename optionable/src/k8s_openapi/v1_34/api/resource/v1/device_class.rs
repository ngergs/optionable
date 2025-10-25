pub struct DeviceClassOpt {
    pub metadata: Option<
        <::k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta as crate::Optionable>::Optioned,
    >,
    pub spec: Option<
        <::k8s_openapi::api::resource::v1::DeviceClassSpec as crate::Optionable>::Optioned,
    >,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::resource::v1::device_class::DeviceClass {
    type Optioned = DeviceClassOpt;
}
#[automatically_derived]
impl crate::Optionable for DeviceClassOpt {
    type Optioned = DeviceClassOpt;
}
#[automatically_derived]
impl crate::OptionableConvert
for ::k8s_openapi::api::resource::v1::device_class::DeviceClass {
    fn into_optioned(self) -> DeviceClassOpt {
        DeviceClassOpt {
            metadata: Some(
                <::k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta as crate::OptionableConvert>::into_optioned(
                    self.metadata,
                ),
            ),
            spec: Some(
                <::k8s_openapi::api::resource::v1::DeviceClassSpec as crate::OptionableConvert>::into_optioned(
                    self.spec,
                ),
            ),
        }
    }
    fn try_from_optioned(
        value: DeviceClassOpt,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            metadata: <::k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta as crate::OptionableConvert>::try_from_optioned(
                value
                    .metadata
                    .ok_or(crate::optionable::Error {
                        missing_field: "metadata",
                    })?,
            )?,
            spec: <::k8s_openapi::api::resource::v1::DeviceClassSpec as crate::OptionableConvert>::try_from_optioned(
                value
                    .spec
                    .ok_or(crate::optionable::Error {
                        missing_field: "spec",
                    })?,
            )?,
        })
    }
    fn merge(&mut self, other: DeviceClassOpt) -> Result<(), crate::optionable::Error> {
        if let Some(other_value) = other.metadata {
            <::k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta as crate::OptionableConvert>::merge(
                &mut self.metadata,
                other_value,
            )?;
        }
        if let Some(other_value) = other.spec {
            <::k8s_openapi::api::resource::v1::DeviceClassSpec as crate::OptionableConvert>::merge(
                &mut self.spec,
                other_value,
            )?;
        }
        Ok(())
    }
}

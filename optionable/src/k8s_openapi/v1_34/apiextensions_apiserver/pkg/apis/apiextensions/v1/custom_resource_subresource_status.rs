pub struct CustomResourceSubresourceStatusOpt(
    pub Option<<::k8s_openapi::serde_json::Value as crate::Optionable>::Optioned>,
);
#[automatically_derived]
impl crate::Optionable
for ::k8s_openapi::apiextensions_apiserver::pkg::apis::apiextensions::v1::custom_resource_subresource_status::CustomResourceSubresourceStatus {
    type Optioned = CustomResourceSubresourceStatusOpt;
}
#[automatically_derived]
impl crate::Optionable for CustomResourceSubresourceStatusOpt {
    type Optioned = CustomResourceSubresourceStatusOpt;
}
#[automatically_derived]
impl crate::OptionableConvert
for ::k8s_openapi::apiextensions_apiserver::pkg::apis::apiextensions::v1::custom_resource_subresource_status::CustomResourceSubresourceStatus {
    fn into_optioned(self) -> CustomResourceSubresourceStatusOpt {
        CustomResourceSubresourceStatusOpt(
            Some(
                <::k8s_openapi::serde_json::Value as crate::OptionableConvert>::into_optioned(
                    self.0,
                ),
            ),
        )
    }
    fn try_from_optioned(
        value: CustomResourceSubresourceStatusOpt,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(
            Self(
                <::k8s_openapi::serde_json::Value as crate::OptionableConvert>::try_from_optioned(
                    value
                        .0
                        .ok_or(crate::optionable::Error {
                            missing_field: "0",
                        })?,
                )?,
            ),
        )
    }
    fn merge(
        &mut self,
        other: CustomResourceSubresourceStatusOpt,
    ) -> Result<(), crate::optionable::Error> {
        if let Some(other_value) = other.0 {
            <::k8s_openapi::serde_json::Value as crate::OptionableConvert>::merge(
                &mut self.0,
                other_value,
            )?;
        }
        Ok(())
    }
}

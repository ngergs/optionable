pub struct CustomResourceSubresourceScaleOpt {
    pub label_selector_path: <Option<
        std::string::String,
    > as crate::Optionable>::Optioned,
    pub spec_replicas_path: Option<<std::string::String as crate::Optionable>::Optioned>,
    pub status_replicas_path: Option<
        <std::string::String as crate::Optionable>::Optioned,
    >,
}
#[automatically_derived]
impl crate::Optionable
for ::k8s_openapi::apiextensions_apiserver::pkg::apis::apiextensions::v1::custom_resource_subresource_scale::CustomResourceSubresourceScale {
    type Optioned = CustomResourceSubresourceScaleOpt;
}
#[automatically_derived]
impl crate::Optionable for CustomResourceSubresourceScaleOpt {
    type Optioned = CustomResourceSubresourceScaleOpt;
}
#[automatically_derived]
impl crate::OptionableConvert
for ::k8s_openapi::apiextensions_apiserver::pkg::apis::apiextensions::v1::custom_resource_subresource_scale::CustomResourceSubresourceScale {
    fn into_optioned(self) -> CustomResourceSubresourceScaleOpt {
        CustomResourceSubresourceScaleOpt {
            label_selector_path: <Option<
                std::string::String,
            > as crate::OptionableConvert>::into_optioned(self.label_selector_path),
            spec_replicas_path: Some(
                <std::string::String as crate::OptionableConvert>::into_optioned(
                    self.spec_replicas_path,
                ),
            ),
            status_replicas_path: Some(
                <std::string::String as crate::OptionableConvert>::into_optioned(
                    self.status_replicas_path,
                ),
            ),
        }
    }
    fn try_from_optioned(
        value: CustomResourceSubresourceScaleOpt,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            label_selector_path: <Option<
                std::string::String,
            > as crate::OptionableConvert>::try_from_optioned(
                value.label_selector_path,
            )?,
            spec_replicas_path: <std::string::String as crate::OptionableConvert>::try_from_optioned(
                value
                    .spec_replicas_path
                    .ok_or(crate::optionable::Error {
                        missing_field: "spec_replicas_path",
                    })?,
            )?,
            status_replicas_path: <std::string::String as crate::OptionableConvert>::try_from_optioned(
                value
                    .status_replicas_path
                    .ok_or(crate::optionable::Error {
                        missing_field: "status_replicas_path",
                    })?,
            )?,
        })
    }
    fn merge(
        &mut self,
        other: CustomResourceSubresourceScaleOpt,
    ) -> Result<(), crate::optionable::Error> {
        <Option<
            std::string::String,
        > as crate::OptionableConvert>::merge(
            &mut self.label_selector_path,
            other.label_selector_path,
        )?;
        if let Some(other_value) = other.spec_replicas_path {
            <std::string::String as crate::OptionableConvert>::merge(
                &mut self.spec_replicas_path,
                other_value,
            )?;
        }
        if let Some(other_value) = other.status_replicas_path {
            <std::string::String as crate::OptionableConvert>::merge(
                &mut self.status_replicas_path,
                other_value,
            )?;
        }
        Ok(())
    }
}

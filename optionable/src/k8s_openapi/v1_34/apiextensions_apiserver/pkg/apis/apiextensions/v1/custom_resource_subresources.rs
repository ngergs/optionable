pub struct CustomResourceSubresourcesOpt {
    pub scale: <Option<
        ::k8s_openapi::apiextensions_apiserver::pkg::apis::apiextensions::v1::CustomResourceSubresourceScale,
    > as crate::Optionable>::Optioned,
    pub status: <Option<
        ::k8s_openapi::apiextensions_apiserver::pkg::apis::apiextensions::v1::CustomResourceSubresourceStatus,
    > as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable
for ::k8s_openapi::apiextensions_apiserver::pkg::apis::apiextensions::v1::custom_resource_subresources::CustomResourceSubresources {
    type Optioned = CustomResourceSubresourcesOpt;
}
#[automatically_derived]
impl crate::Optionable for CustomResourceSubresourcesOpt {
    type Optioned = CustomResourceSubresourcesOpt;
}
#[automatically_derived]
impl crate::OptionableConvert
for ::k8s_openapi::apiextensions_apiserver::pkg::apis::apiextensions::v1::custom_resource_subresources::CustomResourceSubresources {
    fn into_optioned(self) -> CustomResourceSubresourcesOpt {
        CustomResourceSubresourcesOpt {
            scale: <Option<
                ::k8s_openapi::apiextensions_apiserver::pkg::apis::apiextensions::v1::CustomResourceSubresourceScale,
            > as crate::OptionableConvert>::into_optioned(self.scale),
            status: <Option<
                ::k8s_openapi::apiextensions_apiserver::pkg::apis::apiextensions::v1::CustomResourceSubresourceStatus,
            > as crate::OptionableConvert>::into_optioned(self.status),
        }
    }
    fn try_from_optioned(
        value: CustomResourceSubresourcesOpt,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            scale: <Option<
                ::k8s_openapi::apiextensions_apiserver::pkg::apis::apiextensions::v1::CustomResourceSubresourceScale,
            > as crate::OptionableConvert>::try_from_optioned(value.scale)?,
            status: <Option<
                ::k8s_openapi::apiextensions_apiserver::pkg::apis::apiextensions::v1::CustomResourceSubresourceStatus,
            > as crate::OptionableConvert>::try_from_optioned(value.status)?,
        })
    }
    fn merge(
        &mut self,
        other: CustomResourceSubresourcesOpt,
    ) -> Result<(), crate::optionable::Error> {
        <Option<
            ::k8s_openapi::apiextensions_apiserver::pkg::apis::apiextensions::v1::CustomResourceSubresourceScale,
        > as crate::OptionableConvert>::merge(&mut self.scale, other.scale)?;
        <Option<
            ::k8s_openapi::apiextensions_apiserver::pkg::apis::apiextensions::v1::CustomResourceSubresourceStatus,
        > as crate::OptionableConvert>::merge(&mut self.status, other.status)?;
        Ok(())
    }
}

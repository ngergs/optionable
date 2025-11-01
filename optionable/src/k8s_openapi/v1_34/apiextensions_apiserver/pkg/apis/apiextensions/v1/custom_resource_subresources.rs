pub struct CustomResourceSubresourcesAc {
    pub scale: <Option<
        ::k8s_openapi::apiextensions_apiserver::pkg::apis::apiextensions::v1::CustomResourceSubresourceScale,
    > as crate::Optionable>::Optioned,
    pub status: <Option<
        ::k8s_openapi::apiextensions_apiserver::pkg::apis::apiextensions::v1::CustomResourceSubresourceStatus,
    > as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable
for ::k8s_openapi::apiextensions_apiserver::pkg::apis::apiextensions::v1::CustomResourceSubresources {
    type Optioned = CustomResourceSubresourcesAc;
}
#[automatically_derived]
impl crate::Optionable for CustomResourceSubresourcesAc {
    type Optioned = CustomResourceSubresourcesAc;
}
#[automatically_derived]
impl crate::OptionableConvert
for ::k8s_openapi::apiextensions_apiserver::pkg::apis::apiextensions::v1::CustomResourceSubresources {
    fn into_optioned(self) -> CustomResourceSubresourcesAc {
        CustomResourceSubresourcesAc {
            scale: crate::OptionableConvert::into_optioned(self.scale),
            status: crate::OptionableConvert::into_optioned(self.status),
        }
    }
    fn try_from_optioned(
        value: CustomResourceSubresourcesAc,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            scale: crate::OptionableConvert::try_from_optioned(value.scale)?,
            status: crate::OptionableConvert::try_from_optioned(value.status)?,
        })
    }
    fn merge(
        &mut self,
        other: CustomResourceSubresourcesAc,
    ) -> Result<(), crate::optionable::Error> {
        crate::OptionableConvert::merge(&mut self.scale, other.scale)?;
        crate::OptionableConvert::merge(&mut self.status, other.status)?;
        Ok(())
    }
}

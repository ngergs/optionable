pub struct EndpointsOpt {
    pub metadata: ::k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta,
    pub subsets: <Option<
        std::vec::Vec<::k8s_openapi::api::core::v1::EndpointSubset>,
    > as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::core::v1::Endpoints {
    type Optioned = EndpointsOpt;
}
#[automatically_derived]
impl crate::Optionable for EndpointsOpt {
    type Optioned = EndpointsOpt;
}
#[automatically_derived]
impl crate::OptionableConvert for ::k8s_openapi::api::core::v1::Endpoints {
    fn into_optioned(self) -> EndpointsOpt {
        EndpointsOpt {
            metadata: self.metadata,
            subsets: crate::OptionableConvert::into_optioned(self.subsets),
        }
    }
    fn try_from_optioned(value: EndpointsOpt) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            metadata: value.metadata,
            subsets: crate::OptionableConvert::try_from_optioned(value.subsets)?,
        })
    }
    fn merge(&mut self, other: EndpointsOpt) -> Result<(), crate::optionable::Error> {
        self.metadata = other.metadata;
        crate::OptionableConvert::merge(&mut self.subsets, other.subsets)?;
        Ok(())
    }
}

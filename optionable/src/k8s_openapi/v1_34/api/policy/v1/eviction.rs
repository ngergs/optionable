pub struct EvictionOpt {
    pub delete_options: <Option<
        ::k8s_openapi::apimachinery::pkg::apis::meta::v1::DeleteOptions,
    > as crate::Optionable>::Optioned,
    pub metadata: ::k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::policy::v1::Eviction {
    type Optioned = EvictionOpt;
}
#[automatically_derived]
impl crate::Optionable for EvictionOpt {
    type Optioned = EvictionOpt;
}
#[automatically_derived]
impl crate::OptionableConvert for ::k8s_openapi::api::policy::v1::Eviction {
    fn into_optioned(self) -> EvictionOpt {
        EvictionOpt {
            delete_options: crate::OptionableConvert::into_optioned(self.delete_options),
            metadata: self.metadata,
        }
    }
    fn try_from_optioned(value: EvictionOpt) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            delete_options: crate::OptionableConvert::try_from_optioned(
                value.delete_options,
            )?,
            metadata: value.metadata,
        })
    }
    fn merge(&mut self, other: EvictionOpt) -> Result<(), crate::optionable::Error> {
        crate::OptionableConvert::merge(&mut self.delete_options, other.delete_options)?;
        self.metadata = other.metadata;
        Ok(())
    }
}

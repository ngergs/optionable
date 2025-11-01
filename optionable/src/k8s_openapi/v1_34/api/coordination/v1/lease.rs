pub struct LeaseOpt {
    pub metadata: ::k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta,
    pub spec: <Option<
        ::k8s_openapi::api::coordination::v1::LeaseSpec,
    > as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::coordination::v1::Lease {
    type Optioned = LeaseOpt;
}
#[automatically_derived]
impl crate::Optionable for LeaseOpt {
    type Optioned = LeaseOpt;
}
#[automatically_derived]
impl crate::OptionableConvert for ::k8s_openapi::api::coordination::v1::Lease {
    fn into_optioned(self) -> LeaseOpt {
        LeaseOpt {
            metadata: self.metadata,
            spec: crate::OptionableConvert::into_optioned(self.spec),
        }
    }
    fn try_from_optioned(value: LeaseOpt) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            metadata: value.metadata,
            spec: crate::OptionableConvert::try_from_optioned(value.spec)?,
        })
    }
    fn merge(&mut self, other: LeaseOpt) -> Result<(), crate::optionable::Error> {
        self.metadata = other.metadata;
        crate::OptionableConvert::merge(&mut self.spec, other.spec)?;
        Ok(())
    }
}

pub struct NetworkPolicyOpt {
    pub metadata: ::k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta,
    pub spec: <Option<
        ::k8s_openapi::api::networking::v1::NetworkPolicySpec,
    > as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::networking::v1::NetworkPolicy {
    type Optioned = NetworkPolicyOpt;
}
#[automatically_derived]
impl crate::Optionable for NetworkPolicyOpt {
    type Optioned = NetworkPolicyOpt;
}
#[automatically_derived]
impl crate::OptionableConvert for ::k8s_openapi::api::networking::v1::NetworkPolicy {
    fn into_optioned(self) -> NetworkPolicyOpt {
        NetworkPolicyOpt {
            metadata: self.metadata,
            spec: crate::OptionableConvert::into_optioned(self.spec),
        }
    }
    fn try_from_optioned(
        value: NetworkPolicyOpt,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            metadata: value.metadata,
            spec: crate::OptionableConvert::try_from_optioned(value.spec)?,
        })
    }
    fn merge(
        &mut self,
        other: NetworkPolicyOpt,
    ) -> Result<(), crate::optionable::Error> {
        self.metadata = other.metadata;
        crate::OptionableConvert::merge(&mut self.spec, other.spec)?;
        Ok(())
    }
}

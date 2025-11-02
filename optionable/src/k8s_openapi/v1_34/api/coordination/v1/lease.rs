#[derive(Clone, std::fmt::Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct LeaseAc {
    pub metadata: ::k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spec: <Option<
        ::k8s_openapi::api::coordination::v1::LeaseSpec,
    > as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::coordination::v1::Lease {
    type Optioned = LeaseAc;
}
#[automatically_derived]
impl crate::Optionable for LeaseAc {
    type Optioned = LeaseAc;
}
#[automatically_derived]
impl crate::OptionableConvert for ::k8s_openapi::api::coordination::v1::Lease {
    fn into_optioned(self) -> LeaseAc {
        LeaseAc {
            metadata: self.metadata,
            spec: crate::OptionableConvert::into_optioned(self.spec),
        }
    }
    fn try_from_optioned(value: LeaseAc) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            metadata: value.metadata,
            spec: crate::OptionableConvert::try_from_optioned(value.spec)?,
        })
    }
    fn merge(&mut self, other: LeaseAc) -> Result<(), crate::optionable::Error> {
        self.metadata = other.metadata;
        crate::OptionableConvert::merge(&mut self.spec, other.spec)?;
        Ok(())
    }
}
impl k8s_openapi::Resource for LeaseAc {
    const API_VERSION: &'static str = "coordination.k8s.io/v1";
    const GROUP: &'static str = "coordination.k8s.io";
    const KIND: &'static str = "Lease";
    const VERSION: &'static str = "v1";
    const URL_PATH_SEGMENT: &'static str = "leases";
    type Scope = k8s_openapi::NamespaceResourceScope;
}
impl k8s_openapi::Metadata for LeaseAc {
    type Ty = k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta;
    fn metadata(&self) -> &<Self as k8s_openapi::Metadata>::Ty {
        &self.metadata
    }
    fn metadata_mut(&mut self) -> &mut <Self as k8s_openapi::Metadata>::Ty {
        &mut self.metadata
    }
}

pub struct ResourceClaimOpt {
    pub metadata: ::k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta,
    pub spec: Option<
        <::k8s_openapi::api::resource::v1::ResourceClaimSpec as crate::Optionable>::Optioned,
    >,
    pub status: <Option<
        ::k8s_openapi::api::resource::v1::ResourceClaimStatus,
    > as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::resource::v1::ResourceClaim {
    type Optioned = ResourceClaimOpt;
}
#[automatically_derived]
impl crate::Optionable for ResourceClaimOpt {
    type Optioned = ResourceClaimOpt;
}
#[automatically_derived]
impl crate::OptionableConvert for ::k8s_openapi::api::resource::v1::ResourceClaim {
    fn into_optioned(self) -> ResourceClaimOpt {
        ResourceClaimOpt {
            metadata: self.metadata,
            spec: Some(crate::OptionableConvert::into_optioned(self.spec)),
            status: crate::OptionableConvert::into_optioned(self.status),
        }
    }
    fn try_from_optioned(
        value: ResourceClaimOpt,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            metadata: value.metadata,
            spec: crate::OptionableConvert::try_from_optioned(
                value
                    .spec
                    .ok_or(crate::optionable::Error {
                        missing_field: "spec",
                    })?,
            )?,
            status: crate::OptionableConvert::try_from_optioned(value.status)?,
        })
    }
    fn merge(
        &mut self,
        other: ResourceClaimOpt,
    ) -> Result<(), crate::optionable::Error> {
        self.metadata = other.metadata;
        if let Some(other_value) = other.spec {
            crate::OptionableConvert::merge(&mut self.spec, other_value)?;
        }
        crate::OptionableConvert::merge(&mut self.status, other.status)?;
        Ok(())
    }
}

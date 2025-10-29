pub struct IngressClassOpt {
    pub metadata: Option<
        <::k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta as crate::Optionable>::Optioned,
    >,
    pub spec: <Option<
        ::k8s_openapi::api::networking::v1::IngressClassSpec,
    > as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::networking::v1::IngressClass {
    type Optioned = IngressClassOpt;
}
#[automatically_derived]
impl crate::Optionable for IngressClassOpt {
    type Optioned = IngressClassOpt;
}
#[automatically_derived]
impl crate::OptionableConvert for ::k8s_openapi::api::networking::v1::IngressClass {
    fn into_optioned(self) -> IngressClassOpt {
        IngressClassOpt {
            metadata: Some(crate::OptionableConvert::into_optioned(self.metadata)),
            spec: crate::OptionableConvert::into_optioned(self.spec),
        }
    }
    fn try_from_optioned(
        value: IngressClassOpt,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            metadata: crate::OptionableConvert::try_from_optioned(
                value
                    .metadata
                    .ok_or(crate::optionable::Error {
                        missing_field: "metadata",
                    })?,
            )?,
            spec: crate::OptionableConvert::try_from_optioned(value.spec)?,
        })
    }
    fn merge(&mut self, other: IngressClassOpt) -> Result<(), crate::optionable::Error> {
        if let Some(other_value) = other.metadata {
            crate::OptionableConvert::merge(&mut self.metadata, other_value)?;
        }
        crate::OptionableConvert::merge(&mut self.spec, other.spec)?;
        Ok(())
    }
}

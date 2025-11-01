pub struct CSINodeOpt {
    pub metadata: ::k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta,
    pub spec: Option<
        <::k8s_openapi::api::storage::v1::CSINodeSpec as crate::Optionable>::Optioned,
    >,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::storage::v1::CSINode {
    type Optioned = CSINodeOpt;
}
#[automatically_derived]
impl crate::Optionable for CSINodeOpt {
    type Optioned = CSINodeOpt;
}
#[automatically_derived]
impl crate::OptionableConvert for ::k8s_openapi::api::storage::v1::CSINode {
    fn into_optioned(self) -> CSINodeOpt {
        CSINodeOpt {
            metadata: self.metadata,
            spec: Some(crate::OptionableConvert::into_optioned(self.spec)),
        }
    }
    fn try_from_optioned(value: CSINodeOpt) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            metadata: value.metadata,
            spec: crate::OptionableConvert::try_from_optioned(
                value
                    .spec
                    .ok_or(crate::optionable::Error {
                        missing_field: "spec",
                    })?,
            )?,
        })
    }
    fn merge(&mut self, other: CSINodeOpt) -> Result<(), crate::optionable::Error> {
        self.metadata = other.metadata;
        if let Some(other_value) = other.spec {
            crate::OptionableConvert::merge(&mut self.spec, other_value)?;
        }
        Ok(())
    }
}

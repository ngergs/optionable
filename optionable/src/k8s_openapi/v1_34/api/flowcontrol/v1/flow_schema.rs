pub struct FlowSchemaOpt {
    pub metadata: Option<
        <::k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta as crate::Optionable>::Optioned,
    >,
    pub spec: <Option<
        ::k8s_openapi::api::flowcontrol::v1::FlowSchemaSpec,
    > as crate::Optionable>::Optioned,
    pub status: <Option<
        ::k8s_openapi::api::flowcontrol::v1::FlowSchemaStatus,
    > as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::flowcontrol::v1::FlowSchema {
    type Optioned = FlowSchemaOpt;
}
#[automatically_derived]
impl crate::Optionable for FlowSchemaOpt {
    type Optioned = FlowSchemaOpt;
}
#[automatically_derived]
impl crate::OptionableConvert for ::k8s_openapi::api::flowcontrol::v1::FlowSchema {
    fn into_optioned(self) -> FlowSchemaOpt {
        FlowSchemaOpt {
            metadata: Some(crate::OptionableConvert::into_optioned(self.metadata)),
            spec: crate::OptionableConvert::into_optioned(self.spec),
            status: crate::OptionableConvert::into_optioned(self.status),
        }
    }
    fn try_from_optioned(
        value: FlowSchemaOpt,
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
            status: crate::OptionableConvert::try_from_optioned(value.status)?,
        })
    }
    fn merge(&mut self, other: FlowSchemaOpt) -> Result<(), crate::optionable::Error> {
        if let Some(other_value) = other.metadata {
            crate::OptionableConvert::merge(&mut self.metadata, other_value)?;
        }
        crate::OptionableConvert::merge(&mut self.spec, other.spec)?;
        crate::OptionableConvert::merge(&mut self.status, other.status)?;
        Ok(())
    }
}

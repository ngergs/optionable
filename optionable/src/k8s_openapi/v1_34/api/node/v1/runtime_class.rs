pub struct RuntimeClassOpt {
    pub handler: Option<<std::string::String as crate::Optionable>::Optioned>,
    pub metadata: Option<
        <::k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta as crate::Optionable>::Optioned,
    >,
    pub overhead: <Option<
        ::k8s_openapi::api::node::v1::Overhead,
    > as crate::Optionable>::Optioned,
    pub scheduling: <Option<
        ::k8s_openapi::api::node::v1::Scheduling,
    > as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::node::v1::runtime_class::RuntimeClass {
    type Optioned = RuntimeClassOpt;
}
#[automatically_derived]
impl crate::Optionable for RuntimeClassOpt {
    type Optioned = RuntimeClassOpt;
}
#[automatically_derived]
impl crate::OptionableConvert
for ::k8s_openapi::api::node::v1::runtime_class::RuntimeClass {
    fn into_optioned(self) -> RuntimeClassOpt {
        RuntimeClassOpt {
            handler: Some(
                <std::string::String as crate::OptionableConvert>::into_optioned(
                    self.handler,
                ),
            ),
            metadata: Some(
                <::k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta as crate::OptionableConvert>::into_optioned(
                    self.metadata,
                ),
            ),
            overhead: <Option<
                ::k8s_openapi::api::node::v1::Overhead,
            > as crate::OptionableConvert>::into_optioned(self.overhead),
            scheduling: <Option<
                ::k8s_openapi::api::node::v1::Scheduling,
            > as crate::OptionableConvert>::into_optioned(self.scheduling),
        }
    }
    fn try_from_optioned(
        value: RuntimeClassOpt,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            handler: <std::string::String as crate::OptionableConvert>::try_from_optioned(
                value
                    .handler
                    .ok_or(crate::optionable::Error {
                        missing_field: "handler",
                    })?,
            )?,
            metadata: <::k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta as crate::OptionableConvert>::try_from_optioned(
                value
                    .metadata
                    .ok_or(crate::optionable::Error {
                        missing_field: "metadata",
                    })?,
            )?,
            overhead: <Option<
                ::k8s_openapi::api::node::v1::Overhead,
            > as crate::OptionableConvert>::try_from_optioned(value.overhead)?,
            scheduling: <Option<
                ::k8s_openapi::api::node::v1::Scheduling,
            > as crate::OptionableConvert>::try_from_optioned(value.scheduling)?,
        })
    }
    fn merge(&mut self, other: RuntimeClassOpt) -> Result<(), crate::optionable::Error> {
        if let Some(other_value) = other.handler {
            <std::string::String as crate::OptionableConvert>::merge(
                &mut self.handler,
                other_value,
            )?;
        }
        if let Some(other_value) = other.metadata {
            <::k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta as crate::OptionableConvert>::merge(
                &mut self.metadata,
                other_value,
            )?;
        }
        <Option<
            ::k8s_openapi::api::node::v1::Overhead,
        > as crate::OptionableConvert>::merge(&mut self.overhead, other.overhead)?;
        <Option<
            ::k8s_openapi::api::node::v1::Scheduling,
        > as crate::OptionableConvert>::merge(&mut self.scheduling, other.scheduling)?;
        Ok(())
    }
}

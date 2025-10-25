pub struct ControllerRevisionOpt {
    pub data: <Option<
        ::k8s_openapi::apimachinery::pkg::runtime::RawExtension,
    > as crate::Optionable>::Optioned,
    pub metadata: Option<
        <::k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta as crate::Optionable>::Optioned,
    >,
    pub revision: Option<i64>,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::apps::v1::ControllerRevision {
    type Optioned = ControllerRevisionOpt;
}
#[automatically_derived]
impl crate::Optionable for ControllerRevisionOpt {
    type Optioned = ControllerRevisionOpt;
}
#[automatically_derived]
impl crate::OptionableConvert for ::k8s_openapi::api::apps::v1::ControllerRevision {
    fn into_optioned(self) -> ControllerRevisionOpt {
        ControllerRevisionOpt {
            data: <Option<
                ::k8s_openapi::apimachinery::pkg::runtime::RawExtension,
            > as crate::OptionableConvert>::into_optioned(self.data),
            metadata: Some(
                <::k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta as crate::OptionableConvert>::into_optioned(
                    self.metadata,
                ),
            ),
            revision: Some(self.revision),
        }
    }
    fn try_from_optioned(
        value: ControllerRevisionOpt,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            data: <Option<
                ::k8s_openapi::apimachinery::pkg::runtime::RawExtension,
            > as crate::OptionableConvert>::try_from_optioned(value.data)?,
            metadata: <::k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta as crate::OptionableConvert>::try_from_optioned(
                value
                    .metadata
                    .ok_or(crate::optionable::Error {
                        missing_field: "metadata",
                    })?,
            )?,
            revision: value
                .revision
                .ok_or(crate::optionable::Error {
                    missing_field: "revision",
                })?,
        })
    }
    fn merge(
        &mut self,
        other: ControllerRevisionOpt,
    ) -> Result<(), crate::optionable::Error> {
        <Option<
            ::k8s_openapi::apimachinery::pkg::runtime::RawExtension,
        > as crate::OptionableConvert>::merge(&mut self.data, other.data)?;
        if let Some(other_value) = other.metadata {
            <::k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta as crate::OptionableConvert>::merge(
                &mut self.metadata,
                other_value,
            )?;
        }
        if let Some(other_value) = other.revision {
            self.revision = other_value;
        }
        Ok(())
    }
}

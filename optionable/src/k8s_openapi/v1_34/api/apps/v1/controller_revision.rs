#[derive(kube::Resource)]
#[resource(inherit = ControllerRevision)]
pub struct ControllerRevisionAc {
    pub data: <Option<
        ::k8s_openapi::apimachinery::pkg::runtime::RawExtension,
    > as crate::Optionable>::Optioned,
    pub metadata: ::k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta,
    pub revision: Option<i64>,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::apps::v1::ControllerRevision {
    type Optioned = ControllerRevisionAc;
}
#[automatically_derived]
impl crate::Optionable for ControllerRevisionAc {
    type Optioned = ControllerRevisionAc;
}
#[automatically_derived]
impl crate::OptionableConvert for ::k8s_openapi::api::apps::v1::ControllerRevision {
    fn into_optioned(self) -> ControllerRevisionAc {
        ControllerRevisionAc {
            data: crate::OptionableConvert::into_optioned(self.data),
            metadata: self.metadata,
            revision: Some(self.revision),
        }
    }
    fn try_from_optioned(
        value: ControllerRevisionAc,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            data: crate::OptionableConvert::try_from_optioned(value.data)?,
            metadata: value.metadata,
            revision: value
                .revision
                .ok_or(crate::optionable::Error {
                    missing_field: "revision",
                })?,
        })
    }
    fn merge(
        &mut self,
        other: ControllerRevisionAc,
    ) -> Result<(), crate::optionable::Error> {
        crate::OptionableConvert::merge(&mut self.data, other.data)?;
        self.metadata = other.metadata;
        if let Some(other_value) = other.revision {
            self.revision = other_value;
        }
        Ok(())
    }
}
#[allow(unused_imports)]
use ::k8s_openapi::api::apps::v1::ControllerRevision;

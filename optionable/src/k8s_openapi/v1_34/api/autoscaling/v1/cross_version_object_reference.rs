#[derive(Clone, std::fmt::Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CrossVersionObjectReferenceAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_version: <Option<std::string::String> as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kind: Option<<std::string::String as crate::Optionable>::Optioned>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<<std::string::String as crate::Optionable>::Optioned>,
}
#[automatically_derived]
impl crate::Optionable
for ::k8s_openapi::api::autoscaling::v1::CrossVersionObjectReference {
    type Optioned = CrossVersionObjectReferenceAc;
}
#[automatically_derived]
impl crate::Optionable for CrossVersionObjectReferenceAc {
    type Optioned = CrossVersionObjectReferenceAc;
}
#[automatically_derived]
impl crate::OptionableConvert
for ::k8s_openapi::api::autoscaling::v1::CrossVersionObjectReference {
    fn into_optioned(self) -> CrossVersionObjectReferenceAc {
        CrossVersionObjectReferenceAc {
            api_version: crate::OptionableConvert::into_optioned(self.api_version),
            kind: Some(crate::OptionableConvert::into_optioned(self.kind)),
            name: Some(crate::OptionableConvert::into_optioned(self.name)),
        }
    }
    fn try_from_optioned(
        value: CrossVersionObjectReferenceAc,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            api_version: crate::OptionableConvert::try_from_optioned(value.api_version)?,
            kind: crate::OptionableConvert::try_from_optioned(
                value
                    .kind
                    .ok_or(crate::optionable::Error {
                        missing_field: "kind",
                    })?,
            )?,
            name: crate::OptionableConvert::try_from_optioned(
                value
                    .name
                    .ok_or(crate::optionable::Error {
                        missing_field: "name",
                    })?,
            )?,
        })
    }
    fn merge(
        &mut self,
        other: CrossVersionObjectReferenceAc,
    ) -> Result<(), crate::optionable::Error> {
        crate::OptionableConvert::merge(&mut self.api_version, other.api_version)?;
        if let Some(other_value) = other.kind {
            crate::OptionableConvert::merge(&mut self.kind, other_value)?;
        }
        if let Some(other_value) = other.name {
            crate::OptionableConvert::merge(&mut self.name, other_value)?;
        }
        Ok(())
    }
}

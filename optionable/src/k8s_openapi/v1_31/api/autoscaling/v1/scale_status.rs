#[derive(
    Clone,
    std::fmt::Debug,
    Default,
    PartialEq,
    serde::Serialize,
    serde::Deserialize
)]
#[serde(rename_all = "camelCase")]
pub struct ScaleStatusAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replicas: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selector: <Option<std::string::String> as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::autoscaling::v1::ScaleStatus {
    type Optioned = ScaleStatusAc;
}
#[automatically_derived]
impl crate::Optionable for ScaleStatusAc {
    type Optioned = ScaleStatusAc;
}
#[automatically_derived]
impl crate::OptionableConvert for ::k8s_openapi::api::autoscaling::v1::ScaleStatus {
    fn into_optioned(self) -> ScaleStatusAc {
        ScaleStatusAc {
            replicas: Some(self.replicas),
            selector: crate::OptionableConvert::into_optioned(self.selector),
        }
    }
    fn try_from_optioned(value: ScaleStatusAc) -> Result<Self, crate::Error> {
        Ok(Self {
            replicas: value
                .replicas
                .ok_or(crate::Error {
                    missing_field: "replicas",
                })?,
            selector: crate::OptionableConvert::try_from_optioned(value.selector)?,
        })
    }
    fn merge(&mut self, other: ScaleStatusAc) -> Result<(), crate::Error> {
        if let Some(other_value) = other.replicas {
            self.replicas = other_value;
        }
        crate::OptionableConvert::merge(&mut self.selector, other.selector)?;
        Ok(())
    }
}

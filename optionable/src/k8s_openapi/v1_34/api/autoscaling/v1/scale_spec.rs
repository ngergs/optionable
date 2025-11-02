#[derive(Clone, std::fmt::Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct ScaleSpecAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replicas: <Option<i32> as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::autoscaling::v1::ScaleSpec {
    type Optioned = ScaleSpecAc;
}
#[automatically_derived]
impl crate::Optionable for ScaleSpecAc {
    type Optioned = ScaleSpecAc;
}
#[automatically_derived]
impl crate::OptionableConvert for ::k8s_openapi::api::autoscaling::v1::ScaleSpec {
    fn into_optioned(self) -> ScaleSpecAc {
        ScaleSpecAc {
            replicas: crate::OptionableConvert::into_optioned(self.replicas),
        }
    }
    fn try_from_optioned(value: ScaleSpecAc) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            replicas: crate::OptionableConvert::try_from_optioned(value.replicas)?,
        })
    }
    fn merge(&mut self, other: ScaleSpecAc) -> Result<(), crate::optionable::Error> {
        crate::OptionableConvert::merge(&mut self.replicas, other.replicas)?;
        Ok(())
    }
}

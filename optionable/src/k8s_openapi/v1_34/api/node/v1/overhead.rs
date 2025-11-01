pub struct OverheadAc {
    pub pod_fixed: <Option<
        std::collections::BTreeMap<
            std::string::String,
            ::k8s_openapi::apimachinery::pkg::api::resource::Quantity,
        >,
    > as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::node::v1::Overhead {
    type Optioned = OverheadAc;
}
#[automatically_derived]
impl crate::Optionable for OverheadAc {
    type Optioned = OverheadAc;
}
#[automatically_derived]
impl crate::OptionableConvert for ::k8s_openapi::api::node::v1::Overhead {
    fn into_optioned(self) -> OverheadAc {
        OverheadAc {
            pod_fixed: crate::OptionableConvert::into_optioned(self.pod_fixed),
        }
    }
    fn try_from_optioned(value: OverheadAc) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            pod_fixed: crate::OptionableConvert::try_from_optioned(value.pod_fixed)?,
        })
    }
    fn merge(&mut self, other: OverheadAc) -> Result<(), crate::optionable::Error> {
        crate::OptionableConvert::merge(&mut self.pod_fixed, other.pod_fixed)?;
        Ok(())
    }
}

#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
#[serde(from = "crate::k8s_openapi027::apimachinery::pkg::util::intstr::IntOrStringAc")]
pub struct QuantityAc(
    #[serde(skip_serializing_if = "Option::is_none")]
    pub Option<<std::string::String as crate::Optionable>::Optioned>,
);
#[automatically_derived]
impl crate::Optionable for k8s_openapi027::apimachinery::pkg::api::resource::Quantity {
    type Optioned = QuantityAc;
}
#[automatically_derived]
impl crate::Optionable for QuantityAc {
    type Optioned = QuantityAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert
for k8s_openapi027::apimachinery::pkg::api::resource::Quantity {
    fn into_optioned(self) -> QuantityAc {
        QuantityAc(Some(crate::OptionableConvert::into_optioned(self.0)))
    }
    fn try_from_optioned(value: QuantityAc) -> Result<Self, crate::Error> {
        Ok(
            Self(
                crate::OptionableConvert::try_from_optioned(
                    value.0.ok_or(crate::Error { missing_field: "0" })?,
                )?,
            ),
        )
    }
    fn merge(&mut self, other: QuantityAc) -> Result<(), crate::Error> {
        if let Some(other_value) = other.0 {
            crate::OptionableConvert::merge(&mut self.0, other_value)?;
        }
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<k8s_openapi027::apimachinery::pkg::api::resource::Quantity>
for QuantityAc {
    fn from_optionable(
        value: k8s_openapi027::apimachinery::pkg::api::resource::Quantity,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<
        k8s_openapi027::apimachinery::pkg::api::resource::Quantity,
        crate::Error,
    > {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::apimachinery::pkg::api::resource::Quantity,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
impl From<crate::k8s_openapi027::apimachinery::pkg::util::intstr::IntOrStringAc>
for QuantityAc {
    fn from(
        value: crate::k8s_openapi027::apimachinery::pkg::util::intstr::IntOrStringAc,
    ) -> Self {
        QuantityAc(
            match value {
                crate::k8s_openapi027::apimachinery::pkg::util::intstr::IntOrStringAc::Int(
                    i,
                ) => i.map(|i| i.to_string()),
                crate::k8s_openapi027::apimachinery::pkg::util::intstr::IntOrStringAc::String(
                    s,
                ) => s,
            },
        )
    }
}

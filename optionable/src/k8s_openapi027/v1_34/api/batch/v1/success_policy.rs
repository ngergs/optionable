#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct SuccessPolicyAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rules: Option<
        <std::vec::Vec<
            ::k8s_openapi027::api::batch::v1::SuccessPolicyRule,
        > as crate::Optionable>::Optioned,
    >,
}
#[automatically_derived]
impl crate::Optionable for k8s_openapi027::api::batch::v1::SuccessPolicy {
    type Optioned = SuccessPolicyAc;
}
#[automatically_derived]
impl crate::Optionable for SuccessPolicyAc {
    type Optioned = SuccessPolicyAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert for k8s_openapi027::api::batch::v1::SuccessPolicy {
    fn into_optioned(self) -> SuccessPolicyAc {
        SuccessPolicyAc {
            rules: Some(crate::OptionableConvert::into_optioned(self.rules)),
        }
    }
    fn try_from_optioned(value: SuccessPolicyAc) -> Result<Self, crate::Error> {
        Ok(Self {
            rules: crate::OptionableConvert::try_from_optioned(
                value
                    .rules
                    .ok_or(crate::Error {
                        missing_field: "rules",
                    })?,
            )?,
        })
    }
    fn merge(&mut self, other: SuccessPolicyAc) -> Result<(), crate::Error> {
        if let Some(other_value) = other.rules {
            crate::OptionableConvert::merge(&mut self.rules, other_value)?;
        }
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<k8s_openapi027::api::batch::v1::SuccessPolicy>
for SuccessPolicyAc {
    fn from_optionable(value: k8s_openapi027::api::batch::v1::SuccessPolicy) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<k8s_openapi027::api::batch::v1::SuccessPolicy, crate::Error> {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::api::batch::v1::SuccessPolicy,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}

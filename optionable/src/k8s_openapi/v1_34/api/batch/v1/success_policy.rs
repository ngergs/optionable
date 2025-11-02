#[derive(Clone, std::fmt::Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SuccessPolicyAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rules: Option<
        <std::vec::Vec<
            ::k8s_openapi::api::batch::v1::SuccessPolicyRule,
        > as crate::Optionable>::Optioned,
    >,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::batch::v1::SuccessPolicy {
    type Optioned = SuccessPolicyAc;
}
#[automatically_derived]
impl crate::Optionable for SuccessPolicyAc {
    type Optioned = SuccessPolicyAc;
}
#[automatically_derived]
impl crate::OptionableConvert for ::k8s_openapi::api::batch::v1::SuccessPolicy {
    fn into_optioned(self) -> SuccessPolicyAc {
        SuccessPolicyAc {
            rules: Some(crate::OptionableConvert::into_optioned(self.rules)),
        }
    }
    fn try_from_optioned(
        value: SuccessPolicyAc,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            rules: crate::OptionableConvert::try_from_optioned(
                value
                    .rules
                    .ok_or(crate::optionable::Error {
                        missing_field: "rules",
                    })?,
            )?,
        })
    }
    fn merge(&mut self, other: SuccessPolicyAc) -> Result<(), crate::optionable::Error> {
        if let Some(other_value) = other.rules {
            crate::OptionableConvert::merge(&mut self.rules, other_value)?;
        }
        Ok(())
    }
}

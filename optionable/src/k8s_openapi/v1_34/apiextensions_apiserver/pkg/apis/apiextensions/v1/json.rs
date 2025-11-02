#[derive(Clone, std::fmt::Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct JSONAc(
    #[serde(skip_serializing_if = "Option::is_none")]
    pub Option<<::k8s_openapi::serde_json::Value as crate::Optionable>::Optioned>,
);
#[automatically_derived]
impl crate::Optionable
for ::k8s_openapi::apiextensions_apiserver::pkg::apis::apiextensions::v1::JSON {
    type Optioned = JSONAc;
}
#[automatically_derived]
impl crate::Optionable for JSONAc {
    type Optioned = JSONAc;
}
#[automatically_derived]
impl crate::OptionableConvert
for ::k8s_openapi::apiextensions_apiserver::pkg::apis::apiextensions::v1::JSON {
    fn into_optioned(self) -> JSONAc {
        JSONAc(Some(crate::OptionableConvert::into_optioned(self.0)))
    }
    fn try_from_optioned(value: JSONAc) -> Result<Self, crate::optionable::Error> {
        Ok(
            Self(
                crate::OptionableConvert::try_from_optioned(
                    value
                        .0
                        .ok_or(crate::optionable::Error {
                            missing_field: "0",
                        })?,
                )?,
            ),
        )
    }
    fn merge(&mut self, other: JSONAc) -> Result<(), crate::optionable::Error> {
        if let Some(other_value) = other.0 {
            crate::OptionableConvert::merge(&mut self.0, other_value)?;
        }
        Ok(())
    }
}

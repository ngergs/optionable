pub struct JSONOpt(
    pub Option<<::k8s_openapi::serde_json::Value as crate::Optionable>::Optioned>,
);
#[automatically_derived]
impl crate::Optionable
for ::k8s_openapi::apiextensions_apiserver::pkg::apis::apiextensions::v1::json::JSON {
    type Optioned = JSONOpt;
}
#[automatically_derived]
impl crate::Optionable for JSONOpt {
    type Optioned = JSONOpt;
}
#[automatically_derived]
impl crate::OptionableConvert
for ::k8s_openapi::apiextensions_apiserver::pkg::apis::apiextensions::v1::json::JSON {
    fn into_optioned(self) -> JSONOpt {
        JSONOpt(
            Some(
                <::k8s_openapi::serde_json::Value as crate::OptionableConvert>::into_optioned(
                    self.0,
                ),
            ),
        )
    }
    fn try_from_optioned(value: JSONOpt) -> Result<Self, crate::optionable::Error> {
        Ok(
            Self(
                <::k8s_openapi::serde_json::Value as crate::OptionableConvert>::try_from_optioned(
                    value
                        .0
                        .ok_or(crate::optionable::Error {
                            missing_field: "0",
                        })?,
                )?,
            ),
        )
    }
    fn merge(&mut self, other: JSONOpt) -> Result<(), crate::optionable::Error> {
        if let Some(other_value) = other.0 {
            <::k8s_openapi::serde_json::Value as crate::OptionableConvert>::merge(
                &mut self.0,
                other_value,
            )?;
        }
        Ok(())
    }
}

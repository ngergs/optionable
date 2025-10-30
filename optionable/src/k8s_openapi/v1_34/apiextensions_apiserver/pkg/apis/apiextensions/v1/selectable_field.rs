pub struct SelectableFieldOpt {
    pub json_path: Option<<std::string::String as crate::Optionable>::Optioned>,
}
#[automatically_derived]
impl crate::Optionable
for ::k8s_openapi::apiextensions_apiserver::pkg::apis::apiextensions::v1::SelectableField {
    type Optioned = SelectableFieldOpt;
}
#[automatically_derived]
impl crate::Optionable for SelectableFieldOpt {
    type Optioned = SelectableFieldOpt;
}
#[automatically_derived]
impl crate::OptionableConvert
for ::k8s_openapi::apiextensions_apiserver::pkg::apis::apiextensions::v1::SelectableField {
    fn into_optioned(self) -> SelectableFieldOpt {
        SelectableFieldOpt {
            json_path: Some(crate::OptionableConvert::into_optioned(self.json_path)),
        }
    }
    fn try_from_optioned(
        value: SelectableFieldOpt,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            json_path: crate::OptionableConvert::try_from_optioned(
                value
                    .json_path
                    .ok_or(crate::optionable::Error {
                        missing_field: "json_path",
                    })?,
            )?,
        })
    }
    fn merge(
        &mut self,
        other: SelectableFieldOpt,
    ) -> Result<(), crate::optionable::Error> {
        if let Some(other_value) = other.json_path {
            crate::OptionableConvert::merge(&mut self.json_path, other_value)?;
        }
        Ok(())
    }
}

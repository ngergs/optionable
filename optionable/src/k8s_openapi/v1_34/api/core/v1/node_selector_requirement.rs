pub struct NodeSelectorRequirementOpt {
    pub key: Option<<std::string::String as crate::Optionable>::Optioned>,
    pub operator: Option<<std::string::String as crate::Optionable>::Optioned>,
    pub values: <Option<
        std::vec::Vec<std::string::String>,
    > as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::core::v1::NodeSelectorRequirement {
    type Optioned = NodeSelectorRequirementOpt;
}
#[automatically_derived]
impl crate::Optionable for NodeSelectorRequirementOpt {
    type Optioned = NodeSelectorRequirementOpt;
}
#[automatically_derived]
impl crate::OptionableConvert for ::k8s_openapi::api::core::v1::NodeSelectorRequirement {
    fn into_optioned(self) -> NodeSelectorRequirementOpt {
        NodeSelectorRequirementOpt {
            key: Some(
                <std::string::String as crate::OptionableConvert>::into_optioned(
                    self.key,
                ),
            ),
            operator: Some(
                <std::string::String as crate::OptionableConvert>::into_optioned(
                    self.operator,
                ),
            ),
            values: <Option<
                std::vec::Vec<std::string::String>,
            > as crate::OptionableConvert>::into_optioned(self.values),
        }
    }
    fn try_from_optioned(
        value: NodeSelectorRequirementOpt,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            key: <std::string::String as crate::OptionableConvert>::try_from_optioned(
                value
                    .key
                    .ok_or(crate::optionable::Error {
                        missing_field: "key",
                    })?,
            )?,
            operator: <std::string::String as crate::OptionableConvert>::try_from_optioned(
                value
                    .operator
                    .ok_or(crate::optionable::Error {
                        missing_field: "operator",
                    })?,
            )?,
            values: <Option<
                std::vec::Vec<std::string::String>,
            > as crate::OptionableConvert>::try_from_optioned(value.values)?,
        })
    }
    fn merge(
        &mut self,
        other: NodeSelectorRequirementOpt,
    ) -> Result<(), crate::optionable::Error> {
        if let Some(other_value) = other.key {
            <std::string::String as crate::OptionableConvert>::merge(
                &mut self.key,
                other_value,
            )?;
        }
        if let Some(other_value) = other.operator {
            <std::string::String as crate::OptionableConvert>::merge(
                &mut self.operator,
                other_value,
            )?;
        }
        <Option<
            std::vec::Vec<std::string::String>,
        > as crate::OptionableConvert>::merge(&mut self.values, other.values)?;
        Ok(())
    }
}

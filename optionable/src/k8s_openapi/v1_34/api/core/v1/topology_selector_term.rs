pub struct TopologySelectorTermOpt {
    pub match_label_expressions: <Option<
        std::vec::Vec<::k8s_openapi::api::core::v1::TopologySelectorLabelRequirement>,
    > as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable
for ::k8s_openapi::api::core::v1::topology_selector_term::TopologySelectorTerm {
    type Optioned = TopologySelectorTermOpt;
}
#[automatically_derived]
impl crate::Optionable for TopologySelectorTermOpt {
    type Optioned = TopologySelectorTermOpt;
}
#[automatically_derived]
impl crate::OptionableConvert
for ::k8s_openapi::api::core::v1::topology_selector_term::TopologySelectorTerm {
    fn into_optioned(self) -> TopologySelectorTermOpt {
        TopologySelectorTermOpt {
            match_label_expressions: <Option<
                std::vec::Vec<
                    ::k8s_openapi::api::core::v1::TopologySelectorLabelRequirement,
                >,
            > as crate::OptionableConvert>::into_optioned(self.match_label_expressions),
        }
    }
    fn try_from_optioned(
        value: TopologySelectorTermOpt,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            match_label_expressions: <Option<
                std::vec::Vec<
                    ::k8s_openapi::api::core::v1::TopologySelectorLabelRequirement,
                >,
            > as crate::OptionableConvert>::try_from_optioned(
                value.match_label_expressions,
            )?,
        })
    }
    fn merge(
        &mut self,
        other: TopologySelectorTermOpt,
    ) -> Result<(), crate::optionable::Error> {
        <Option<
            std::vec::Vec<::k8s_openapi::api::core::v1::TopologySelectorLabelRequirement>,
        > as crate::OptionableConvert>::merge(
            &mut self.match_label_expressions,
            other.match_label_expressions,
        )?;
        Ok(())
    }
}

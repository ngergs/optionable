#[derive(
    Clone,
    std::fmt::Debug,
    Default,
    PartialEq,
    serde::Serialize,
    serde::Deserialize
)]
#[serde(rename_all = "camelCase")]
pub struct TopologySelectorTermAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub match_label_expressions: <Option<
        std::vec::Vec<::k8s_openapi::api::core::v1::TopologySelectorLabelRequirement>,
    > as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::core::v1::TopologySelectorTerm {
    type Optioned = TopologySelectorTermAc;
}
#[automatically_derived]
impl crate::Optionable for TopologySelectorTermAc {
    type Optioned = TopologySelectorTermAc;
}
#[automatically_derived]
impl crate::OptionableConvert for ::k8s_openapi::api::core::v1::TopologySelectorTerm {
    fn into_optioned(self) -> TopologySelectorTermAc {
        TopologySelectorTermAc {
            match_label_expressions: crate::OptionableConvert::into_optioned(
                self.match_label_expressions,
            ),
        }
    }
    fn try_from_optioned(
        value: TopologySelectorTermAc,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            match_label_expressions: crate::OptionableConvert::try_from_optioned(
                value.match_label_expressions,
            )?,
        })
    }
    fn merge(
        &mut self,
        other: TopologySelectorTermAc,
    ) -> Result<(), crate::optionable::Error> {
        crate::OptionableConvert::merge(
            &mut self.match_label_expressions,
            other.match_label_expressions,
        )?;
        Ok(())
    }
}

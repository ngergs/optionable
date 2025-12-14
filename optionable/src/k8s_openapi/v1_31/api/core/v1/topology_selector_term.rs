#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
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
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert for ::k8s_openapi::api::core::v1::TopologySelectorTerm {
    fn into_optioned(self) -> TopologySelectorTermAc {
        TopologySelectorTermAc {
            match_label_expressions: crate::OptionableConvert::into_optioned(
                self.match_label_expressions,
            ),
        }
    }
    fn try_from_optioned(value: TopologySelectorTermAc) -> Result<Self, crate::Error> {
        Ok(Self {
            match_label_expressions: crate::OptionableConvert::try_from_optioned(
                value.match_label_expressions,
            )?,
        })
    }
    fn merge(&mut self, other: TopologySelectorTermAc) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(
            &mut self.match_label_expressions,
            other.match_label_expressions,
        )?;
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<::k8s_openapi::api::core::v1::TopologySelectorTerm>
for TopologySelectorTermAc {
    fn from_optionable(
        value: ::k8s_openapi::api::core::v1::TopologySelectorTerm,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<::k8s_openapi::api::core::v1::TopologySelectorTerm, crate::Error> {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut ::k8s_openapi::api::core::v1::TopologySelectorTerm,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}

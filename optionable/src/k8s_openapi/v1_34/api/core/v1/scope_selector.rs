pub struct ScopeSelectorAc {
    pub match_expressions: <Option<
        std::vec::Vec<::k8s_openapi::api::core::v1::ScopedResourceSelectorRequirement>,
    > as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::core::v1::ScopeSelector {
    type Optioned = ScopeSelectorAc;
}
#[automatically_derived]
impl crate::Optionable for ScopeSelectorAc {
    type Optioned = ScopeSelectorAc;
}
#[automatically_derived]
impl crate::OptionableConvert for ::k8s_openapi::api::core::v1::ScopeSelector {
    fn into_optioned(self) -> ScopeSelectorAc {
        ScopeSelectorAc {
            match_expressions: crate::OptionableConvert::into_optioned(
                self.match_expressions,
            ),
        }
    }
    fn try_from_optioned(
        value: ScopeSelectorAc,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            match_expressions: crate::OptionableConvert::try_from_optioned(
                value.match_expressions,
            )?,
        })
    }
    fn merge(&mut self, other: ScopeSelectorAc) -> Result<(), crate::optionable::Error> {
        crate::OptionableConvert::merge(
            &mut self.match_expressions,
            other.match_expressions,
        )?;
        Ok(())
    }
}

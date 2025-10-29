pub struct DownwardAPIProjectionOpt {
    pub items: <Option<
        std::vec::Vec<::k8s_openapi::api::core::v1::DownwardAPIVolumeFile>,
    > as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::core::v1::DownwardAPIProjection {
    type Optioned = DownwardAPIProjectionOpt;
}
#[automatically_derived]
impl crate::Optionable for DownwardAPIProjectionOpt {
    type Optioned = DownwardAPIProjectionOpt;
}
#[automatically_derived]
impl crate::OptionableConvert for ::k8s_openapi::api::core::v1::DownwardAPIProjection {
    fn into_optioned(self) -> DownwardAPIProjectionOpt {
        DownwardAPIProjectionOpt {
            items: crate::OptionableConvert::into_optioned(self.items),
        }
    }
    fn try_from_optioned(
        value: DownwardAPIProjectionOpt,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            items: crate::OptionableConvert::try_from_optioned(value.items)?,
        })
    }
    fn merge(
        &mut self,
        other: DownwardAPIProjectionOpt,
    ) -> Result<(), crate::optionable::Error> {
        crate::OptionableConvert::merge(&mut self.items, other.items)?;
        Ok(())
    }
}

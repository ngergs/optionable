pub struct DownwardAPIProjectionAc {
    pub items: <Option<
        std::vec::Vec<::k8s_openapi::api::core::v1::DownwardAPIVolumeFile>,
    > as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::core::v1::DownwardAPIProjection {
    type Optioned = DownwardAPIProjectionAc;
}
#[automatically_derived]
impl crate::Optionable for DownwardAPIProjectionAc {
    type Optioned = DownwardAPIProjectionAc;
}
#[automatically_derived]
impl crate::OptionableConvert for ::k8s_openapi::api::core::v1::DownwardAPIProjection {
    fn into_optioned(self) -> DownwardAPIProjectionAc {
        DownwardAPIProjectionAc {
            items: crate::OptionableConvert::into_optioned(self.items),
        }
    }
    fn try_from_optioned(
        value: DownwardAPIProjectionAc,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            items: crate::OptionableConvert::try_from_optioned(value.items)?,
        })
    }
    fn merge(
        &mut self,
        other: DownwardAPIProjectionAc,
    ) -> Result<(), crate::optionable::Error> {
        crate::OptionableConvert::merge(&mut self.items, other.items)?;
        Ok(())
    }
}

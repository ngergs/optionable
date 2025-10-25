pub struct IPAddressSpecOpt {
    pub parent_ref: Option<
        <::k8s_openapi::api::networking::v1::ParentReference as crate::Optionable>::Optioned,
    >,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::networking::v1::IPAddressSpec {
    type Optioned = IPAddressSpecOpt;
}
#[automatically_derived]
impl crate::Optionable for IPAddressSpecOpt {
    type Optioned = IPAddressSpecOpt;
}
#[automatically_derived]
impl crate::OptionableConvert for ::k8s_openapi::api::networking::v1::IPAddressSpec {
    fn into_optioned(self) -> IPAddressSpecOpt {
        IPAddressSpecOpt {
            parent_ref: Some(
                <::k8s_openapi::api::networking::v1::ParentReference as crate::OptionableConvert>::into_optioned(
                    self.parent_ref,
                ),
            ),
        }
    }
    fn try_from_optioned(
        value: IPAddressSpecOpt,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            parent_ref: <::k8s_openapi::api::networking::v1::ParentReference as crate::OptionableConvert>::try_from_optioned(
                value
                    .parent_ref
                    .ok_or(crate::optionable::Error {
                        missing_field: "parent_ref",
                    })?,
            )?,
        })
    }
    fn merge(
        &mut self,
        other: IPAddressSpecOpt,
    ) -> Result<(), crate::optionable::Error> {
        if let Some(other_value) = other.parent_ref {
            <::k8s_openapi::api::networking::v1::ParentReference as crate::OptionableConvert>::merge(
                &mut self.parent_ref,
                other_value,
            )?;
        }
        Ok(())
    }
}

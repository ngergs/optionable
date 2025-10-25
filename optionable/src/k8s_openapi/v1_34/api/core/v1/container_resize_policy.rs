pub struct ContainerResizePolicyOpt {
    pub resource_name: Option<<std::string::String as crate::Optionable>::Optioned>,
    pub restart_policy: Option<<std::string::String as crate::Optionable>::Optioned>,
}
#[automatically_derived]
impl crate::Optionable
for ::k8s_openapi::api::core::v1::container_resize_policy::ContainerResizePolicy {
    type Optioned = ContainerResizePolicyOpt;
}
#[automatically_derived]
impl crate::Optionable for ContainerResizePolicyOpt {
    type Optioned = ContainerResizePolicyOpt;
}
#[automatically_derived]
impl crate::OptionableConvert
for ::k8s_openapi::api::core::v1::container_resize_policy::ContainerResizePolicy {
    fn into_optioned(self) -> ContainerResizePolicyOpt {
        ContainerResizePolicyOpt {
            resource_name: Some(
                <std::string::String as crate::OptionableConvert>::into_optioned(
                    self.resource_name,
                ),
            ),
            restart_policy: Some(
                <std::string::String as crate::OptionableConvert>::into_optioned(
                    self.restart_policy,
                ),
            ),
        }
    }
    fn try_from_optioned(
        value: ContainerResizePolicyOpt,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            resource_name: <std::string::String as crate::OptionableConvert>::try_from_optioned(
                value
                    .resource_name
                    .ok_or(crate::optionable::Error {
                        missing_field: "resource_name",
                    })?,
            )?,
            restart_policy: <std::string::String as crate::OptionableConvert>::try_from_optioned(
                value
                    .restart_policy
                    .ok_or(crate::optionable::Error {
                        missing_field: "restart_policy",
                    })?,
            )?,
        })
    }
    fn merge(
        &mut self,
        other: ContainerResizePolicyOpt,
    ) -> Result<(), crate::optionable::Error> {
        if let Some(other_value) = other.resource_name {
            <std::string::String as crate::OptionableConvert>::merge(
                &mut self.resource_name,
                other_value,
            )?;
        }
        if let Some(other_value) = other.restart_policy {
            <std::string::String as crate::OptionableConvert>::merge(
                &mut self.restart_policy,
                other_value,
            )?;
        }
        Ok(())
    }
}

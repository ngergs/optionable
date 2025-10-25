pub struct MutatingWebhookOpt {
    pub admission_review_versions: Option<
        <std::vec::Vec<std::string::String> as crate::Optionable>::Optioned,
    >,
    pub client_config: Option<
        <::k8s_openapi::api::admissionregistration::v1::WebhookClientConfig as crate::Optionable>::Optioned,
    >,
    pub failure_policy: <Option<std::string::String> as crate::Optionable>::Optioned,
    pub match_conditions: <Option<
        std::vec::Vec<::k8s_openapi::api::admissionregistration::v1::MatchCondition>,
    > as crate::Optionable>::Optioned,
    pub match_policy: <Option<std::string::String> as crate::Optionable>::Optioned,
    pub name: Option<<std::string::String as crate::Optionable>::Optioned>,
    pub namespace_selector: <Option<
        ::k8s_openapi::apimachinery::pkg::apis::meta::v1::LabelSelector,
    > as crate::Optionable>::Optioned,
    pub object_selector: <Option<
        ::k8s_openapi::apimachinery::pkg::apis::meta::v1::LabelSelector,
    > as crate::Optionable>::Optioned,
    pub reinvocation_policy: <Option<
        std::string::String,
    > as crate::Optionable>::Optioned,
    pub rules: <Option<
        std::vec::Vec<::k8s_openapi::api::admissionregistration::v1::RuleWithOperations>,
    > as crate::Optionable>::Optioned,
    pub side_effects: Option<<std::string::String as crate::Optionable>::Optioned>,
    pub timeout_seconds: <Option<i32> as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable
for ::k8s_openapi::api::admissionregistration::v1::MutatingWebhook {
    type Optioned = MutatingWebhookOpt;
}
#[automatically_derived]
impl crate::Optionable for MutatingWebhookOpt {
    type Optioned = MutatingWebhookOpt;
}
#[automatically_derived]
impl crate::OptionableConvert
for ::k8s_openapi::api::admissionregistration::v1::MutatingWebhook {
    fn into_optioned(self) -> MutatingWebhookOpt {
        MutatingWebhookOpt {
            admission_review_versions: Some(
                <std::vec::Vec<
                    std::string::String,
                > as crate::OptionableConvert>::into_optioned(
                    self.admission_review_versions,
                ),
            ),
            client_config: Some(
                <::k8s_openapi::api::admissionregistration::v1::WebhookClientConfig as crate::OptionableConvert>::into_optioned(
                    self.client_config,
                ),
            ),
            failure_policy: <Option<
                std::string::String,
            > as crate::OptionableConvert>::into_optioned(self.failure_policy),
            match_conditions: <Option<
                std::vec::Vec<
                    ::k8s_openapi::api::admissionregistration::v1::MatchCondition,
                >,
            > as crate::OptionableConvert>::into_optioned(self.match_conditions),
            match_policy: <Option<
                std::string::String,
            > as crate::OptionableConvert>::into_optioned(self.match_policy),
            name: Some(
                <std::string::String as crate::OptionableConvert>::into_optioned(
                    self.name,
                ),
            ),
            namespace_selector: <Option<
                ::k8s_openapi::apimachinery::pkg::apis::meta::v1::LabelSelector,
            > as crate::OptionableConvert>::into_optioned(self.namespace_selector),
            object_selector: <Option<
                ::k8s_openapi::apimachinery::pkg::apis::meta::v1::LabelSelector,
            > as crate::OptionableConvert>::into_optioned(self.object_selector),
            reinvocation_policy: <Option<
                std::string::String,
            > as crate::OptionableConvert>::into_optioned(self.reinvocation_policy),
            rules: <Option<
                std::vec::Vec<
                    ::k8s_openapi::api::admissionregistration::v1::RuleWithOperations,
                >,
            > as crate::OptionableConvert>::into_optioned(self.rules),
            side_effects: Some(
                <std::string::String as crate::OptionableConvert>::into_optioned(
                    self.side_effects,
                ),
            ),
            timeout_seconds: <Option<
                i32,
            > as crate::OptionableConvert>::into_optioned(self.timeout_seconds),
        }
    }
    fn try_from_optioned(
        value: MutatingWebhookOpt,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            admission_review_versions: <std::vec::Vec<
                std::string::String,
            > as crate::OptionableConvert>::try_from_optioned(
                value
                    .admission_review_versions
                    .ok_or(crate::optionable::Error {
                        missing_field: "admission_review_versions",
                    })?,
            )?,
            client_config: <::k8s_openapi::api::admissionregistration::v1::WebhookClientConfig as crate::OptionableConvert>::try_from_optioned(
                value
                    .client_config
                    .ok_or(crate::optionable::Error {
                        missing_field: "client_config",
                    })?,
            )?,
            failure_policy: <Option<
                std::string::String,
            > as crate::OptionableConvert>::try_from_optioned(value.failure_policy)?,
            match_conditions: <Option<
                std::vec::Vec<
                    ::k8s_openapi::api::admissionregistration::v1::MatchCondition,
                >,
            > as crate::OptionableConvert>::try_from_optioned(value.match_conditions)?,
            match_policy: <Option<
                std::string::String,
            > as crate::OptionableConvert>::try_from_optioned(value.match_policy)?,
            name: <std::string::String as crate::OptionableConvert>::try_from_optioned(
                value
                    .name
                    .ok_or(crate::optionable::Error {
                        missing_field: "name",
                    })?,
            )?,
            namespace_selector: <Option<
                ::k8s_openapi::apimachinery::pkg::apis::meta::v1::LabelSelector,
            > as crate::OptionableConvert>::try_from_optioned(value.namespace_selector)?,
            object_selector: <Option<
                ::k8s_openapi::apimachinery::pkg::apis::meta::v1::LabelSelector,
            > as crate::OptionableConvert>::try_from_optioned(value.object_selector)?,
            reinvocation_policy: <Option<
                std::string::String,
            > as crate::OptionableConvert>::try_from_optioned(
                value.reinvocation_policy,
            )?,
            rules: <Option<
                std::vec::Vec<
                    ::k8s_openapi::api::admissionregistration::v1::RuleWithOperations,
                >,
            > as crate::OptionableConvert>::try_from_optioned(value.rules)?,
            side_effects: <std::string::String as crate::OptionableConvert>::try_from_optioned(
                value
                    .side_effects
                    .ok_or(crate::optionable::Error {
                        missing_field: "side_effects",
                    })?,
            )?,
            timeout_seconds: <Option<
                i32,
            > as crate::OptionableConvert>::try_from_optioned(value.timeout_seconds)?,
        })
    }
    fn merge(
        &mut self,
        other: MutatingWebhookOpt,
    ) -> Result<(), crate::optionable::Error> {
        if let Some(other_value) = other.admission_review_versions {
            <std::vec::Vec<
                std::string::String,
            > as crate::OptionableConvert>::merge(
                &mut self.admission_review_versions,
                other_value,
            )?;
        }
        if let Some(other_value) = other.client_config {
            <::k8s_openapi::api::admissionregistration::v1::WebhookClientConfig as crate::OptionableConvert>::merge(
                &mut self.client_config,
                other_value,
            )?;
        }
        <Option<
            std::string::String,
        > as crate::OptionableConvert>::merge(
            &mut self.failure_policy,
            other.failure_policy,
        )?;
        <Option<
            std::vec::Vec<::k8s_openapi::api::admissionregistration::v1::MatchCondition>,
        > as crate::OptionableConvert>::merge(
            &mut self.match_conditions,
            other.match_conditions,
        )?;
        <Option<
            std::string::String,
        > as crate::OptionableConvert>::merge(
            &mut self.match_policy,
            other.match_policy,
        )?;
        if let Some(other_value) = other.name {
            <std::string::String as crate::OptionableConvert>::merge(
                &mut self.name,
                other_value,
            )?;
        }
        <Option<
            ::k8s_openapi::apimachinery::pkg::apis::meta::v1::LabelSelector,
        > as crate::OptionableConvert>::merge(
            &mut self.namespace_selector,
            other.namespace_selector,
        )?;
        <Option<
            ::k8s_openapi::apimachinery::pkg::apis::meta::v1::LabelSelector,
        > as crate::OptionableConvert>::merge(
            &mut self.object_selector,
            other.object_selector,
        )?;
        <Option<
            std::string::String,
        > as crate::OptionableConvert>::merge(
            &mut self.reinvocation_policy,
            other.reinvocation_policy,
        )?;
        <Option<
            std::vec::Vec<
                ::k8s_openapi::api::admissionregistration::v1::RuleWithOperations,
            >,
        > as crate::OptionableConvert>::merge(&mut self.rules, other.rules)?;
        if let Some(other_value) = other.side_effects {
            <std::string::String as crate::OptionableConvert>::merge(
                &mut self.side_effects,
                other_value,
            )?;
        }
        <Option<
            i32,
        > as crate::OptionableConvert>::merge(
            &mut self.timeout_seconds,
            other.timeout_seconds,
        )?;
        Ok(())
    }
}

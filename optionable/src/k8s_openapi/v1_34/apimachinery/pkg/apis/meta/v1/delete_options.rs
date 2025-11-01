pub struct DeleteOptionsAc {
    pub api_version: <Option<std::string::String> as crate::Optionable>::Optioned,
    pub dry_run: <Option<
        std::vec::Vec<std::string::String>,
    > as crate::Optionable>::Optioned,
    pub grace_period_seconds: <Option<i64> as crate::Optionable>::Optioned,
    pub ignore_store_read_error_with_cluster_breaking_potential: <Option<
        bool,
    > as crate::Optionable>::Optioned,
    pub kind: <Option<std::string::String> as crate::Optionable>::Optioned,
    pub orphan_dependents: <Option<bool> as crate::Optionable>::Optioned,
    pub preconditions: <Option<
        ::k8s_openapi::apimachinery::pkg::apis::meta::v1::Preconditions,
    > as crate::Optionable>::Optioned,
    pub propagation_policy: <Option<std::string::String> as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable
for ::k8s_openapi::apimachinery::pkg::apis::meta::v1::DeleteOptions {
    type Optioned = DeleteOptionsAc;
}
#[automatically_derived]
impl crate::Optionable for DeleteOptionsAc {
    type Optioned = DeleteOptionsAc;
}
#[automatically_derived]
impl crate::OptionableConvert
for ::k8s_openapi::apimachinery::pkg::apis::meta::v1::DeleteOptions {
    fn into_optioned(self) -> DeleteOptionsAc {
        DeleteOptionsAc {
            api_version: crate::OptionableConvert::into_optioned(self.api_version),
            dry_run: crate::OptionableConvert::into_optioned(self.dry_run),
            grace_period_seconds: crate::OptionableConvert::into_optioned(
                self.grace_period_seconds,
            ),
            ignore_store_read_error_with_cluster_breaking_potential: crate::OptionableConvert::into_optioned(
                self.ignore_store_read_error_with_cluster_breaking_potential,
            ),
            kind: crate::OptionableConvert::into_optioned(self.kind),
            orphan_dependents: crate::OptionableConvert::into_optioned(
                self.orphan_dependents,
            ),
            preconditions: crate::OptionableConvert::into_optioned(self.preconditions),
            propagation_policy: crate::OptionableConvert::into_optioned(
                self.propagation_policy,
            ),
        }
    }
    fn try_from_optioned(
        value: DeleteOptionsAc,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            api_version: crate::OptionableConvert::try_from_optioned(value.api_version)?,
            dry_run: crate::OptionableConvert::try_from_optioned(value.dry_run)?,
            grace_period_seconds: crate::OptionableConvert::try_from_optioned(
                value.grace_period_seconds,
            )?,
            ignore_store_read_error_with_cluster_breaking_potential: crate::OptionableConvert::try_from_optioned(
                value.ignore_store_read_error_with_cluster_breaking_potential,
            )?,
            kind: crate::OptionableConvert::try_from_optioned(value.kind)?,
            orphan_dependents: crate::OptionableConvert::try_from_optioned(
                value.orphan_dependents,
            )?,
            preconditions: crate::OptionableConvert::try_from_optioned(
                value.preconditions,
            )?,
            propagation_policy: crate::OptionableConvert::try_from_optioned(
                value.propagation_policy,
            )?,
        })
    }
    fn merge(&mut self, other: DeleteOptionsAc) -> Result<(), crate::optionable::Error> {
        crate::OptionableConvert::merge(&mut self.api_version, other.api_version)?;
        crate::OptionableConvert::merge(&mut self.dry_run, other.dry_run)?;
        crate::OptionableConvert::merge(
            &mut self.grace_period_seconds,
            other.grace_period_seconds,
        )?;
        crate::OptionableConvert::merge(
            &mut self.ignore_store_read_error_with_cluster_breaking_potential,
            other.ignore_store_read_error_with_cluster_breaking_potential,
        )?;
        crate::OptionableConvert::merge(&mut self.kind, other.kind)?;
        crate::OptionableConvert::merge(
            &mut self.orphan_dependents,
            other.orphan_dependents,
        )?;
        crate::OptionableConvert::merge(&mut self.preconditions, other.preconditions)?;
        crate::OptionableConvert::merge(
            &mut self.propagation_policy,
            other.propagation_policy,
        )?;
        Ok(())
    }
}

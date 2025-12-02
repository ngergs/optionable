## Circumventing the orphanrule

Example how the `#[optionable(optioned_type=...)]` field helper attribute can be used to derive `Optionable`
for structs where certain fields are not `required` and also do not implement `Optionable` themselves.
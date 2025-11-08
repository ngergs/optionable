# Server side apply example

One main motivation for the `optionable` crate is to support type-safe server-side apply for Kubernetes in Rust.

## Deployment

For the source code see [src/bin/apply_deployment.rs](src/bin/apply_deployment.rs).

It is possible to express the intent to e.g. only patch the `spec.replica` count of a `Deployment` as:
```rust
let patch = DeploymentAc {
    metadata: ObjectMeta {
    name: Some("test".to_owned()),
        ..Default::default()
    },
    spec: Some(DeploymentSpecAc {
        replicas: Some(2),
        ..Default::default()
    }),
    ..Default::default()
};
```

The following script deploys an example `Deployment` to the current configured Kubernetes cluster and subsequently uses
the [rust server-side apply example](src/bin/apply_deployment.rs) to patch the replica count.

```bash
# prepare deployment
kubectl apply -f deployment.yaml
# run server side apply to change replica count to 2
cargo run --bin apply_deployment
# verify that server side apply has been used
kubectl get deployment test -oyaml --show-managed-fields=true | yq .metadata.managedFields
```

The resulting output will be:
```yaml
- apiVersion: apps/v1
  fieldsType: FieldsV1
  fieldsV1:
    f:spec:
      f:replicas: {}
  manager: rust-manager
  operation: Apply
- <other entries from kubectl and other manager interacting with the deployment>
```

## CRDs

For the source code see [src/lib.rs](src/lib.rs) for the CRD definition and [src/bin/apply_crd.rs](src/bin/apply_crd.rs) for
the optioned type example.

The most relevant helper attribute macros to derive optioned types for `kube::CustomResource` are [`#[optionable_kube_cr]`](https://docs.rs/optionable/latest/optionable/kube/attr.optionable_kube_cr.html) 
for the type with the `#[derive(CustomResource)]` derive macro and [`#[optionable_kube]`](https://docs.rs/optionable/latest/optionable/kube/attr.optionable_kube.html) for general types like nested subfields.

Also, here we can express the granular intent to only patch `spec.template.replicas` while preserving type-safety.
```rust
let patch = MyCrdAc {
    metadata: ObjectMeta {
        name: Some("test".to_owned()),
        ..Default::default()
    },
    spec: Some(MyCrdSpecAc {
        template: Some(MyCrdSpecTemplateAc { replicas: Some(2) }),
        ..Default::default()
    }),
..Default::default()
};
```

The following script deploys the example `CRD` and an example `CR` to the current configured Kubernetes cluster and subsequently uses
the [rust server-side apply example](src/bin/apply_crd.rs) to patch the `spec.template.replicas` count.
```bash
# setup crd
cargo run --bin crd | kubectl apply -f -
# prepare deployment
kubectl apply -f cr.yaml
# run server side apply to change replica count to 2
cargo run --bin apply_crd
# verify that server side apply has been used
kubectl get mycrd test -oyaml --show-managed-fields=true | yq .metadata.managedFields
```

The resulting output will be:
```yaml
- apiVersion: example.localhost/v1
  fieldsType: FieldsV1
  fieldsV1:
    f:spec:
      f:template:
        f:replicas: {}
  manager: rust-manager
  operation: Apply
- apiVersion: example.localhost/v1
  fieldsType: FieldsV1
  fieldsV1:
    f:metadata:
      f:annotations:
        .: {}
        f:kubectl.kubernetes.io/last-applied-configuration: {}
    f:spec:
      .: {}
      f:msg: {}
      f:template: {}
  manager: kubectl-client-side-apply
  operation: Update
```
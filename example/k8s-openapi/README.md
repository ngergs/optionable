# Server side apply example

One main motivation for the `optionable` crate is to support type-safe server-side apply for Kubernetes in Rust. 
It is possible to express the intend to e.g. only patch the `spec.replica` count of a `Deployment` as:
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

## Deployment

For the source code see [src/bin/apply_deployment.rs](src/bin/apply_deployment.rs).

A very simple example for how the `optionable`-`k8s-openapi` types can be used to implement
Kubernetes server-side-apply.

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

For the source code see [src/lib.rs](src/lib.rs) for the CRD definition and [src/apply_crd.rs](src/apply_crd.rs) for
the optioned type example.

Another simple example for how the `optionable` Kubernetes optimized derives can be used to implement
Kubernetes server-side-apply for CRDs.

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
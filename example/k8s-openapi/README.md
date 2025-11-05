# Server side apply example

## Deployment

A very simple for how the `optionable`-`k8s-openapi` types can be used to implement
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

## CRD

How optioned types can be generated for `kube::CustomResources`.

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
# Server side apply example

A very simple for how the `optionable`-`k8s-openapi` types can be used to implement
Kubernetes server-side-apply.

```bash
# prepare deployment
kubectl apply -f deployment.yaml
# run server side apply to change replica count to 2
cargo run
# verify that server side apply has been used
kubectl get deployment test -oyaml --show-managed-fields=true | yq .metadata.mana
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
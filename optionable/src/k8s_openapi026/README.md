# Codegen

The codegen generates optioned types and implements the `Optionable`-trait for types from [`k8s-openapi`](https://github.com/Arnavion/k8s-openapi).

To regenerate the files run:
```bash
./codegen.sh <path to git root of the cloned k8s-openapi repo>
```

## Limitations

[WatchEvent](https://kubernetes.io/docs/reference/generated/kubernetes-api/v1.34/#watchevent-v1-meta) is a very special 
case as it is an [externally tagged enum](https://github.com/Arnavion/k8s-openapi/blob/master/src/v1_34/apimachinery/pkg/apis/meta/v1/watch_event.rs)
where two variants share a tag `ERROR`. There is no obvious way how to model this without using a custom serialization/deserialization.
As a WatchEvent is usually only consumed and not patched there is likely not a use case for having
an optioned WatchEvent so it is dropped for now.
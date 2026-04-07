use k8s_openapi_codegen_common::get_rust_ident;
use openapiv3::{OpenAPI, ReferenceOr, Schema, SchemaKind, Type};
use std::cmp::PartialEq;
use std::{
    collections::HashMap,
    fs::{self, File},
    path::PathBuf,
};

/// Kubernetes server-side apply list type, see <https://kubernetes.io/docs/reference/using-api/server-side-apply>/
#[derive(PartialEq, Debug)]
pub enum ListType {
    Atomic,
    Set,
    /// Contains the list-map keys as value
    Map(Vec<String>),
}

/// Determines the `x-kubernetes-list-type` and for `map` the `x-kubernetes-list-map-keys` for each type from the Kubernetes Openapi v3 specification.
/// Skips embedded type definitions (only reference types are followed).
/// The input directory should point to this: <https://github.com/kubernetes/kubernetes/tree/master/api/openapi-spec/v3>.
/// The keys of the map are the kubernetes type identifier with `io.k8s.`-prefix removed that they all share,
/// so e.g. `api.core.v1.Container`. The values for `ListType::Map` are the rustified (k8s openapi mapping) rust field names.
pub fn determine_list_map_keys(
    k8s_openapi_v3_dir: &PathBuf,
) -> Result<HashMap<String, ListType>, Box<dyn std::error::Error>> {
    let mut result = HashMap::new();
    let schema_files = fs::read_dir(k8s_openapi_v3_dir)?;
    for schema_file in schema_files {
        let schema_file = schema_file?;
        let openapi: OpenAPI = serde_json::from_reader(File::open(schema_file.path())?)?;

        if let Some(components) = openapi.components {
            for (name, schema) in components.schemas {
                process_schema(&mut result, &[&name], schema.into_item().as_ref())?;
            }
        }
    }
    Ok(result)
}

/// Parses the schema recursively and extract the `list_map` behavior into the result `HashMap`.
fn process_schema(
    result: &mut HashMap<String, ListType>,
    field_path: &[&str],
    schema: Option<&Schema>,
) -> Result<(), Box<dyn std::error::Error>> {
    if let Some(schema) = schema
        && let SchemaKind::Type(item) = &schema.schema_kind
    {
        let list_type=schema.schema_data.extensions.get("x-kubernetes-list-type").map(|ty|{
        let serde_json::Value::String(ty)=ty else {
                return Err(format!("found `x-kubernetes-list-type` extension but without a string value: {schema:?}"));
            };
             match ty.as_ref() {
                "atomic" => Ok(ListType::Atomic),
                "set" => Ok(ListType::Set),
                "map" => {
                    let map_keys=schema.schema_data.extensions.get("x-kubernetes-list-map-keys").ok_or_else(||format!("found `x-kubernetes-list-type` extension but without a string value: {schema:?}"))?;
                    let serde_json::Value::Array(map_keys)=map_keys else{
                        return Err(format!("Extension `x-kubernetes-list-map-keys` does not has as value an array of strings: {schema:?}"));
                    };
                     let map_keys=   map_keys
                            .iter()
                            .map(|v| {
                                if let serde_json::Value::String(s) = v {
                                    Ok(get_rust_ident(s).into_owned())
                                } else {
                                    Err(format!(
                                        "Unsupported `x-kubernetes-list-map-keys` value: {v:?}"
                                    ))
                                }
                            })
                            .collect::<Result<Vec<_>, _>>();
                    map_keys.map(ListType::Map)
                },
                _ => Err(format!("Unsupported `x-kubernetes-list-type`: {ty}")),
        }}).transpose()?;
        // save found map type, ignore explicitly embedded types
        if let Some(list_type) = list_type
            && let Type::Array(array) = &item
            && let Some(array_schema) = &array.items
            && let Some(array_schema) = array_schema.clone().as_item()
            && let SchemaKind::AllOf { all_of } = &array_schema.schema_kind
            && all_of.len() == 1
            && let Some(ReferenceOr::Reference { reference }) = all_of.first()
            && let Some(reference) = reference.strip_prefix("#/components/schemas/io.k8s.")
        {
            let field_path_joined = field_path.join(".");
            if let Some(existing_list_type) = result.get(&field_path_joined)
                && *existing_list_type != list_type
            {
                return Err(format!(
                    "Conflicting map keys for `{reference}`: field={field_path:?}, existing={existing_list_type:?}, new={list_type:?}"
                )
                .into());
            }
            result.insert(field_path_joined, list_type);
        }
        // continue recursion
        match item {
            Type::Object(obj) => {
                for (name, obj_schema) in &obj.properties {
                    let mut field_path = field_path.to_vec();
                    field_path.push(name);
                    process_schema(
                        result,
                        &field_path,
                        obj_schema.clone().into_item().as_deref(),
                    )?;
                }
            }
            Type::Array(array) => {
                // not needed at the time of writing but omitting arrays silently is not great
                if let Some(array_schema) = &array.items {
                    let mut field_path = field_path.to_vec();
                    field_path.push("[]");
                    process_schema(
                        result,
                        &field_path,
                        array_schema.clone().into_item().as_deref(),
                    )?;
                }
            }
            _ => {}
        }
    }
    Ok(())
}

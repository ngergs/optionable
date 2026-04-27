use k8s_openapi_codegen_common::get_rust_ident;
use openapiv3::{OpenAPI, ReferenceOr, Schema, SchemaKind, Type};
use std::cmp::PartialEq;
use std::{
    collections::HashMap,
    fs::{self, File},
    path::PathBuf,
};

/// Kubernetes server-side apply list type, see <https://kubernetes.io/docs/reference/using-api/server-side-apply>/
#[derive(PartialEq, Clone, Debug)]
pub enum ListType {
    Atomic,
    Set,
    /// Contains the list-map keys as value
    Map(Vec<String>),
}

/// Kubernetes server-side apply map type, see <https://kubernetes.io/docs/reference/using-api/server-side-apply>/
#[derive(PartialEq, Clone, Debug)]
pub enum MapType {
    Atomic,
    Granular,
}

/// Mapping obtained from the upstream Kubernetes `OpenAPIv3` specifications, atm only for list merge types.
#[derive(Default)]
pub struct OpenApiListExtensions {
    // Field identifier (e.g. `api.core.v1.Container.Env`) to ListType
    pub map_type: HashMap<String, MapType>,
    // Field identifier (e.g. `api.core.v1.Container.Env`) to ListType
    pub list_type: HashMap<String, ListType>,
    // Struct identifier (e.g. `api.core.v1.EnvVar`) to list keys if it's used somewhere with list type `Map`.
    pub list_map_keys: HashMap<String, Vec<String>>,
}

/// Determines the `x-kubernetes-list-type` and for `map` the `x-kubernetes-list-map-keys` for each type from the Kubernetes Openapi v3 specification.
/// Skips embedded type definitions (only reference types are followed).
/// The input directory should point to this: <https://github.com/kubernetes/kubernetes/tree/master/api/openapi-spec/v3>.
/// The keys of the map are the kubernetes type identifier with `io.k8s.`-prefix removed that they all share,
/// so e.g. `api.core.v1.Container`. The values for `ListType::Map` are the rustified (k8s openapi mapping) rust field names.
///
/// # Errors
/// - Reading the file info for the schema input directory fails
/// - Reading a input schema file from the filesystem fails
/// - JSON parsing the `OpenAPIv3` spec fails
/// - `OpenAPIv3` parsing the read spec fails
/// - Unsupported values for Kubernetes schema extensions `x-kubernetes-list-type`,`x-kubernetes-map-type`, `x-kubernetes-list-keys` are encountered
/// - These are used in an unsupported schema constellation
/// - Conflicting values for these are aggregated from the schema for some Kubernetes types/fields.
pub fn determine_list_map_keys(
    k8s_openapi_v3_dir: &PathBuf,
) -> Result<OpenApiListExtensions, Box<dyn std::error::Error>> {
    let mut result = OpenApiListExtensions::default();
    let schema_files = fs::read_dir(k8s_openapi_v3_dir)?;
    for schema_file in schema_files {
        let schema_file = schema_file?;
        let openapi: OpenAPI = serde_json::from_reader(File::open(schema_file.path())?)?;

        if let Some(components) = openapi.components {
            for (name, schema) in components.schemas {
                let schema = schema.into_item();
                process_schema(&mut result, &[&name], schema.as_ref(), false)?;
                process_schema(&mut result, &[&name], schema.as_ref(), true)?;
            }
        }
    }
    Ok(result)
}

/// Parses the schema recursively and extract the `list_map` behavior into the result `HashMap`.
#[allow(clippy::too_many_lines)]
fn process_schema(
    result: &mut OpenApiListExtensions,
    field_path: &[&str],
    schema: Option<&Schema>,
    second_pass: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    let Some(schema) = schema else { return Ok(()) };
    let map_type = schema
            .schema_data
            .extensions
            .get("x-kubernetes-map-type")
            .map(|ty| {
                let serde_json::Value::String(ty)=ty else {
                    return Err(format!("found `x-kubernetes-list-type` extension but without a string value: {schema:?}"));
                };
                match ty.as_ref() {
                    "atomic" => Ok(MapType::Atomic),
                    "granular" => Ok(MapType::Granular),
                    _ => Err(format!("Unsupported `x-kubernetes-map-type`: {ty}")),
                }
            }).transpose()?;
    if let Some(map_type) = map_type {
        insert_err_on_conflict(&mut result.map_type, map_type, field_path)?;
    }

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
    if let SchemaKind::Type(item) = &schema.schema_kind {
        if let Some(list_type) = &list_type {
            // save found list type, ignore explicitly embedded types
            // only supported way to determine the list key type
            if let ListType::Map(list_keys) = &list_type {
                if let Type::Array(array) = &item
                    && let Some(array_schema) = &array.items
                    && let Some(array_schema) = array_schema.clone().as_item()
                    && let SchemaKind::AllOf { all_of } = &array_schema.schema_kind
                    && all_of.len() == 1
                    && let Some(ReferenceOr::Reference { reference }) = all_of.first()
                    && let Some(reference) = reference.strip_prefix("#/components/schemas/io.k8s.")
                {
                    if let Some(existing_list_type) = result.list_map_keys.get(reference)
                        && existing_list_type != list_keys
                    {
                        return Err(format!(
                    "Conflicting map keys for `{reference}`: field={field_path:?}, existing={existing_list_type:?}, new={list_type:?}"
                )
                .into());
                    }
                    result
                        .list_map_keys
                        .insert(reference.to_owned(), list_keys.clone());
                } else {
                    return Err(format!("Unsupported list mapping for {field_path:?}").into());
                }
            }
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
                        second_pass,
                    )?;
                }
            }
            Type::Array(array) => {
                // not needed at the time of writing but omitting arrays silently is not great
                if let Some(array_schema) = &array.items {
                    let field_path = field_path.to_vec();

                    process_schema(
                        result,
                        &field_path,
                        array_schema.clone().into_item().as_deref(),
                        second_pass,
                    )?;
                }
            }
            _ => {}
        }
    }
    if let Some(list_type) = list_type {
        insert_err_on_conflict(&mut result.list_type, list_type, field_path)?;
    }

    // only supported way to determine to resolve references
    if second_pass
        && let SchemaKind::AllOf { all_of } = &schema.schema_kind
        && all_of.len() == 1
        && let Some(ReferenceOr::Reference { reference }) = all_of.first()
        && let Some(reference) = reference.strip_prefix("#/components/schemas/io.k8s.")
    {
        // todo: also just references can specify map and list-types
        // schema.schema_data.extensions

        // The Kubernetes OpenAPI spec sometimes (e.g. for `claimRef` from `PersistentVolumeSpec` has map type extensions
        // for a given field with a different map type than the referenced type. The field ones (already handled on second pass)
        // take precendence.
        if let Some(map_type) = result.map_type.get(reference).cloned() {
            insert_if_not_present(&mut result.map_type, map_type, field_path)?;
        }
        if let Some(list_type) = result.list_type.get(reference).cloned() {
            insert_if_not_present(&mut result.list_type, list_type, field_path)?;
        }
    }
    Ok(())
}

/// Computes the map key as a Kubernetes style type identifier, e.g. `api.core.v1.Container`.
/// Inserts it in the `target` after verifying that there is no entry with different value already present.
fn insert_err_on_conflict<V: PartialEq + std::fmt::Debug>(
    target: &mut HashMap<String, V>,
    value: V,
    field_path: &[&str],
) -> Result<(), Box<dyn std::error::Error>> {
    let field_path_id = field_path_identifier(field_path, &value)?;
    if let Some(existing_entry) = target.get(&field_path_id) {
        if existing_entry != &value {
            return Err(format!(
                    "Conflicting map keys for field={field_path:?}: existing={existing_entry:?}, new={value:?}"
                )
                .into());
        }
    } else {
        target.insert(field_path_id, value);
    }
    Ok(())
}

/// Computes the map key as a Kubernetes style type identifier, e.g. `api.core.v1.Container`.
/// Inserts it in the `target` if it does not exist yet.
fn insert_if_not_present<V: PartialEq + std::fmt::Debug>(
    target: &mut HashMap<String, V>,
    value: V,
    field_path: &[&str],
) -> Result<(), Box<dyn std::error::Error>> {
    let field_path_id = field_path_identifier(field_path, &value)?;
    target.entry(field_path_id).or_insert(value);
    Ok(())
}

/// Computes the identifier for the given field path.  Basically the dot notated field path without the `io.k8s.`-prefix
/// and with the field name rustified, e.g. `api.core.v1.PersistenVolumeSpec.claim_ref`
fn field_path_identifier<V: std::fmt::Debug>(
    field_path: &[&str],
    value: &V,
) -> Result<String, Box<dyn std::error::Error>> {
    if field_path.is_empty() {
        return Err(format!("received empty field path for storage of value: {value:?}").into());
    }
    let field_path_joined = if field_path.len() == 1 {
        field_path[0].to_owned()
    } else {
        let mut result = field_path[..field_path.len() - 1].join(".");
        result.push('.');
        let rust_ident = get_rust_ident(field_path.last().unwrap()).into_owned();
        result.push_str(&rust_ident);
        result
    };
    let field_path_no_dot = field_path_joined.strip_prefix(".");
    let field_path_joined = field_path_no_dot.unwrap_or(&field_path_joined);
    let Some(field_path_joined) = field_path_joined.strip_prefix("io.k8s.") else {
        return Err(format!(
            "all fields path have to start with `io.k8s.`, found {field_path_joined:?}"
        )
        .into());
    };
    Ok(field_path_joined.to_owned())
}

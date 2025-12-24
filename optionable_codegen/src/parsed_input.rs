use crate::helper::{error, is_option, type_path_replace_crate};
use crate::parsed_input::FieldHandling::{IsOption, ManualOptioned, Other, Required};
use crate::FieldHelperAttributes;
use darling::FromAttributes;
use proc_macro2::Ident;
use std::iter;
use syn::{Error, Field, Fields, Path};

/// How we handle different cases in order of importance/detection.
/// E.g. if a field is `Required` we don't care whether it's of `Option` type or not.
#[derive(Debug, Clone)]
#[allow(dead_code)] // used by k8s_openapi feature and splitting the enum over this is not worth it
pub(crate) enum FieldHandling {
    Required,
    ManualOptioned(Path),
    IsOption,
    OptionedOnly,
    Other,
}

/// Type of the Struct we handle
#[derive(Debug, PartialEq)]
pub(crate) enum StructType {
    Named,
    Unnamed,
    Unit,
}

/// The field and the given case we detected for handling it.
#[derive(Debug, Clone)]
pub(crate) struct FieldParsed {
    pub field: Field,
    pub handling: FieldHandling,
}

/// Fields with extracted data how we want to handle them and most relevant struct/enum associated
/// data
#[derive(Debug)]
pub(crate) struct StructParsed {
    pub crate_name: Path,
    pub struct_type: StructType,
    pub fields: Vec<FieldParsed>,
}

/// Extracts information about the fields we care about, like
/// whether #[optional(required)] is set or whether the type is `Option<...>`.
pub(crate) fn into_field_handling(
    crate_name: Path,
    fields: Fields,
    input_crate_replacement: Option<&Ident>,
    option_wrap: Option<()>,
) -> Result<StructParsed, Error> {
    let struct_named = match &fields {
        Fields::Named(_) => StructType::Named,
        Fields::Unnamed(_) => StructType::Unnamed,
        Fields::Unit => StructType::Unit,
    };

    let fields_iter: Box<dyn Iterator<Item = Field>> = match fields {
        Fields::Named(f) => Box::new(f.named.into_iter()),
        Fields::Unnamed(f) => Box::new(f.unnamed.into_iter()),
        Fields::Unit => Box::new(iter::empty()),
    };
    let fields_with_handling = fields_iter
        .map(|mut field| {
            let attrs = FieldHelperAttributes::from_attributes(&field.attrs)?;
            // check whether input `crate` replacement is set and whether we have a path type that has `crate` as first entry.
            if let Some(input_crate_replacement) = input_crate_replacement {
                type_path_replace_crate(&mut field.ty, input_crate_replacement);
            }
            if attrs.required.is_some() && attrs.optioned_ty.is_some(){
                return error("Setting `required` and `optioned` together is not supported. You don't need to set `optioned` for `required` fields.");
            }
            let handling = if attrs.required.is_some() {
                Required
            } else if let Some(ty)=attrs.optioned_ty {
                ManualOptioned(ty)
            } else if option_wrap.is_none() && is_option(&field.ty) {
                IsOption
            } else {
                Other
            };
            Ok::<_, Error>(FieldParsed { field, handling })
        })
        .collect::<Result<Vec<_>, _>>()?;

    Ok(StructParsed {
        crate_name,
        struct_type: struct_named,
        fields: fields_with_handling,
    })
}

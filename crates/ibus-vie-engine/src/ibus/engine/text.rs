use zbus::zvariant::{Array, Dict, Signature, StructureBuilder, Value};

/// Build an IBusLookupTable with a single candidate (for popup preedit display).
pub fn ibus_lookup_table(preedit: &str) -> Value<'static> {
    let empty_dict = Value::Dict(Dict::new(&Signature::Str, &Signature::Variant));

    let candidate = ibus_text_value(preedit);
    let mut candidates = Array::new(&Signature::Variant);
    candidates.append(Value::Value(Box::new(candidate))).expect("valid candidate");

    let empty_labels = Array::new(&Signature::Variant);

    let table = StructureBuilder::new()
        .append_field(Value::Str("IBusLookupTable".into()))
        .append_field(empty_dict)
        .append_field(Value::U32(1))
        .append_field(Value::U32(0))
        .append_field(Value::Bool(false))
        .append_field(Value::Bool(false))
        .append_field(Value::I32(1))
        .append_field(Value::Array(candidates))
        .append_field(Value::Array(empty_labels))
        .build()
        .expect("valid IBusLookupTable structure");
    Value::Structure(table)
}

/// Build an IBusText variant with an underline attribute.
pub fn ibus_text_with_underline(text: &str) -> Value<'static> {
    let empty_dict = Value::Dict(Dict::new(&Signature::Str, &Signature::Variant));
    let text_len = text.chars().count() as u32;

    let attr = StructureBuilder::new()
        .append_field(Value::Str("IBusAttribute".into()))
        .append_field(empty_dict.clone())
        .append_field(Value::U32(1))         // type=1 (underline)
        .append_field(Value::U32(1))         // value
        .append_field(Value::U32(0))         // start
        .append_field(Value::U32(text_len))  // end
        .build()
        .expect("valid IBusAttribute structure");

    let mut attrs_array = Array::new(&Signature::Variant);
    attrs_array
        .append(Value::Value(Box::new(Value::Structure(attr))))
        .expect("valid variant value");

    let attr_list = StructureBuilder::new()
        .append_field(Value::Str("IBusAttrList".into()))
        .append_field(empty_dict.clone())
        .append_field(Value::Array(attrs_array))
        .build()
        .expect("valid IBusAttrList structure");

    let ibus_text = StructureBuilder::new()
        .append_field(Value::Str("IBusText".into()))
        .append_field(empty_dict)
        .append_field(Value::Str(text.to_string().into()))
        .append_field(Value::Value(Box::new(Value::Structure(attr_list))))
        .build()
        .expect("valid IBusText structure");

    Value::Structure(ibus_text)
}

/// Build an IBusText variant for commit text (no underline attribute).
pub fn ibus_text_value(text: &str) -> Value<'static> {
    let empty_dict = Value::Dict(Dict::new(&Signature::Str, &Signature::Variant));
    let empty_arr = Value::Array(Array::new(&Signature::Variant));

    let attr_list = StructureBuilder::new()
        .append_field(Value::Str("IBusAttrList".into()))
        .append_field(empty_dict.clone())
        .append_field(empty_arr)
        .build()
        .expect("valid IBusAttrList structure");

    let ibus_text = StructureBuilder::new()
        .append_field(Value::Str("IBusText".into()))
        .append_field(empty_dict)
        .append_field(Value::Str(text.to_string().into()))
        .append_field(Value::Value(Box::new(Value::Structure(attr_list))))
        .build()
        .expect("valid IBusText structure");

    Value::Structure(ibus_text)
}

use zbus::zvariant::{Array, Dict, Signature, StructureBuilder, Value};

/// Build an IBusText variant for sending over DBus.
///
/// Format: `(sa{sv}sv)` = `("IBusText", {}, text, variant(IBusAttrList))`
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

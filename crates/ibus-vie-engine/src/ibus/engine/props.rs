use zbus::zvariant::{Array, Dict, Signature, StructureBuilder, Value};

const PROP_TYPE_MENU: u32 = 3;
const PROP_TYPE_RADIO: u32 = 2;

const PROP_STATE_UNCHECKED: u32 = 0;
const PROP_STATE_CHECKED: u32 = 1;

/// Build the full IBusPropList (method menu + input mode menu).
pub fn full_prop_list(active_method: &str, input_mode: &str) -> Value<'static> {
    let method_menu = build_method_menu(active_method);
    let mode_menu = build_mode_menu(input_mode);
    ibus_prop_list(&[method_menu, mode_menu])
}

/// Build an updated IBusProperty for the method menu label.
pub fn method_menu_property(active_method: &str) -> Value<'static> {
    build_method_menu(active_method)
}

/// Build an updated IBusProperty for the input mode menu label.
pub fn mode_menu_property(input_mode: &str) -> Value<'static> {
    build_mode_menu(input_mode)
}

fn build_method_menu(active_method: &str) -> Value<'static> {
    let telex_checked = if active_method == "telex" { PROP_STATE_CHECKED } else { PROP_STATE_UNCHECKED };
    let vni_checked = if active_method == "vni" { PROP_STATE_CHECKED } else { PROP_STATE_UNCHECKED };

    let telex_prop = ibus_property("method-telex", PROP_TYPE_RADIO, "Telex", telex_checked, None);
    let vni_prop = ibus_property("method-vni", PROP_TYPE_RADIO, "VNI", vni_checked, None);
    let sub_props = ibus_prop_list(&[telex_prop, vni_prop]);

    let label = match active_method { "vni" => "VNI", _ => "Telex" };
    ibus_property("method-menu", PROP_TYPE_MENU, label, 0, Some(sub_props))
}

fn build_mode_menu(input_mode: &str) -> Value<'static> {
    let preedit_checked = if input_mode != "forward" { PROP_STATE_CHECKED } else { PROP_STATE_UNCHECKED };
    let forward_checked = if input_mode == "forward" { PROP_STATE_CHECKED } else { PROP_STATE_UNCHECKED };

    let preedit_prop = ibus_property("mode-preedit", PROP_TYPE_RADIO, "Preedit", preedit_checked, None);
    let forward_prop = ibus_property("mode-forward", PROP_TYPE_RADIO, "Gõ trực tiếp", forward_checked, None);
    let sub_props = ibus_prop_list(&[preedit_prop, forward_prop]);

    let label = if input_mode == "forward" { "Gõ trực tiếp" } else { "Preedit" };
    ibus_property("mode-menu", PROP_TYPE_MENU, label, 0, Some(sub_props))
}

fn ibus_prop_list(props: &[Value<'static>]) -> Value<'static> {
    let empty_dict = Value::Dict(Dict::new(&Signature::Str, &Signature::Variant));
    let mut props_array = Array::new(&Signature::Variant);
    for p in props {
        props_array.append(Value::Value(Box::new(p.clone()))).expect("valid variant value");
    }
    let prop_list = StructureBuilder::new()
        .append_field(Value::Str("IBusPropList".into()))
        .append_field(empty_dict)
        .append_field(Value::Array(props_array))
        .build()
        .expect("valid IBusPropList structure");
    Value::Structure(prop_list)
}

fn ibus_property(
    key: &str,
    prop_type: u32,
    label: &str,
    state: u32,
    sub_props: Option<Value<'static>>,
) -> Value<'static> {
    let empty_dict = Value::Dict(Dict::new(&Signature::Str, &Signature::Variant));
    let ibus_label = ibus_text(label);
    let ibus_tooltip = ibus_text("");
    let ibus_symbol = ibus_text("");
    let sub = sub_props.unwrap_or_else(|| ibus_prop_list(&[]));

    let prop = StructureBuilder::new()
        .append_field(Value::Str("IBusProperty".into()))
        .append_field(empty_dict)
        .append_field(Value::Str(key.to_string().into()))
        .append_field(Value::U32(prop_type))
        .append_field(Value::Value(Box::new(ibus_label)))
        .append_field(Value::Str("".into()))
        .append_field(Value::Value(Box::new(ibus_tooltip)))
        .append_field(Value::Bool(true))
        .append_field(Value::Bool(true))
        .append_field(Value::U32(state))
        .append_field(Value::Value(Box::new(sub)))
        .append_field(Value::Value(Box::new(ibus_symbol)))
        .build()
        .expect("valid IBusProperty structure");
    Value::Structure(prop)
}

fn ibus_text(text: &str) -> Value<'static> {
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

use zbus::zvariant::{Array, Dict, Signature, StructureBuilder, Value};

/// IBus property type constants.
const PROP_TYPE_MENU: u32 = 3;
const PROP_TYPE_RADIO: u32 = 2;

/// IBus property state constants.
const PROP_STATE_UNCHECKED: u32 = 0;
const PROP_STATE_CHECKED: u32 = 1;

/// Build the IBusPropList for the method selector menu and preedit underline menu.
///
/// Creates a method menu (Telex/VNI) and a preedit underline menu.
pub fn method_prop_list(active_method: &str, preedit_underline: u32) -> Value<'static> {
    let telex_checked = if active_method == "telex" {
        PROP_STATE_CHECKED
    } else {
        PROP_STATE_UNCHECKED
    };
    let vni_checked = if active_method == "vni" {
        PROP_STATE_CHECKED
    } else {
        PROP_STATE_UNCHECKED
    };

    let telex_prop = ibus_property(
        "method-telex",
        PROP_TYPE_RADIO,
        "Telex",
        telex_checked,
        None,
    );
    let vni_prop = ibus_property("method-vni", PROP_TYPE_RADIO, "VNI", vni_checked, None);

    // Sub-menu prop list containing the radio items
    let sub_props = ibus_prop_list(&[telex_prop, vni_prop]);

    // Top-level menu item
    let label = match active_method {
        "vni" => "VNI",
        _ => "Telex",
    };
    let menu_prop = ibus_property("method-menu", PROP_TYPE_MENU, label, 0, Some(sub_props));

    // Preedit underline submenu
    let preedit_menu = build_preedit_menu(preedit_underline);

    // Top-level prop list
    ibus_prop_list(&[menu_prop, preedit_menu])
}

/// Build an updated IBusProperty for the preedit underline menu.
pub fn preedit_menu_property(preedit_underline: u32) -> Value<'static> {
    build_preedit_menu(preedit_underline)
}

fn build_preedit_menu(preedit_underline: u32) -> Value<'static> {
    let single_checked = if preedit_underline == 1 {
        PROP_STATE_CHECKED
    } else {
        PROP_STATE_UNCHECKED
    };
    let none_checked = if preedit_underline == 0 {
        PROP_STATE_CHECKED
    } else {
        PROP_STATE_UNCHECKED
    };

    let single_prop = ibus_property(
        "preedit-underline-single",
        PROP_TYPE_RADIO,
        "Gạch đơn",
        single_checked,
        None,
    );
    let none_prop = ibus_property(
        "preedit-underline-none",
        PROP_TYPE_RADIO,
        "Không gạch chân",
        none_checked,
        None,
    );
    let sub_props = ibus_prop_list(&[single_prop, none_prop]);

    let label = if preedit_underline == 1 {
        "Gạch đơn"
    } else {
        "Không gạch chân"
    };
    ibus_property("preedit-menu", PROP_TYPE_MENU, label, 0, Some(sub_props))
}

/// Build an updated IBusProperty for the method menu label.
pub fn method_menu_property(active_method: &str) -> Value<'static> {
    let label = match active_method {
        "vni" => "VNI",
        _ => "Telex",
    };

    let telex_checked = if active_method == "telex" {
        PROP_STATE_CHECKED
    } else {
        PROP_STATE_UNCHECKED
    };
    let vni_checked = if active_method == "vni" {
        PROP_STATE_CHECKED
    } else {
        PROP_STATE_UNCHECKED
    };

    let telex_prop = ibus_property(
        "method-telex",
        PROP_TYPE_RADIO,
        "Telex",
        telex_checked,
        None,
    );
    let vni_prop = ibus_property("method-vni", PROP_TYPE_RADIO, "VNI", vni_checked, None);
    let sub_props = ibus_prop_list(&[telex_prop, vni_prop]);

    ibus_property("method-menu", PROP_TYPE_MENU, label, 0, Some(sub_props))
}

/// Build an IBusPropList variant.
fn ibus_prop_list(props: &[Value<'static>]) -> Value<'static> {
    let empty_dict = Value::Dict(Dict::new(&Signature::Str, &Signature::Variant));

    let props_vec: Vec<Value<'static>> = props
        .iter()
        .map(|p| Value::Value(Box::new(p.clone())))
        .collect();

    let mut props_array = Array::new(&Signature::Variant);
    for v in props_vec {
        props_array.append(v).expect("valid variant value");
    }

    let prop_list = StructureBuilder::new()
        .append_field(Value::Str("IBusPropList".into()))
        .append_field(empty_dict)
        .append_field(Value::Array(props_array))
        .build()
        .expect("valid IBusPropList structure");

    Value::Structure(prop_list)
}

/// Build an IBusProperty variant.
///
/// IBus serialization format: `(sa{sv}suvsvbbuvv)`
/// Fields: type_name, attachments, key, type, label, icon, tooltip, sensitive, visible, state, sub_props, symbol
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
        .append_field(Value::Str(key.to_string().into())) // key (s)
        .append_field(Value::U32(prop_type)) // type (u)
        .append_field(Value::Value(Box::new(ibus_label))) // label (v)
        .append_field(Value::Str("".into())) // icon (s)
        .append_field(Value::Value(Box::new(ibus_tooltip))) // tooltip (v)
        .append_field(Value::Bool(true)) // sensitive (b)
        .append_field(Value::Bool(true)) // visible (b)
        .append_field(Value::U32(state)) // state (u)
        .append_field(Value::Value(Box::new(sub))) // sub_props (v)
        .append_field(Value::Value(Box::new(ibus_symbol))) // symbol (v)
        .build()
        .expect("valid IBusProperty structure");

    Value::Structure(prop)
}

/// Build a simple IBusText variant (no attributes).
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

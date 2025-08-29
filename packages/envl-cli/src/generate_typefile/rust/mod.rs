use std::io::Error;

use envl_config::misc::{
    config::{Config, Var},
    variable::{Type, Value},
};

pub fn parse_v_type(v_name: String, v_type: Type, structs: &mut Vec<String>) -> String {
    match v_type {
        Type::Array(boxed_element_type) => {
            format!(
                "Vec<{}>",
                parse_v_type(v_name, *boxed_element_type, structs)
            )
        }
        Type::Bool => "bool".to_string(),
        Type::Char => "char".to_string(),
        Type::Float => "f64".to_string(),
        Type::Int => "i64".to_string(),
        Type::Null => "None".to_string(),
        Type::String => "String".to_string(),
        Type::Struct(elements) => {
            let struct_name = format!("Struct{}", v_name);
            let mut struct_value = String::new();

            for (name, value) in elements {
                struct_value += &format!(
                    "pub {}: {},\n",
                    name,
                    parse_v_type(format!("{}{}", struct_name, name), value, structs)
                );
            }

            structs.push(format!(
                "#[derive(Debug, Clone)]\n{} {{ {} }}",
                struct_name, struct_value
            ));

            struct_name
        }
        Type::Uint => "u64".to_string(),
    }
}

pub fn parse_var(name: String, var: Var, structs: &mut Vec<String>) -> String {
    let parsed_type = parse_v_type(name.clone(), var.v_type, structs);
    if var.default_value != Value::Null {
        format!("pub {}: Option<{}>,", name, parsed_type)
    } else {
        format!("pub {}: {},", name, parsed_type)
    }
}

pub fn gen_rust_typefile(config: Config) -> Result<String, Error> {
    let mut typefile = String::from(include_str!("start.txt"));
    let mut structs = Vec::new();

    for (name, var) in config.vars {
        let result = parse_var(name, var, &mut structs);

        typefile += &format!("\n{}", result);
    }

    typefile.push_str(include_str!("end.txt"));

    Ok(format!("{}\n{}", structs.join("\n"), typefile))
}

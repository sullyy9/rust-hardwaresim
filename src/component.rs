pub struct Component {
    pub name: String,
    pub type_name: String,
}
impl Component {
    pub fn new_not_gate(new_name: &str) -> Component {
        Component {
            name: new_name.to_string(),
            type_name: "not gate".to_string(),
        }
    }
    pub fn new_node(new_name: &str) -> Component {
        Component {
            name: new_name.to_string(),
            type_name: "node".to_string(),
        }
    }
}
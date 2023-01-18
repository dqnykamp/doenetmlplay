// use std::string;

use std::collections::HashMap;

pub type ComponentName = String;

pub type ComponentNode = Box<dyn ComponentLike>;

pub type ComponentStore = HashMap<ComponentName, ComponentNode>;

fn main() {
    // enum ComponentNode {
    //     Text(TextComponent),
    //     FancyText(FancyTextComponent)
    // }

    let mut components: ComponentStore = HashMap::new();

    let string1 = String::from("hello");

    let my_text2 = TextComponent {
        hidden: false,
        copy_source: None,
        children: vec![ComponentOrString::String(string1)],
    };

    components.insert("text2".to_string(), Box::new(my_text2));

    let string1a = String::from("I'm fancy");

    let my_text3 = FancyTextComponent {
        hidden: false,
        copy_source: None,
        fancy_level: 2,
        children: vec![ComponentOrString::String(string1a)],
    };

    components.insert("text3".to_string(), Box::new(my_text3));

    let string2 = String::from(" world!");
    let my_text1 = TextComponent {
        hidden: false,
        copy_source: Some("text_source".to_string()),
        children: vec![
            ComponentOrString::Component("text2".to_string()),
            ComponentOrString::String(string2),
            ComponentOrString::Component("text3".to_string()),
        ],
    };

    components.insert("text1".to_string(), Box::new(my_text1));

    let string_source = String::from("copy me!");

    let my_text_source = TextComponent {
        hidden: false,
        copy_source: None,
        children: vec![ComponentOrString::String(string_source)],
    };

    components.insert("text_source".to_string(), Box::new(my_text_source));

    let text_as_component_node = components.get("text1").unwrap();

    println!(
        "{:?}",
        text_as_component_node
            .get_state_var_value(0, &components)
            .unwrap()
    );
}

pub trait ComponentLike {
    fn get_hidden(&self) -> bool;

    fn get_children<'a>(&'a self, components: &'a ComponentStore) -> Vec<&'a ComponentOrString>;

    fn get_state_var_list(&self) -> Vec<(&'static str, StateVarType)>;

    fn get_state_var_value(&self, ind: usize, components: &ComponentStore)
        -> Option<StateVarValue>;

    fn get_component_type(&self) -> &'static str;
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum StateVarType {
    Number,
    String,
}

#[derive(Debug, Clone)]
pub enum StateVarValue {
    Number(f64),
    String(String),
}

pub enum ComponentOrString {
    Component(ComponentName),
    String(String),
}

pub struct TextComponent {
    // value: String,
    pub hidden: bool,

    pub copy_source: Option<ComponentName>,

    pub children: Vec<ComponentOrString>,
}

impl TextComponent {
    const STATE_VAR_LIST: [(&str, StateVarType); 1] = [("value", StateVarType::String)];

    fn get_value(&self, components: &ComponentStore) -> String {
        let mut value = String::from("");

        for child in self.get_children(components).iter() {
            match child {
                ComponentOrString::String(str) => {
                    value.push_str(str);
                }
                ComponentOrString::Component(comp_name) => {
                    let comp = components.get(comp_name).unwrap();

                    if let Some(ind) = comp
                        .get_state_var_list()
                        .iter()
                        .position(|x| x == &("value", StateVarType::String))
                    {
                        if let Some(StateVarValue::String(str)) =
                            comp.get_state_var_value(ind, components)
                        {
                            value.push_str(&str.clone())
                        }
                    }
                }
            }
        }

        value
    }
}

impl ComponentLike for TextComponent {
    fn get_hidden(&self) -> bool {
        self.hidden
    }

    fn get_children<'a>(&'a self, components: &'a ComponentStore) -> Vec<&'a ComponentOrString> {
        let mut children: Vec<&ComponentOrString> = Vec::new();

        if let Some(copy_source) = &self.copy_source {
            if let Some(source) = components.get(copy_source) {
                children.extend(source.get_children(components));
            }
        }

        children.extend(self.children.iter());

        children
    }

    fn get_state_var_list(&self) -> Vec<(&'static str, StateVarType)> {
        TextComponent::STATE_VAR_LIST.to_vec()
    }

    fn get_state_var_value(
        &self,
        ind: usize,
        components: &ComponentStore,
    ) -> Option<StateVarValue> {
        if ind == 0 {
            Some(StateVarValue::String(self.get_value(components)))
        } else {
            None
        }
    }

    fn get_component_type(&self) -> &'static str {
        "text"
    }
}

pub struct FancyTextComponent {
    // value: String,
    pub hidden: bool,

    pub copy_source: Option<ComponentName>,

    pub fancy_level: usize,

    pub children: Vec<ComponentOrString>,
}

impl FancyTextComponent {
    const STATE_VAR_LIST: [(&str, StateVarType); 2] = [
        ("value", StateVarType::String),
        ("fancy_level", StateVarType::Number),
    ];

    fn get_value(&self, components: &ComponentStore) -> String {
        let delimiters = "*".repeat(self.fancy_level);

        let mut value = String::from(&delimiters);

        for child in self.get_children(components).iter() {
            match child {
                ComponentOrString::String(str) => {
                    value.push_str(str);
                }
                ComponentOrString::Component(comp_name) => {
                    let comp = components.get(comp_name).unwrap();

                    if let Some(ind) = comp
                        .get_state_var_list()
                        .iter()
                        .position(|x| x == &("value", StateVarType::String))
                    {
                        if let Some(StateVarValue::String(str)) =
                            comp.get_state_var_value(ind, components)
                        {
                            value.push_str(&str.clone())
                        }
                    }
                }
            }
        }

        value.push_str(&delimiters);

        value
    }
}

impl ComponentLike for FancyTextComponent {
    fn get_hidden(&self) -> bool {
        self.hidden
    }

    fn get_children<'a>(&'a self, components: &'a ComponentStore) -> Vec<&'a ComponentOrString> {
        let mut children: Vec<&ComponentOrString> = Vec::new();

        if let Some(copy_source) = &self.copy_source {
            if let Some(source) = components.get(copy_source) {
                children.extend(source.get_children(components));
            }
        }

        children.extend(self.children.iter());

        children
    }

    fn get_state_var_list(&self) -> Vec<(&'static str, StateVarType)> {
        FancyTextComponent::STATE_VAR_LIST.to_vec()
    }

    fn get_state_var_value(
        &self,
        ind: usize,
        components: &ComponentStore,
    ) -> Option<StateVarValue> {
        if ind == 0 {
            Some(StateVarValue::String(self.get_value(components)))
        } else if (ind == 1) {
            Some(StateVarValue::Number(self.fancy_level as f64))
        } else {
            None
        }
    }

    fn get_component_type(&self) -> &'static str {
        "fancy_text"
    }
}

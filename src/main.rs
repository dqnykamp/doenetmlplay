// use std::string;

use std::collections::HashMap;

pub type ComponentName = String;

pub type ComponentBox = Box<dyn Component>;
pub type ComponentStateBox = Box<dyn ComponentState>;
pub type EssentialDataBox = Box<dyn EssentialData>;

pub type ComponentStore = HashMap<ComponentName, ComponentBox>;

fn main() {
    // enum ComponentNode {
    //     Text(TextComponent),
    //     FancyText(FancyTextComponent)
    // }

    let mut components: ComponentStore = HashMap::new();
    let mut component_states = HashMap::new();
    let mut essential_data = HashMap::new();

    let string1 = String::from("hello");

    let my_text2 = TextComponent {
        hidden: false,
        copy_source: None,
        children: vec![ComponentOrString::String(string1)],
    };

    components.insert("text2".to_string(), Box::new(my_text2));

    // let string1a = String::from("I'm fancy");

    // let my_text3 = FancyTextComponent {
    //     hidden: false,
    //     copy_source: None,
    //     fancy_level: 2,
    //     children: vec![ComponentOrString::String(string1a)],
    // };

    // components.insert("text3".to_string(), Box::new(my_text3));

    let string2 = String::from(" world!");
    let my_text1 = TextComponent {
        hidden: false,
        copy_source: Some("text_source".to_string()),
        children: vec![
            ComponentOrString::Component("text2".to_string()),
            ComponentOrString::String(string2),
            // ComponentOrString::Component("text3".to_string()),
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


    create_component_states(&components, &mut component_states);

    create_essential_data(&components, &mut essential_data);

    let text_as_component_node = components.get("text1").unwrap();

    println!(
        "{:?}",
        text_as_component_node
            .get_state_var_value(0, &components)
            .unwrap()
    );
}


fn create_component_states(components: &HashMap<ComponentName, ComponentBox>,
    component_states: &mut HashMap<ComponentName, ComponentStateBox>
) {
    for (comp_name, comp_node) in components {
        component_states.insert(comp_name.clone(), comp_node.initialize_state());
    }
}

fn create_essential_data(components: &HashMap<ComponentName, ComponentBox>,
    essential_data: &mut HashMap<ComponentName, EssentialDataBox>
) {
    for (comp_name, comp_node) in components {
        essential_data.insert(comp_name.clone(), comp_node.initialize_essential_data());
    }
}

pub trait Component {
    fn get_hidden(&self) -> bool;

    fn get_children<'a>(&'a self, components: &'a ComponentStore) -> Vec<&'a ComponentOrString>;

    fn get_state_var_list(&self) -> Vec<(&'static str, StateVarType)>;

    fn get_state_var_value(&self, ind: usize, components: &ComponentStore)
        -> Option<StateVarValue>;

    // fn get_number_sv_value(&self, ind: usize) -> f64;

    fn get_component_type(&self) -> &'static str;

    fn initialize_state(&self) -> ComponentStateBox;
    fn initialize_essential_data(&self) -> EssentialDataBox;
}

#[derive(Debug, Clone, Copy)]
pub enum NumberState {
    Fresh(f64),
    Stale,
    Unresolved
}

#[derive(Debug, Clone)]
pub enum StringState {
    Fresh(String),
    Stale,
    Unresolved
}

pub trait ComponentState {
    fn get_number_sv_value(&self, ind: usize) -> NumberState;
    fn get_string_sv_value(&self, ind: usize) -> StringState;
    fn set_number_sv_value(&mut self, ind: usize, val: f64);
    fn set_string_sv_value(&mut self, ind: usize, val: String);
}

pub trait EssentialData {
    fn get_number_sv_essential_value(&self, ind: usize) -> f64;
    fn get_string_sv_essential_value(&self, ind: usize) -> String;
    fn set_number_sv_essential_value(&mut self, ind: usize, val: f64);
    fn set_string_sv_essential_value(&mut self, ind: usize, val: String);
    fn get_number_child_value(&self, ind: usize) -> f64;
    fn get_string_child_value(&self, ind: usize) -> String;
    fn set_number_child_value(&mut self, ind: usize, val: f64);
    fn set_string_child_value(&mut self, ind: usize, val: String);
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

impl Component for TextComponent {
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

    fn initialize_state(&self) -> ComponentStateBox {
        let tcs = TextComponentState {
            numbers: vec![NumberState::Unresolved; 2],
            strings: vec![StringState::Unresolved; 3],
        };

        Box::new(tcs)
    }

    fn initialize_essential_data(&self) -> EssentialDataBox {
        let ted = TextEssentialData {
            string_children: self.children.iter().map(|c| {
                if let ComponentOrString::String(str) = c {
                    str.clone()
                } else {
                    String::from("")
                }
            }).collect()
        };

        Box::new(ted)
    }

}


pub struct TextComponentState {
    numbers: Vec<NumberState>,
    strings: Vec<StringState>,
}

impl ComponentState for TextComponentState {
    fn get_number_sv_value(&self, ind: usize) -> NumberState {
        self.numbers[ind]
    }
    fn get_string_sv_value(&self, ind: usize) -> StringState {
        self.strings[ind].clone()
    }
    fn set_number_sv_value(&mut self, ind: usize, val: f64) {
        self.numbers[ind] = NumberState::Fresh(val);
    }
    fn set_string_sv_value(&mut self, ind: usize, val: String) {
        self.strings[ind] = StringState::Fresh(val);
    }
}


pub struct TextEssentialData {
    string_children: Vec<String>
}


impl EssentialData for TextEssentialData {

    fn get_number_sv_essential_value(&self, ind: usize) -> f64 {
        panic!("text doesn't have essential numbers");
    }
    fn get_string_sv_essential_value(&self, ind: usize) -> String {
        panic!("text doesn't have essential strings");
    }
    fn set_number_sv_essential_value(&mut self, ind: usize, val: f64) {
        panic!("text doesn't have essential numbers");
    }
    fn set_string_sv_essential_value(&mut self, ind: usize, val: String) {
        panic!("text doesn't have essential strings");
    }
    fn get_number_child_value(&self, ind: usize) -> f64 {
        panic!("text doesn't have essential number children");
    }
    fn get_string_child_value(&self, ind: usize) -> String {
        self.string_children[ind].clone()
    }
    fn set_number_child_value(&mut self, ind: usize, val: f64){
        panic!("text doesn't have essential number children");
    }
    fn set_string_child_value(&mut self, ind: usize, val: String) {
        self.string_children[ind] = val.clone();
    }
}

// pub struct FancyTextComponent {
//     // value: String,
//     pub hidden: bool,

//     pub copy_source: Option<ComponentName>,

//     pub fancy_level: usize,

//     pub children: Vec<ComponentOrString>,
// }

// impl FancyTextComponent {
//     const STATE_VAR_LIST: [(&str, StateVarType); 2] = [
//         ("value", StateVarType::String),
//         ("fancy_level", StateVarType::Number),
//     ];

//     fn get_value(&self, components: &ComponentStore) -> String {
//         let delimiters = "*".repeat(self.fancy_level);

//         let mut value = String::from(&delimiters);

//         for child in self.get_children(components).iter() {
//             match child {
//                 ComponentOrString::String(str) => {
//                     value.push_str(str);
//                 }
//                 ComponentOrString::Component(comp_name) => {
//                     let comp = components.get(comp_name).unwrap();

//                     if let Some(ind) = comp
//                         .get_state_var_list()
//                         .iter()
//                         .position(|x| x == &("value", StateVarType::String))
//                     {
//                         if let Some(StateVarValue::String(str)) =
//                             comp.get_state_var_value(ind, components)
//                         {
//                             value.push_str(&str.clone())
//                         }
//                     }
//                 }
//             }
//         }

//         value.push_str(&delimiters);

//         value
//     }
// }

// impl Component for FancyTextComponent {
//     fn get_hidden(&self) -> bool {
//         self.hidden
//     }

//     fn get_children<'a>(&'a self, components: &'a ComponentStore) -> Vec<&'a ComponentOrString> {
//         let mut children: Vec<&ComponentOrString> = Vec::new();

//         if let Some(copy_source) = &self.copy_source {
//             if let Some(source) = components.get(copy_source) {
//                 children.extend(source.get_children(components));
//             }
//         }

//         children.extend(self.children.iter());

//         children
//     }

//     fn get_state_var_list(&self) -> Vec<(&'static str, StateVarType)> {
//         FancyTextComponent::STATE_VAR_LIST.to_vec()
//     }

//     fn get_state_var_value(
//         &self,
//         ind: usize,
//         components: &ComponentStore,
//     ) -> Option<StateVarValue> {
//         if ind == 0 {
//             Some(StateVarValue::String(self.get_value(components)))
//         } else if (ind == 1) {
//             Some(StateVarValue::Number(self.fancy_level as f64))
//         } else {
//             None
//         }
//     }

//     fn get_component_type(&self) -> &'static str {
//         "fancy_text"
//     }
// }

// use std::string;

use std::{collections::HashMap, ops::Index};

pub type ComponentName = String;

pub type ComponentNode = Box<dyn ComponentLike>;


fn main() {

    // enum ComponentNode {
    //     Text(TextComponent),
    //     FancyText(FancyTextComponent)
    // }


    // let components: HashMap<ComponentName, ComponentName> = HashMap::new();


    let string1 = String::from("hello");

    let my_text2 = TextComponent {
        hidden: false,
        children: vec![ComponentOrString::String(string1)]
    };

    // let string1a = String::from("I'm fancy");

    // let my_text3 = FancyTextComponent {
    //     base_component: ComponentImpl {
    //         hidden: false,
    //         children: vec![],
    //     },
    //     textlike_children: vec![Box::new(&string1a)],
    // };

    

    let string2 = String::from(" world!");
    let my_text1 = TextComponent {
        hidden: false,
        children: vec![ComponentOrString::Component(Box::new(my_text2)), ComponentOrString::String(string2)]
    };

    // println!("{:#?}", my_text1);

    let textAsComponentNode: ComponentNode = Box::new(my_text1);

    println!("{:?}", textAsComponentNode.get_state_var_value(0).unwrap());


}

// struct ComponentDefinition {
//     component_type: String,
//     state_variables: StateVariableVariant
// }

// struct ComponentStateVariables {

// }

pub trait ComponentLike {
    fn get_hidden(&self) -> bool;

    fn get_children(&self) -> &Vec<ComponentOrString>;

    fn get_state_var_list(&self) -> Vec<(&'static str, StateVarType)>;

    fn get_state_var_value(&self, ind: usize) -> Option<StateVarValue>;

    fn get_component_type(&self) -> &'static str;

}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum StateVarType {
    Number,
    String
}

#[derive(Debug, Clone)]
pub enum StateVarValue {
    Number(f64),
    String(String)
}

pub enum ComponentOrString {
    Component(ComponentNode),
    String(String),
}



pub struct TextComponent {
    // value: String,

    pub hidden: bool,


    pub children: Vec<ComponentOrString>,

}


impl TextComponent {
    const COMPONENT_TYPE: &'static str = "text";

    const STATE_VAR_LIST: [(&str, StateVarType); 1] = [("value", StateVarType::String)];

    fn get_value(&self) -> String {
        let mut value = String::from("");

        for child in self.children.iter() {
            match child {
                ComponentOrString::String(str) => {
                    value.push_str(str);
                }
                ComponentOrString::Component(comp) => {

                    if let Some(ind) = comp.get_state_var_list().iter().position(|x| x == &("value", StateVarType::String)) {
                        if let Some(StateVarValue::String(str)) = comp.get_state_var_value(ind) {
                            value.push_str(&str.clone())

                        }
                    }
                }
            }
        }

        value
    }
}

// impl<'a> TextLike for TextComponent<'a> {
//     fn get_value(&self) -> String {
//         // let children = self.get_children();

//         let mut value = String::from("");

//         for child in self.textlike_children.iter() {
//             value.push_str(&child.as_ref().get_value())
//         }

//         value
//     }
// }

impl ComponentLike for TextComponent {
    fn get_hidden(&self) -> bool {
        self.hidden
    }

    fn get_children(&self) -> &Vec<ComponentOrString> {
        &self.children
    }

    fn get_state_var_list(&self) -> Vec<(&'static str, StateVarType)> {
        TextComponent::STATE_VAR_LIST.to_vec()
    }

    fn get_state_var_value(&self, ind: usize) -> Option<StateVarValue> {
        if ind == 0 {
            Some(StateVarValue::String(self.get_value()))
        } else {
            None
        }
    }

    fn get_component_type(&self) -> &'static str {
        "text"
    }
}

// pub struct FancyTextComponent<'a> {
//     base_component: ComponentImpl<'a>,

//     textlike_children: Vec<Box<&'a dyn TextLike>>,
// }

// impl<'a> TextLike for FancyTextComponent<'a> {
//     fn get_value(&self) -> String {
//         let mut value = String::from("**");

//         for child in self.textlike_children.iter() {
//             value.push_str(&child.as_ref().get_value())
//         }

//         value.push_str("**");

//         value
//     }
// }

// impl<'a> ComponentLike for FancyTextComponent<'a> {
//     fn get_hidden(&self) -> bool {
//         self.base_component.get_hidden()
//     }
//     fn get_children(&self) -> &Vec<ComponentOrString> {
//         &self.base_component.get_children()
//     }
// }

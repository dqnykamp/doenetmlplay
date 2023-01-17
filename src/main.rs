// use std::string;

fn main() {
    let string1 = String::from("hello");

    let my_text2 = Text {
        base_component: ComponentImpl {
            hidden: false,
            children: vec![],
        },
        textlike_children: vec![Box::new(&string1)],
    };

    let string1a = String::from("I'm fancy");

    let my_text3 = FancyText {
        base_component: ComponentImpl {
            hidden: false,
            children: vec![],
        },
        textlike_children: vec![Box::new(&string1a)],
    };

    

    let string2 = String::from(" world!");
    let my_text1 = Text {
        base_component: ComponentImpl {
            hidden: false,
            children: vec![],
        },
        textlike_children: vec![Box::new(&my_text2), Box::new(&string2), Box::new(&my_text3)],
    };

    // println!("{:#?}", my_text1);

    println!("{}", my_text1.get_value());

    struct hmm {
        a: usize,
        b: usize
    };

    let v = vec![hmm{a:3,b:2}, hmm{a:5,b:6}];

    let v1 = &v[0];



}

// struct ComponentDefinition {
//     component_type: String,
//     state_variables: StateVariableVariant
// }

// struct ComponentStateVariables {

// }

pub trait Component {
    fn get_hidden(&self) -> bool;

    fn get_children(&self) -> &Vec<ComponentOrString>;

    // fn get_state_variable_value(&self);
}

struct ComponentImpl<'a> {
    hidden: bool,
    children: Vec<ComponentOrString<'a>>,
}

impl<'a> Component for ComponentImpl<'a> {
    fn get_hidden(&self) -> bool {
        self.hidden
    }
    fn get_children(&self) -> &Vec<ComponentOrString> {
        &self.children
    }
}

pub enum ComponentOrString<'a> {
    Point(Point<'a>),
    Text(Text<'a>),
    String(String),
}

pub trait PointLike {
    fn get_n_dimensions(&self) -> usize;
    fn get_xs(&self) -> &Vec<f32>;
    fn get_x(&self, ind: usize) -> f32;
}

pub struct Point<'a> {
    xs: Vec<f32>,
    base_component: ComponentImpl<'a>,
}

impl<'a> PointLike for Point<'a> {
    fn get_n_dimensions(&self) -> usize {
        self.xs.len()
    }
    fn get_xs(&self) -> &Vec<f32> {
        &self.xs
    }
    fn get_x(&self, ind: usize) -> f32 {
        self.xs[ind]
    }
}

impl<'a> Component for Point<'a> {
    fn get_hidden(&self) -> bool {
        self.base_component.get_hidden()
    }
    fn get_children(&self) -> &Vec<ComponentOrString> {
        &self.base_component.get_children()
    }
}

pub trait TextLike {
    fn get_value(&self) -> String;
}

pub struct Text<'a> {
    // value: String,

    // Child dependency
    // 1. value depends on all children with TextLike trait
    // 2. list of all my textlike children and their appropriate state variables
    // 3. calculate the value from 2
    base_component: ComponentImpl<'a>,

    textlike_children: Vec<Box<&'a dyn TextLike>>,
}


impl<'a> Text<'a> {
    const COMPONENT_TYPE: &'static str = "text";
}

impl<'a> TextLike for Text<'a> {
    fn get_value(&self) -> String {
        // let children = self.get_children();

        let mut value = String::from("");

        for child in self.textlike_children.iter() {
            value.push_str(&child.as_ref().get_value())
        }

        value
    }
}

impl<'a> Component for Text<'a> {
    fn get_hidden(&self) -> bool {
        self.base_component.get_hidden()
    }
    fn get_children(&self) -> &Vec<ComponentOrString> {
        &self.base_component.get_children()
    }
}

pub struct FancyText<'a> {
    base_component: ComponentImpl<'a>,

    textlike_children: Vec<Box<&'a dyn TextLike>>,
}

impl<'a> TextLike for FancyText<'a> {
    fn get_value(&self) -> String {
        let mut value = String::from("**");

        for child in self.textlike_children.iter() {
            value.push_str(&child.as_ref().get_value())
        }

        value.push_str("**");

        value
    }
}

impl<'a> Component for FancyText<'a> {
    fn get_hidden(&self) -> bool {
        self.base_component.get_hidden()
    }
    fn get_children(&self) -> &Vec<ComponentOrString> {
        &self.base_component.get_children()
    }
}

impl TextLike for String {
    fn get_value(&self) -> String {
        self.clone()
    }
}

// <text><text>hello <text>hmm</text> </text> $point.x!</text>

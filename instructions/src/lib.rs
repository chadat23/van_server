use std::collections::HashMap;


#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum Device {
    Fan,
    AC,
    Heater,
    ExteriorLight,
    KitchenLight,
    BedroomLight,
    InteriorFan
}

impl Device {
    pub fn from_instruction_str(s: &str) -> Option<(&str, Self)> {
        use Device::*;
        let device_set: HashMap<&str, Device> = HashMap::from([
            ("fan", Fan),
            ("ac", AC),
            ("heater", Heater),
            ("exterior light", ExteriorLight),
            ("kitchen light", KitchenLight),
            ("bedroom light", BedroomLight),
            ("interior fan", InteriorFan),
        ]);

        for (key, &value) in device_set.iter() {
            if s.starts_with(key) {
                return Some((key, value))
            }
        }

        None
    }
}

#[derive(Debug, Copy, Clone)]
pub enum Action {
    On,
    Off,
    Up,
    Down,
    Min,
    Max,
    Set,
    NoAction,
}

impl Action {
    pub fn from_instruction_str(s: &str) -> Option<(&str, Self)> {
        use Action::*;
        let action_set: HashMap<&str, Action> = HashMap::from([
            ("on", On),
            ("off", Off),
            ("up", Up),
            ("down", Down),
            ("min", Min),
            ("max", Max),
            ("set", Set),
        ]);

        for (key, &value) in action_set.iter() {
            if s.starts_with(key) {
                return Some((key, value))
            }
        }

        None
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}

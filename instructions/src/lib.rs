use std::collections::HashMap;


const DEVICES: [(Device, &'static str); 7] = [
    (Device::RoofVent, "roof vent"),
    (Device::AC, "ac"),
    (Device::Heater, "heater"),
    (Device::ExteriorLight, "exterior light"),
    (Device::KitchenLight, "kitchen light"),
    (Device::BedroomLight, "bedroom light"),
    (Device::InteriorFan, "interior fan"),
];

const ACTIONS: [(Action, &'static str); 7] = [
    (Action::On, "on"),
    (Action::Off, "off"),
    (Action::Up, "up"),
    (Action::Down, "down"),
    (Action::Min, "min"),
    (Action::Max, "max"),
    (Action::Set, "set"),
];

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Device {
    RoofVent,
    AC,
    Heater,
    ExteriorLight,
    KitchenLight,
    BedroomLight,
    InteriorFan,
}

impl Device {
    pub fn from_instruction_str(s: &str) -> Option<(&str, Self)> {
        use Device::*;
        let device_set: HashMap<&str, Device> = DEVICES.iter().map(|(d, s)| (*s, *d)).collect();

        for (key, &value) in device_set.iter() {
            if s.starts_with(key) {
                return Some((key, value))
            }
        }

        None
    }

    pub fn to_str(&self) -> &str {
        for d in DEVICES {
            if d.0 == *self {
                return d.1
            }
        }
        ""
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
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
        let action_set: HashMap<&str, Action> = ACTIONS.iter().map(|(d, s)| (*s, *d)).collect();

        for (key, &value) in action_set.iter() {
            if s.starts_with(key) {
                return Some((key, value))
            }
        }

        None
    }

    pub fn to_str(&self) -> &str {
        for a in ACTIONS {
            if a.0 == *self {
                return a.1
            }
        }
        ""
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

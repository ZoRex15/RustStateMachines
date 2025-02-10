use std::default;

use schemars::JsonSchema;
use statig::prelude::*;

#[derive(serde::Deserialize, JsonSchema)]
pub enum Event {
    TurnOn,
    TurnOff,
    Switch,
}

// pub struct Update {
//     event: Event
// }

// enum LampState {
//     ON,
// }

// #[derive(Clone, Debug)]
// pub enum LampPower {
//     PRESENT {
//         voltage: u8
//     },
//     ABSENT,
// }

// impl Default for LampPower {
//     fn default() -> Self {
//         Self::ABSENT
//     }
// }

#[derive(Default, Clone)]
pub struct Lamp {
    pub led: bool,
    // is_powered: LampPower,
}


#[state_machine(initial = "State::off()", state(derive(Debug)), on_transition = "Self::on_transition",)]
impl Lamp {
    #[state]
    pub fn on(&mut self, event: &Event) -> Response<State> {
        use Event::*;
        match event {
            TurnOff => {
                self.led = false;
                Transition(State::off())
            }
            Switch => {
                self.led = false;
                Transition(State::off())
            }
            TurnOn => Handled,
        }
    }

    #[state]
    pub fn off(&mut self, event: &Event) -> Response<State> {
        use Event::*;

        match event {
            TurnOn  => {
                self.led = true;
                println!("");
                Transition(State::on())
            }
            Switch  => {
                self.led = true;
                Transition(State::on())
            }
            TurnOff => Handled,
        }
    }

    fn on_transition(&mut self, source: &State, target: &State) {
        println!("transitioned from `{:?}` to `{:?}`", source, target);
    }
}


#[cfg(test)]
mod test {
    use super::*;

    #[tokio::test]
    async fn on_3() {
        let mut led = Lamp::default().state_machine();
        led.handle(&Event::TurnOn);
        led.handle(&Event::TurnOn);
        led.handle(&Event::TurnOn);
        assert_eq!(led.led, true); 
    }

    #[tokio::test]
    async fn off_3() {
        let mut led = Lamp::default().state_machine();
        led.handle(&Event::TurnOff);
        led.handle(&Event::TurnOff);
        led.handle(&Event::TurnOff);
        assert_eq!(led.led, false); 
    }

    #[tokio::test]
    async fn switch_3() {
        let mut led = Lamp::default().state_machine();
        led.handle(&Event::Switch);
        led.handle(&Event::Switch);
        led.handle(&Event::Switch);
        assert_eq!(led.led, true); 
    }
}


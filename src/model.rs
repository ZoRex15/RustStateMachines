use std::default;

use statig::prelude::*;

pub enum Event {
    TurnOn {
        power: LampPower
    },
    TurnOff {
        power: LampPower
    },
    Switch {
        power: LampPower
    },
}

enum LampState {
    ON,
}

#[derive(Clone, Debug)]
pub enum LampPower {
    PRESENT {
        voltage: u8
    },
    ABSENT,
}

impl Default for LampPower {
    fn default() -> Self {
        Self::ABSENT
    }
}

#[derive(Default, Clone)]
pub struct Lamp {
    pub led: bool,
    // is_powered: LampPower,
}


#[state_machine(initial = "State::off()", state(derive(Debug)), on_transition = "Self::on_transition",)]
impl Lamp {
    #[state]
    pub async fn on(&mut self, event: &Event) -> Response<State> {
        use Event::*;
        match event {
            TurnOff {power: LampPower::ABSENT} => {
                self.led = false;
                Transition(State::off())
            }
            Switch {power: LampPower::PRESENT {voltage: (200..=240)}}  => {
                self.led = false;
                Transition(State::off())
            }
            TurnOn {power: LampPower::PRESENT {voltage: (200..=240)}} => Handled,
            TurnOn {power: LampPower::ABSENT}  
            | Switch {power: LampPower::ABSENT} => Transition(State::off()),
        }
    }

    #[state]
    pub async fn off(&mut self, event: &Event) -> Response<State> {
        use Event::*;

        match event {
            (TurnOn, LampPower::PRESENT)  => {
                self.led = true;
                Transition(State::on())
            }
            (Switch, LampPower::PRESENT)  => {
                self.led = true;
                Transition(State::on())
            }
            (TurnOff, _) => Handled,
            (TurnOn | Switch, LampPower::ABSENT) => Handled,
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
        led.handle(&Event::TurnOn).await;
        led.handle(&Event::TurnOn).await;
        led.handle(&Event::TurnOn).await;
        assert_eq!(led.led, true); 
    }

    #[tokio::test]
    async fn off_3() {
        let mut led = Lamp::default().state_machine();
        led.handle(&Event::TurnOff).await;
        led.handle(&Event::TurnOff).await;
        led.handle(&Event::TurnOff).await;
        assert_eq!(led.led, true); 
    }

    #[tokio::test]
    async fn switch_3() {
        let mut led = Lamp::default().state_machine();
        led.handle(&Event::Switch).await;
        led.handle(&Event::Switch).await;
        led.handle(&Event::Switch).await;
        assert_eq!(led.led, true); 
    }
}


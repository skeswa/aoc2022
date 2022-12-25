use crate::{instruction::Instruction, screen::Screen};

/// Executes instructions.
#[derive(Debug)]
pub(crate) struct Computer<F>
where
    F: FnMut(i64) -> (),
{
    /// How many cycles have completed.
    cycle: u64,
    /// Callback that gets invoked when a new signal strength is emitted.
    on_signal_strength: F,
    /// State of this [Computer].
    register: i64,
    /// Screen that this computer writes to.
    pub(crate) screen: Screen,
}

impl<F> Computer<F>
where
    F: FnMut(i64) -> (),
{
    /// Creates and returns a new [Computer].
    ///
    ///  * `on_signal_strength` is the callback that gets invoked when a new
    ///    signal strength is emitted
    pub(crate) fn new(on_signal_strength: F) -> Computer<F> {
        Computer {
            cycle: 0,
            on_signal_strength: on_signal_strength,
            register: 1,
            screen: Screen::new(40, 6, 3),
        }
    }

    /// Executes the given `instruction`.
    pub(crate) fn compute(&mut self, instruction: Instruction) {
        match instruction {
            Instruction::Add(integer) => {
                self.tick();
                self.tick();

                self.register += integer;
            }
            Instruction::NoOp => {
                self.tick();
            }
        }
    }

    /// Advances the instruction clock.
    fn tick(&mut self) {
        self.cycle = self.cycle + 1;

        if self.cycle == 20 || (self.cycle > 20 && ((self.cycle - 20) % 40 == 0)) {
            let signal_strength = (self.cycle as i64) * self.register;

            (self.on_signal_strength)(signal_strength)
        }

        self.screen.paint(self.register);
    }
}

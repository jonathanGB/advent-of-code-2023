use std::{
    collections::{HashMap, VecDeque},
    str::Lines,
};

use crate::solver::Solver;

const BROADCASTER_ID: &str = "broadcaster";
const BUTTON_ID: &str = "button";
const RX_ID: &str = "rx";

#[derive(Clone, Copy, Debug, PartialEq)]
enum Pulse {
    High,
    Low,
}

impl Pulse {
    fn is_low_pulse(&self) -> bool {
        *self == Pulse::Low
    }

    fn is_high_pulse(&self) -> bool {
        *self == Pulse::High
    }
}

#[derive(Clone, Copy, Debug)]
struct Signal<'a> {
    sender: &'a str,
    sendee: &'a str,
    pulse: Pulse,
}

#[derive(Debug)]
enum ModuleHandler<'a> {
    // Either on or off, starts at off.
    // If it receives a high pulse, ignores.
    // If receives a low pulse, flips state.
    //   If the new state is on, it sends a high pulse.
    //   If the new state is off, it sends a low pulse.
    FlipFlop {
        activated: bool,
    },
    // Remembers pulses from all input modules, starts at low pulse for each of them.
    // When it receives a pulse, it updates its memory for that sender. Then, if it
    // remembers a high pulse from all inputs, it sends a low pulse. Otherwise, it
    // issues a high pulse.
    Conjunction {
        most_recent_incoming_pulses: HashMap<&'a str, Pulse>,
    },
    // Dispatches the same input pulse to all of its destination.
    Broadcast,
    // Sends a low pulse to the broadcaster module. Only can do so once there are no
    // pulses being propagated in the system.
    Button,
    // Unnamed module. Can receive signals, but does nothing.
    // For part 2, we keep track whether the sink (named rx) has
    // received a low pulse.
    Sink {
        received_low_pulse: bool,
    },
}

impl<'a> ModuleHandler<'a> {
    fn handle(&mut self, signal: Signal<'a>) -> Option<Pulse> {
        match self {
            ModuleHandler::FlipFlop { activated } => {
                if signal.pulse.is_high_pulse() {
                    return None;
                }

                *activated = !*activated;
                if *activated {
                    Some(Pulse::High)
                } else {
                    Some(Pulse::Low)
                }
            }
            ModuleHandler::Conjunction {
                most_recent_incoming_pulses,
            } => {
                most_recent_incoming_pulses.insert(signal.sender, signal.pulse);
                let any_incoming_low_pulse = most_recent_incoming_pulses
                    .values()
                    .any(Pulse::is_low_pulse);
                if any_incoming_low_pulse {
                    Some(Pulse::High)
                } else {
                    Some(Pulse::Low)
                }
            }
            ModuleHandler::Broadcast => Some(signal.pulse),
            ModuleHandler::Button => Some(Pulse::Low),
            ModuleHandler::Sink { received_low_pulse } => {
                if signal.pulse.is_low_pulse() {
                    *received_low_pulse = true;
                }

                None
            }
        }
    }
}

#[derive(Debug)]
struct Module<'a> {
    output_modules: Vec<&'a str>,
    handler: ModuleHandler<'a>,
}

impl<'a> Module<'a> {
    fn handle(&mut self, inbound_signal: Signal<'a>) -> Vec<Signal<'a>> {
        match self.handler.handle(inbound_signal) {
            Some(outbound_pulse) => self
                .output_modules
                .iter()
                .map(move |output_module| Signal {
                    sender: inbound_signal.sendee,
                    sendee: output_module,
                    pulse: outbound_pulse,
                })
                .collect(),
            None => Vec::new(),
        }
    }
}

#[derive(Debug)]
struct CommunicationCoordinator<'a> {
    modules: HashMap<&'a str, Module<'a>>,
}

impl<'a> CommunicationCoordinator<'a> {
    fn initiate_communication(&mut self) -> (u64, u64) {
        let mut num_low_pulses = 0;
        let mut num_high_pulses = 0;
        let mut signals_in_flight = VecDeque::from([Signal {
            sender: BUTTON_ID,
            sendee: BROADCASTER_ID,
            pulse: Pulse::Low,
        }]);

        while let Some(signal_in_flight) = signals_in_flight.pop_front() {
            match signal_in_flight.pulse {
                Pulse::High => num_high_pulses += 1,
                Pulse::Low => num_low_pulses += 1,
            }

            let outbound_signals = self
                .modules
                .get_mut(signal_in_flight.sendee)
                .unwrap()
                .handle(signal_in_flight);
            signals_in_flight.extend(outbound_signals);
        }

        (num_low_pulses, num_high_pulses)
    }

    fn new(lines: Lines<'a>) -> Self {
        let mut modules = HashMap::new();
        let button_module = Module {
            output_modules: vec![BROADCASTER_ID],
            handler: ModuleHandler::Button,
        };
        let broadcaster_module = Module {
            output_modules: Vec::new(),
            handler: ModuleHandler::Broadcast,
        };
        modules.insert(BUTTON_ID, button_module);
        modules.insert(BROADCASTER_ID, broadcaster_module);

        let mut all_senders = Vec::new();
        let mut all_sendees: Vec<Vec<_>> = Vec::new();
        for line in lines {
            let (sender, sendees) = line.split_once(" -> ").unwrap();
            all_senders.push(sender);
            all_sendees.push(sendees.split(", ").collect());
        }

        // Start by initializing the modules. We will subsequently update the modules with their
        // inbound/outbound modules.
        for sender in &mut all_senders {
            if sender.starts_with('%') {
                *sender = sender.strip_prefix('%').unwrap();
                modules.insert(
                    sender,
                    Module {
                        output_modules: Vec::new(),
                        handler: ModuleHandler::FlipFlop { activated: false },
                    },
                );
            } else if sender.starts_with('&') {
                *sender = sender.strip_prefix('&').unwrap();
                modules.insert(
                    sender,
                    Module {
                        output_modules: Vec::new(),
                        handler: ModuleHandler::Conjunction {
                            most_recent_incoming_pulses: HashMap::new(),
                        },
                    },
                );
            } else {
                // The only other case is the broadcaster module, which is already initialized.
            }
        }

        for (sender, sendees) in all_senders.iter().zip(all_sendees.into_iter()) {
            for sendee in &sendees {
                match modules.get_mut(sendee) {
                    Some(sendee_module) => {
                        if let ModuleHandler::Conjunction {
                            most_recent_incoming_pulses,
                        } = &mut sendee_module.handler
                        {
                            most_recent_incoming_pulses.insert(sender, Pulse::Low);
                        }
                    }
                    None => {
                        // If the sendee does not exist, then this is a test output. Mark it
                        // as a sink.
                        modules.insert(
                            sendee,
                            Module {
                                output_modules: Vec::new(),
                                handler: ModuleHandler::Sink {
                                    received_low_pulse: false,
                                },
                            },
                        );
                    }
                }
            }

            modules.get_mut(sender).unwrap().output_modules = sendees;
        }

        CommunicationCoordinator { modules }
    }
}

pub struct Day20Solver {}

impl Solver for Day20Solver {
    fn solve_part1() {
        let file = std::fs::read_to_string("src/day20/input.txt").unwrap();
        let mut communication_coordinator = CommunicationCoordinator::new(file.lines());

        let (low_pulse_counts, high_pulse_counts): (Vec<_>, Vec<_>) = (0..1000)
            .map(|_| communication_coordinator.initiate_communication())
            .unzip();
        let sum_of_all_low_and_high_pulses_products =
            low_pulse_counts.iter().sum::<u64>() * high_pulse_counts.iter().sum::<u64>();
        println!(
            "The sum of all product of all low and high pulses after 1000 iterations is {}",
            sum_of_all_low_and_high_pulses_products
        );
    }

    fn solve_part2() {
        // NOTE: This will not finish within a reasonable amount of time, per the README.

        let file = std::fs::read_to_string("src/day20/input.txt").unwrap();
        let mut communication_coordinator = CommunicationCoordinator::new(file.lines());

        for i in 1.. {
            communication_coordinator.initiate_communication();
            match communication_coordinator
                .modules
                .get(RX_ID)
                .unwrap()
                .handler
            {
                ModuleHandler::Sink { received_low_pulse } if received_low_pulse => {
                    println!("The rx module was activated after {} presses", i);
                    break;
                }
                _ => {}
            }
        }
    }
}

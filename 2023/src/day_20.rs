/*

--- Day 20: Pulse Propagation ---

With your help, the Elves manage to find the right parts and fix all of the machines. Now, they just need to send
the command to boot up the machines and get the sand flowing again.

The machines are far apart and wired together with long cables. The cables don't connect to the machines
directly, but rather to communication modules attached to the machines that perform various initialization tasks
and also act as communication relays.

Modules communicate using pulses. Each pulse is either a high pulse or a low pulse. When a module sends a
pulse, it sends that type of pulse to each module in its list of destination modules.

There are several different types of modules:

Flip-flop modules (prefix %) are either on or off; they are initially off. If a flip-flop module receives a high pulse,
it is ignored and nothing happens. However, if a flip-flop module receives a low pulse, it flips between on and
off. If it was off, it turns on and sends a high pulse. If it was on, it turns off and sends a low pulse.

Conjunction modules (prefix &) remember the type of the most recent pulse received from each of their
connected input modules; they initially default to remembering a low pulse for each input. When a pulse is
received, the conjunction module first updates its memory for that input. Then, if it remembers high pulses for
all inputs, it sends a low pulse; otherwise, it sends a high pulse.

There is a single broadcast module (named broadcaster). When it receives a pulse, it sends the same pulse to all
of its destination modules.

Here at Desert Machine Headquarters, there is a module with a single button on it called, aptly, the button
module. When you push the button, a single low pulse is sent directly to the broadcaster module.

After pushing the button, you must wait until all pulses have been delivered and fully handled before pushing it
again. Never push the button if modules are still processing pulses.

Pulses are always processed in the order they are sent. So, if a pulse is sent to modules a, b, and c, and then
module a processes its pulse and sends more pulses, the pulses sent to modules b and c would have to be
handled first.

The module configuration (your puzzle input) lists each module. The name of the module is preceded by a
symbol identifying its type, if any. The name is then followed by an arrow and a list of its destination modules.
For example:

broadcaster -> a, b, c
%a -> b
%b -> c
%c -> inv
&inv -> a

In this module configuration, the broadcaster has three destination modules named a, b, and c. Each of these
modules is a flip-flop module (as indicated by the % prefix). a outputs to b which outputs to c which outputs to
another module named inv. inv is a conjunction module (as indicated by the & prefix) which, because it has only
one input, acts like an inverter (it sends the opposite of the pulse type it receives); it outputs to a.

By pushing the button once, the following pulses are sent:

button -low-> broadcaster
broadcaster -low-> a
broadcaster -low-> b
broadcaster -low-> c
a -high-> b
b -high-> c
c -high-> inv
inv -low-> a
a -low-> b
b -low-> c
c -low-> inv
inv -high-> a

After this sequence, the flip-flop modules all end up off, so pushing the button again repeats the same
sequence.

Here's a more interesting example:

broadcaster -> a
%a -> inv, con
&inv -> b
%b -> con
&con -> output

This module configuration includes the broadcaster, two flip-flops (named a and b), a single-input conjunction
module (inv), a multi-input conjunction module (con), and an untyped module named output (for testing
purposes). The multi-input conjunction module con watches the two flip-flop modules and, if they're both on,
sends a low pulse to the output module.

Here's what happens if you push the button once:

button -low-> broadcaster
broadcaster -low-> a
a -high-> inv
a -high-> con
inv -low-> b
con -high-> output
b -high-> con
con -low-> output

Both flip-flops turn on and a low pulse is sent to output! However, now that both flip-flops are on and con
remembers a high pulse from each of its two inputs, pushing the button a second time does something
different:

button -low-> broadcaster
broadcaster -low-> a
a -low-> inv
a -low-> con
inv -high-> b
con -high-> output

Flip-flop a turns off! Now, con remembers a low pulse from module a, and so it sends only a high pulse to output.

Push the button a third time:

button -low-> broadcaster
broadcaster -low-> a
a -high-> inv
a -high-> con
inv -low-> b
con -low-> output
b -low-> con
con -high-> output

This time, flip-flop a turns on, then flip-flop b turns off. However, before b can turn off, the pulse sent to con is
handled first, so it briefly remembers all high pulses for its inputs and sends a low pulse to output. After that,
flip-flop b turns off, which causes con to update its state and send a high pulse to output.

Finally, with a on and b off, push the button a fourth time:

button -low-> broadcaster
broadcaster -low-> a
a -low-> inv
a -low-> con
inv -high-> b
con -high-> output

This completes the cycle: a turns off, causing con to remember only low pulses and restoring all modules to their
original states.

To get the cables warmed up, the Elves have pushed the button 1000 times. How many pulses got sent as a
result (including the pulses sent by the button itself)?

In the first example, the same thing happens every time the button is pushed: 8 low pulses and 4 high pulses are
sent. So, after pushing the button 1000 times, 8000 low pulses and 4000 high pulses are sent. Multiplying these
together gives 32000000.

In the second example, after pushing the button 1000 times, 4250 low pulses and 2750 high pulses are sent.
Multiplying these together gives 11687500.

Consult your module configuration; determine the number of low pulses and high pulses that would be sent
after pushing the button 1000 times, waiting for all pulses to be fully handled after each push of the button. What
do you get if you multiply the total number of low pulses sent by the total number of high pulses sent?

 */

#[cfg(test)]
mod test {
    use std::collections::{HashMap, VecDeque};

    #[derive(Copy, Clone, Debug)]
    enum Pulse {
        Low,
        High,
    }
    use Pulse::*;

    #[derive(Debug)]
    enum Module<'a> {
        FlipFlop {
            state: bool,
            outputs: Vec<usize>,
        },
        Conjunction {
            inputs: Vec<(usize, Pulse)>,
            outputs: Vec<usize>,
        },
        Broadcaster {
            outputs: Vec<usize>,
        },
        Button {
            outputs: Vec<usize>,
        },
        Output(&'a str),
    }
    use Module::*;

    impl<'a> Module<'a> {
        fn outputs(&self) -> Option<&Vec<usize>> {
            match self {
                FlipFlop { state: _, outputs } => Some(outputs),
                Conjunction { inputs: _, outputs } => Some(outputs),
                Broadcaster { outputs } | Button { outputs } => Some(outputs),
                Output(_) => None,
            }
        }

        fn outputs_mut(&mut self) -> Option<&mut Vec<usize>> {
            match self {
                FlipFlop { state: _, outputs } => Some(outputs),
                Conjunction { inputs: _, outputs } => Some(outputs),
                Broadcaster { outputs } | Button { outputs } => Some(outputs),
                Output(_) => None,
            }
        }

        fn num_outputs(&self) -> usize {
            match self.outputs() {
                Some(outputs) => outputs.len(),
                None => 0,
            }
        }
    }

    fn parse_input(input: &str) -> (usize, Vec<Module>) {
        let input = input.trim();
        let (mut modules, mut indexmap) = input.lines().fold(
            (
                vec![Button {
                    outputs: Vec::new(),
                }],
                HashMap::<&str, usize>::new(),
            ),
            |(mut modules, mut indexmap), line| {
                let (name, _) = line
                    .trim_start_matches(&['%', '&'])
                    .split_once(" -> ")
                    .unwrap();
                indexmap.entry(name).or_insert(modules.len());
                modules.push(match name {
                    _ if line.starts_with("%") => FlipFlop {
                        state: false,
                        outputs: Vec::new(),
                    },
                    _ if line.starts_with("&") => Conjunction {
                        inputs: Vec::new(),
                        outputs: Vec::new(),
                    },
                    "broadcaster" => Broadcaster {
                        outputs: Vec::new(),
                    },
                    _ => panic!("Unrecognized module"),
                });
                (modules, indexmap)
            },
        );
        let mut outputs: Vec<usize> = Vec::new();
        for line in input.lines() {
            let (name, dststr) = line
                .trim_start_matches(&['%', '&'])
                .split_once(" -> ")
                .unwrap();
            let mi = indexmap[name];
            outputs.extend(
                dststr
                    .split(", ")
                    .filter_map(|dst| match indexmap.get(dst).clone() {
                        Some(mi) => Some(*mi),
                        None => {
                            modules.push(Output(dst));
                            indexmap.insert(dst, modules.len() - 1);
                            Some(modules.len() - 1)
                        }
                    }),
            );
            for &di in &outputs {
                match &mut modules[di] {
                    Conjunction { inputs, .. } => {
                        match inputs.iter_mut().find(|(i, _p)| i == &mi) {
                            Some((_, p)) => *p = Low,
                            None => inputs.push((mi, Low)),
                        }
                    }
                    _ => {}
                }
            }
            match modules[mi].outputs_mut() {
                Some(dst) => dst.extend(outputs.drain(..)),
                None => {}
            }
        }
        let broadcaster_i = modules
            .iter()
            .position(|m| matches!(m, Broadcaster { .. }))
            .unwrap();
        match modules[0].outputs_mut() {
            Some(dst) => dst.push(broadcaster_i),
            None => {}
        }
        (broadcaster_i, modules)
    }

    fn part_1(input: &str) -> usize {
        let (broadcaster_i, mut modules) = parse_input(input);
        let mut queue = VecDeque::<(usize, usize, Pulse)>::with_capacity(modules.len());
        let mut nlo = 0usize;
        let mut nhi = 0usize;
        for _ in 0..1000 {
            queue.clear();
            nlo += 1; // Button to broadcaster.
            queue.push_back((broadcaster_i, 0, Low));
            while let Some((receiver, sender, pulse)) = queue.pop_front() {
                let pulse = match (&mut modules[receiver], pulse) {
                    (FlipFlop { .. }, High) => continue,
                    (FlipFlop { state, .. }, Low) => {
                        if *state {
                            *state = false;
                            Low
                        } else {
                            *state = true;
                            High
                        }
                    }
                    (Conjunction { inputs, .. }, pulse) => {
                        inputs.iter_mut().find(|(i, _p)| *i == sender).unwrap().1 = pulse;
                        if inputs.iter().all(|(_i, p)| matches!(p, High)) {
                            Low
                        } else {
                            High
                        }
                    }
                    (Broadcaster { .. }, pulse) => pulse,
                    (Output(_), _) | (Button { .. }, _) => continue,
                };
                let noutputs = modules[receiver].num_outputs();
                match pulse {
                    Low => nlo += noutputs,
                    High => nhi += noutputs,
                }
                match modules[receiver].outputs() {
                    Some(dst) => queue.extend(dst.iter().map(|dst| (*dst, receiver, pulse))),
                    None => {}
                }
            }
        }
        return nlo * nhi;
    }

    fn part_2(input: &str) -> usize {
        let (broadcaster_i, mut modules) = parse_input(input);
        println!("{}", modules.len());
        println!("{modules:?}");
        // todo!();
        let mut queue = VecDeque::<(usize, usize, Pulse)>::with_capacity(modules.len());
        let mut count = 1usize;
        loop {
            queue.clear();
            queue.push_back((broadcaster_i, 0, Low));
            while let Some((receiver, sender, pulse)) = queue.pop_front() {
                let pulse = match (&mut modules[receiver], pulse) {
                    (FlipFlop { .. }, High) => continue,
                    (FlipFlop { state, .. }, Low) => {
                        if *state {
                            *state = false;
                            Low
                        } else {
                            *state = true;
                            High
                        }
                    }
                    (Conjunction { inputs, .. }, pulse) => {
                        inputs.iter_mut().find(|(i, _p)| *i == sender).unwrap().1 = pulse;
                        if inputs.iter().all(|(_i, p)| matches!(p, High)) {
                            Low
                        } else {
                            High
                        }
                    }
                    (Broadcaster { .. }, pulse) => pulse,
                    (Output(name), Low) if (*name == "rx") => {
                        return count;
                    }
                    (Output(name), pulse) => {
                        println!("{name}, {pulse:?}");
                        continue;
                    }
                    (Button { .. }, _) => continue,
                };
                match modules[receiver].outputs() {
                    Some(dst) => queue.extend(dst.iter().map(|dst| (*dst, receiver, pulse))),
                    None => {}
                }
            }
            count += 1;
        }
    }

    #[test]
    fn t_part_1() {
        assert_eq!(part_1(EXAMPLE_1), 32000000);
        assert_eq!(part_1(EXAMPLE_2), 11687500);
        assert_eq!(part_1(INPUT), 836127690);
    }

    #[test]
    fn _t_part_2() {
        assert!(false);
        assert_eq!(part_2(INPUT), 0);
    }

    const EXAMPLE_1: &str = "
broadcaster -> a, b, c
%a -> b
%b -> c
%c -> inv
&inv -> a
";

    const EXAMPLE_2: &str = "
broadcaster -> a
%a -> inv, con
&inv -> b
%b -> con
&con -> output
";

    const INPUT: &str = "
%ng -> vz
%hv -> zz
%cn -> fv, kp
%sc -> sm
&rt -> jf, hv, bs, kt, fn
%bc -> fv
%sb -> gk
%vz -> gk, lg
%sm -> mx
%kp -> fv, pq
&gk -> mx, sc, vq, bz, ng, zk, sm
%bs -> rt, mk
%pn -> rt
%rq -> sl, xd
%jr -> fv, bc
%vm -> rt, pn
%rk -> gk, sb
%gs -> xt
%dc -> sl
%bz -> gk, sc
%ql -> sl, fz
%kt -> bt
%gn -> fv, hk
broadcaster -> bs, rq, cn, bz
%rl -> rh, gk
&hj -> rx
%vj -> rt, rr
%jx -> fv, bf
&ks -> hj
%rh -> gk, vq
%hk -> jx
%fn -> vj
%jl -> mr, sl
%vq -> ng
%mr -> sl, dc
%fk -> cc
%jc -> fk, sl
&jf -> hj
%lg -> gk, rk
%zz -> jg, rt
%pq -> lx, fv
%xt -> gn
%bf -> fv, jr
&qs -> hj
%gv -> sl, jl
%bt -> rt, fn
%mm -> sl, ql
%jg -> vm, rt
%lx -> gs
%rr -> hv, rt
&fv -> xt, qs, gs, cn, lx, hk
%mx -> rl
&zk -> hj
&sl -> fk, rq, fz, xd, ks
%fz -> jc
%mk -> rt, kt
%xd -> mm
%cc -> sl, gv
";
}

use core::time;
use std::collections::{BinaryHeap, HashMap, VecDeque, HashSet};

use itertools::Itertools;

pub fn run() {
    println!("Day 16");

    draw_line();

    let input = include_str!("../input/day_16.txt");

    let network = TunnelNetwork::new(input);

    let best = network.find_best_path();

    println!("{}", best);

    let best_with_elephants = network.find_best_path_with_elephants();

    println!("{}", best_with_elephants);
}

struct Tunnel {
    id: usize,
    flow: u64,
    connections: Vec<usize>,
    dist: Vec<usize>,
}

struct TunnelNetwork {
    tunnels: Vec<Tunnel>,
    start_tunnel: usize,
}

impl TunnelNetwork {
    fn new(s: &str) -> Self {
        let mut name_id_map = HashMap::new();

        let mut tunnels = Vec::new();

        let mut start = None;

        for (id, line) in s.lines().enumerate() {
            let mut things = line.split_whitespace();
            things.next();
            let name = things.next().unwrap();

            name_id_map.insert(name, id);

            if name == "AA" {
                start = Some(id);
            }
        }

        for (id, line) in s.lines().enumerate() {
            let mut things = line.split_whitespace();
            things.next();
            things.next();
            things.next();
            things.next();

            let flow = things.next().map_or(0, |s| {
                str::parse(s.split_once("=").unwrap().1.trim_end_matches(";")).unwrap()
            });

            things.next();
            things.next();
            things.next();
            things.next();

            let connections = things
                .map(|s| s.trim_end_matches(","))
                .map(|s| *name_id_map.get(s).unwrap())
                .collect_vec();

            tunnels.push(Tunnel {
                id,
                flow,
                connections,
                dist: Vec::new(),
            });
        }

        for i in 0..tunnels.len() {
            for j in 0..tunnels.len() {
                let mut open = VecDeque::new();
                let mut explored = HashSet::new();
               
                open.push_back((i, 0));

                let mut dist = 0;
                while !open.is_empty() {
                    let cur = open.pop_front().unwrap();

                    if explored.contains(&cur.0) {
                        continue;
                    }

                    if cur.0 == j {
                        dist = cur.1;
                        break;
                    }

                    explored.insert(cur.0);
                    for connection in tunnels[cur.0].connections.iter() {
                        open.push_back((*connection, cur.1 + 1));
                    }
                }

                tunnels[i].dist.push(dist);
            }
        }

        TunnelNetwork {
            tunnels,
            start_tunnel: start.unwrap(),
        }
    }

    fn find_best_path(&self) -> u64 {
        let mut states = BinaryHeap::new();

        states.push(TunnelStateView {
            state: TunnelTraverseState {
                pos: self.start_tunnel,
                visited: 0,
                total_flow: 0,
                last: None,
                time_left: 30,
            },
            potential: u64::MAX,
        });

        let mut states_buffer = Vec::new();
        let best_flow = loop {
            let state = states.pop().unwrap().state;

            if state.time_left <= 0 {
                break state.total_flow;
            }

            state.progress(&self.tunnels[state.pos], &mut states_buffer);

            for next_state in states_buffer.drain(..) {
                let potential = next_state.total_flow + next_state.potential(&self.tunnels);

                states.push(TunnelStateView {
                    state: next_state,
                    potential,
                });
            }
        };

        best_flow
    }

    fn find_best_path_with_elephants(&self) -> u64 {
        let mut states = BinaryHeap::new();

        states.push(DoubleTunnelStateView {
            states: (
                TunnelTraverseState {
                    pos: self.start_tunnel,
                    visited: 0,
                    total_flow: 0,
                    last: None,
                    time_left: 26,
                },
                TunnelTraverseState {
                    pos: self.start_tunnel,
                    visited: 0,
                    total_flow: 0,
                    last: None,
                    time_left: 26,
                },
            ),
            potential: u64::MAX,
        });

        let mut states_buffer = Vec::new();
        let mut states_buffer_2 = Vec::new();
        let best_flow = loop {
            let mut state = states.pop().unwrap().states;

            if state.0.time_left <= 0 {
                break state.0.total_flow + state.1.total_flow;
            }

            state.0.visited = state.1.visited;
            state
                .0
                .progress(&self.tunnels[state.0.pos], &mut states_buffer);

            for next_state in states_buffer.iter() {
                state.1.visited = next_state.visited;
                state
                    .1
                    .progress(&self.tunnels[state.1.pos], &mut states_buffer_2);
                for other_next_state in states_buffer_2.drain(..) {
                    let potential = next_state.total_flow
                        + other_next_state.total_flow
                        + next_state.double_potential(&other_next_state, &self.tunnels);

                    states.push(DoubleTunnelStateView {
                        states: (*next_state, other_next_state),
                        potential,
                    });
                }
            }

            states_buffer.clear();
        };

        best_flow
    }
}

struct TunnelStateView {
    state: TunnelTraverseState,
    potential: u64,
}

impl PartialEq for TunnelStateView {
    fn eq(&self, other: &Self) -> bool {
        self.potential == other.potential
    }
}

impl Eq for TunnelStateView {}

impl PartialOrd for TunnelStateView {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for TunnelStateView {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.potential.cmp(&other.potential)
    }
}

struct DoubleTunnelStateView {
    states: (TunnelTraverseState, TunnelTraverseState),
    potential: u64,
}

impl PartialEq for DoubleTunnelStateView {
    fn eq(&self, other: &Self) -> bool {
        self.potential == other.potential
    }
}

impl Eq for DoubleTunnelStateView {}

impl PartialOrd for DoubleTunnelStateView {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for DoubleTunnelStateView {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.potential.cmp(&other.potential)
    }
}

#[derive(Clone, Copy, Default)]
struct TunnelTraverseState {
    pos: usize,
    visited: u64,
    total_flow: u64,
    time_left: u64,
    last: Option<usize>,
}

impl TunnelTraverseState {
    fn progress(&self, cur_tunnel: &Tunnel, other_buffer: &mut Vec<TunnelTraverseState>) {
        let visited_this = self.visited(cur_tunnel.id);

        if cur_tunnel.flow != 0 && !visited_this {
            let total_flow =
                self.total_flow + cur_tunnel.flow * (self.time_left.checked_sub(1).unwrap_or(0));
            other_buffer.push(TunnelTraverseState {
                pos: self.pos,
                visited: self.visited | (1 << cur_tunnel.id as u64),
                total_flow,
                time_left: self.time_left - 1,
                last: None,
            });
        }

        let last_hack = self.last.unwrap_or(1000); //hack
        for tunnel in cur_tunnel.connections.iter() {
            if *tunnel == last_hack {
                continue;
            }

            other_buffer.push(TunnelTraverseState {
                pos: *tunnel,
                visited: self.visited,
                total_flow: self.total_flow,
                time_left: self.time_left - 1,
                last: Some(self.pos),
            });
        }
    }

    fn potential(&self, possible: &Vec<Tunnel>) -> u64 {
        possible
            .iter()
            .filter(|s| !self.visited(s.id))
            .map(|s| s.flow * (self.time_left.checked_sub(1 + 
                (possible[self.pos].dist[s.id] - 1) as u64).unwrap_or(1)))
            .sum()
    }

    fn double_potential(&self, other: &TunnelTraverseState, possible: &Vec<Tunnel>) -> u64 {
        possible
            .iter()
            .filter(|s| !self.visited(s.id))
            .map(|s| s.flow * (self.time_left.checked_sub(1 + 
                ((possible[self.pos].dist[s.id]).min(possible[other.pos].dist[s.id]) - 1) as u64).unwrap_or(1)))
            .sum()
    }

    fn visited(&self, other: usize) -> bool {
        self.visited & (1 << other as u64) != 0
    }
}

fn draw_line() {
    println!("-------------------------------------------------------------------------");
}
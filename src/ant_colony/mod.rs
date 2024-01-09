use std::collections::HashMap;

use itertools::Itertools;

use instance_parser::{Customer, Instance};

use crate::instance_parser;

#[derive(Copy, Clone, Debug)]
pub struct ExtendedCustomer {
    time: u16,
    capacity: u16,
    customer: Customer,
}


#[derive(Clone, Debug)]
pub struct Ant {
    time: u16,
    capacity: u16,
    distance: f32,
    path: Vec<ExtendedCustomer>,
}


#[derive(Clone, Debug)]
struct Solution {
    vehicle_count: u16,
    routes: Vec<Ant>,
}

impl Solution {
    fn edges(&self) -> impl Iterator<Item=(Customer, Customer)> + '_ {
        self.routes.iter()
            .flat_map(|x| {
                x.path.iter()
                    .map(|y| y.customer)
                    .tuple_windows::<(_, _)>()
            })
    }
}


struct Pheromones {
    decay_rate: f32,
    cost_function: Box<dyn Fn(&Solution) -> f32>,
    reinforcement_function: Box<dyn Fn(f32) -> f32>,
    pheromones: HashMap<(Customer, Customer), f32>,
}

impl Pheromones {
    fn new(
        decay_rate: f32,
        cost_function: Box<dyn Fn(&Solution) -> f32>,
        reinforcement_function: Box<dyn Fn(f32) -> f32>,
    ) -> Self {
        Pheromones {
            decay_rate,
            cost_function,
            reinforcement_function,
            pheromones: HashMap::new(),
        }
    }

    fn evaporation(&mut self) {
        for edge in self.pheromones.clone().into_keys() {
            *self.pheromones.entry(edge.to_owned())
                .or_insert(1.0) *= (1.0 - self.decay_rate);
        }
    }

    fn reinforce(&mut self, solution: &Solution) {
        for edge in solution.edges() {
            *self.pheromones.entry(edge.to_owned())
                .or_insert(1.0) += (self.reinforcement_function)((self.cost_function)(solution));
        }
    }
}

fn cost_function(
    solution: &Solution,
    instance: &Instance
) -> f32 {
    let theoretical_min: f32 = (instance.customers.iter()
        .map(|x| x.demand as f32)
        .sum::<f32>() / instance.vehicle_capacity as f32).ceil();

    let cost = theoretical_min / solution.vehicle_count as f32;

    let mut max_distance = solution.routes.iter()
        .map(|x| x.distance)
        .fold(f32::INFINITY, |a, b| a.max(b));

    if max_distance == 0.0 {
        max_distance = 1.0;
    }

    cost - solution.routes.iter()
        .map(|x| x.distance / max_distance)
        .sum::<f32>() / solution.vehicle_count as f32
}


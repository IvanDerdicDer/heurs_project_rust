use std::fs::read_to_string;

use anyhow::Result;
use regex::Regex;

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
pub struct Customer {
    number: u32,
    x: i16,
    y: i16,
    pub(crate) demand: u16,
    ready_time: u32,
    service_time: u32,
}


#[derive(Clone, Debug)]
pub struct Instance {
    vehicle_count: u16,
    pub(crate) vehicle_capacity: u16,
    pub(crate) customers: Vec<Customer>,
}


pub fn parse_instance(path: &str) -> Result<Instance> {
    let binding = read_to_string(path)?;
    let lines: Vec<&str> = binding.as_str()
        .lines()
        .map(|x| x.trim())
        .collect();

    let vehicle_line: Vec<&str> = Regex::new(r" +")?
        .splitn(lines[2], 2)
        .collect();

    let customers: Vec<Customer> = lines[7..].iter()
        .map(|x| -> Result<Vec<&str>> {
            Ok(Regex::new(r" +")?.split(x).collect::<Vec<&str>>())
        })
        .map(|x| -> Result<Customer> {
            let x = x?;
            Ok(Customer {
                number: x[0].parse()?,
                x: x[1].parse()?,
                y: x[2].parse()?,
                demand: x[3].parse()?,
                ready_time: x[4].parse()?,
                service_time: x[5].parse()?,
            })
        })
        .collect::<Result<Vec<Customer>>>()?;

    return Ok(Instance {
        vehicle_count: vehicle_line[0].parse()?,
        vehicle_capacity: vehicle_line[1].parse()?,
        customers,
    });
}
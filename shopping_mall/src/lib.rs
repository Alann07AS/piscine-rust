pub mod mall;


// pub use mall::*;
pub use mall::floor;
pub use floor::store;

pub fn biggest_store(mall: mall::Mall) -> store::Store { //receives a Mall and returns the Store with the biggest square_meters.
    mall.floors.iter()
        .flat_map(|f| f.stores.iter())
        .max_by_key(|s| s.square_meters)
        .unwrap().clone()
}

pub fn highest_paid_employee(mall: mall::Mall) -> Vec<store::employee::Employee> { //receives a Mall and returns a vector containing the Employee(s) with the highest salary.
    let employes = mall.floors.iter()
        .flat_map(|f| f.stores.iter()
        .flat_map(|s| s.employees.clone().into_iter()));
    let max_salary = employes.clone()
        .max_by(|e1, e2| e1.salary.partial_cmp(&e2.salary).unwrap() )
        .unwrap().salary;
    employes
    .filter(|e| e.salary == max_salary)
    .collect()
}

pub fn nbr_of_employees(mall: mall::Mall) -> usize { //receives a Mall and returns the number of employees and guards as a usize.
    mall.floors.iter().flat_map(|f| f.stores.iter().map(|s| s.employees.len())).sum::<usize>() +
    mall.guards.len()
}

pub fn check_for_securities(mall: &mut mall::Mall, guards: Vec<mall::guard::Guard>) { //receives a Mall and a vector of Guard. If there is not at least 1 guard for every 200 square meters of floor size, a guard should be added to the Mall.guards.
    let surface = mall.floors.iter().fold(0 as usize, |acc, f| f.size_limit as usize + acc );
    let mut guards = guards;
    for _ in 0..(surface/200 - mall.guards.len()) {
        mall.guards.push(
            guards.pop().unwrap()
        )
    }
}
pub fn cut_or_raise(mall: &mut mall::Mall)  { //receives a Mall. For each employee, the salary will be raised by 10% if they work more than 10 hours, else their salary will be decreased by 10%. You can consider that guards are not employees of the mall.
    let employes = mall.floors.iter_mut()
    .flat_map(|f| f.stores.iter_mut()
    .flat_map(|s| s.employees.iter_mut()));

    employes.for_each(|e| {
        if e.working_hours.0.abs_diff(e.working_hours.1) >= 10 {
            e.raise(e.salary *0.1);
        } else {
            e.cut(e.salary *0.1);
        }
    })
}

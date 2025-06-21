pub mod mall;

pub use mall::*;

pub fn biggest_store(mall: &Mall) -> (String, Store) {
    let mut biggest: u64 = 0;
    let mut res_store: (Option<String>, Option<Store>) = (None, None);

    for (_, floor) in &mall.floors {
        for (key, store) in &floor.stores {
            if store.square_meters > biggest {
                biggest = store.square_meters;
                res_store = (Some(key.to_string()), Some(store.clone()));
            }
        }
    }

    (res_store.0.unwrap(), res_store.1.unwrap())
}

pub fn highest_paid_employee(mal: &Mall) -> Vec<(&str, Employee)> {
    let mut emp_vec: Vec<(&str, Employee)> = vec![];
    let mut max = 0.;

    for (_, flor) in mal.floors.clone() {
        for (_, stor) in flor.stores {
            for (_, emp) in stor.employees {
                if max < emp.salary {
                    max = emp.salary;
                }
            }
        }
    }
    for (_, flor) in &mal.floors {
        for (_, stor) in &flor.stores {
            for (nn, emp) in &stor.employees {
                if max == emp.salary {
                    emp_vec.push((nn, emp.clone()))
                }
            }
        }
    }

    emp_vec
}
pub fn nbr_of_employees(mall: &Mall) -> usize {
    let mut counter: usize = 0;

    for (_, _) in mall.guards.clone() {
        counter += 1;
    }

    for (_, floor) in &mall.floors {
        for (_, store) in &floor.stores {
            for (_, _) in store.employees.clone() {
                counter += 1;
            }
        }
    }
    counter
}

pub fn check_for_securities(mall: &mut Mall, guards: Vec<(String, Guard)>) {
    let mut g = guards.clone();
    let mut total_size: u64 = 0;
    let mut total_guards: u64 = 0;

    for (_, floor) in &mall.floors {
        for (_, store) in &floor.stores {
            total_size += store.square_meters;
        }
    }

    while total_guards * 200 < total_size && !g.is_empty() {
        total_guards += 1;
        mall.hire_guard(g[0].0.clone(), g[0].1.clone());
        g.remove(0);
    }
}

pub fn cut_or_raise(mall: &mut Mall) {
    for (_, floor) in &mut mall.floors {
        for (_, store) in &mut floor.stores {
            for (_, employee) in &mut store.employees {
                dbg!(employee.working_hours.1 - employee.working_hours.0);
                if employee.working_hours.1 - employee.working_hours.0 >= 10 {
                    employee.raise(employee.salary * 0.10);
                } else {
                    employee.cut(employee.salary * 0.10);
                }
            }
        }
    }
}

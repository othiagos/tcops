use crate::common::{instance::Instance, solution::Solution};

pub fn print_instance_info(instance: &Instance) {
    println!(
        "Instance loaded with success (nodes {}, subgroups {}, clusters {}, vehicles {})",
        instance.nodes.len(),
        instance.subgroups.len(),
        instance.clusters.len(),
        instance.vehicles.len()
    );
}

pub fn print_solution(solution: &Solution) {
    println!("Instance: {}", solution.instance.name);
    println!("Solving Time: {:.2?}", solution.duration);
    println!("Status: {:?}", solution.status);
    println!("Objective Value: {:.2}", solution.get_objective_value());
    println!("Total Cost: {:.2}", solution.total_cost);
    println!("Routes:");

    for route in &solution.routes {
        let vehicle = &solution.instance.vehicles[route.vehicle_id];
        println!(
            "Vehicle {:02}: Cost: {:.2}/{:.2} ({:.1}%), Score: {:.2}, Path ({} nodes): {}",
            route.vehicle_id,
            route.cost,
            vehicle.budget,
            (route.cost / vehicle.budget * 1000.0).trunc() / 10.0,
            route.score,
            route.path.len(),
            format_path(&route.path)
        );
    }
}

fn format_path(path: &[usize]) -> String {
    let len = path.len();

    if len <= 12 {
        return format!("{:?}", path);
    }

    let first: Vec<String> = path[..5].iter().map(|n| format!("{:?}", n)).collect();
    let last: Vec<String> = path[len - 5..].iter().map(|n| format!("{:?}", n)).collect();

    format!("[{}, ..., {}]", first.join(", "), last.join(", "))
}

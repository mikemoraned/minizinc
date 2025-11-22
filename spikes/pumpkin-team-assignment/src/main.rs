use std::time::Duration;

use pumpkin_solver::Solver;
use tracing::info;

fn main() {
    let fzn_path = "./team-assignment.fzn";
    info!("Flatzinc file path: {}", fzn_path);

    let time_limit = Duration::from_mins(5);
    info!("Solving with time limit: {:?}", time_limit);

    let solver = Solver::default();

    // let flatzinc_options = FlatZincOptions::default();
}

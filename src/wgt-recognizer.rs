use z3::{Config, Context, Solver, ast::Int};

fn main() {
    // Create a Z3 configuration and context
    let cfg = Config::new();
    let ctx = Context::new(&cfg);
    
    // Create an integer variable
    let x = Int::new_const(&ctx, "x");
    
    // Create a solver
    let solver = Solver::new(&ctx);
    
    // Add constraint: x > 0
    solver.assert(&x.gt(&Int::from_i64(&ctx, 0)));
    
    // Check if the constraint is satisfiable
    match solver.check() {
        z3::SatResult::Sat => {
            println!("Constraint is satisfiable!");
            let model = solver.get_model().unwrap();
            let x_val = model.get_const_interp(&x).unwrap().as_i64().unwrap();
            println!("Solution: x = {}", x_val);
        }
        z3::SatResult::Unsat => {
            println!("Constraint is unsatisfiable!");
        }
        z3::SatResult::Unknown => {
            println!("Z3 could not determine if the constraint is satisfiable.");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_z3_solver() {
        let cfg = Config::new();
        let ctx = Context::new(&cfg);
        let x = Int::new_const(&ctx, "x");
        let solver = Solver::new(&ctx);
        
        solver.assert(&x.gt(&Int::from_i64(&ctx, 0)));
        
        assert_eq!(solver.check(), z3::SatResult::Sat);
        
        let model = solver.get_model().unwrap();
        let x_val = model.get_const_interp(&x).unwrap().as_i64().unwrap();
        assert!(x_val > 0);
    }
}

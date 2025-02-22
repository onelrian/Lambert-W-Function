// Computing the Lambert W function using the Newton-Raphson method
use anyhow::Result;

pub fn lambert_w(x: f64) -> Result<f64> {
    // x must be > -1/e
    if x < -1.0 / std::f64::consts::E {
        return Err(anyhow::anyhow!("Input out of domain: x must be >= -1/e"));
    }

    // Frist guess
    let mut w = if x == 0.0 {
                0.0
            } else if x < 1.0 {
                x
            } else {
            x * (1.0 - x.exp())
            };
     

    for _ in 0..500 {
        let ew = w.exp();
        let wew = w * ew;
        let l = wew - x;
        let l_prime = ew * (w + 1.0);
        
        // Pure Newton-Raphson update
        let w_next = w - l / l_prime; 

        // convergence tolerance
        if (w_next - w).abs() < 1e-7 { 
            return Ok(w_next);
        }

        w = w_next;
    }

    Err(anyhow::anyhow!("Failed to converge"))
}

// use anyhow::Result;

// pub fn lambert_w(x: f64) -> Result<f64> {
//     
//     let e = std::f64::consts::E;
//     if x < (-1.0 / e) {
//         return Err(anyhow::anyhow!("Input out of domain: x must be ≥ -1/e"));
//     }

//     let mut w = if x == 0.0 {
//         0.0
//     } else if x < 1.0 {
//         x
//     } else {
//         (x.ln() - x.ln().ln()).max(0.0)
//     };

//     // Newton-Raphson process
//     for _ in 0..100 {
//         let ew = w.exp();
//         let wew = w * ew;
//         let l = wew - x;
//         let l_prime = ew * (w + 1.0);
//         let l_double_prime = ew * (w + 2.0);
    
//         let denominator = l_prime - (l * l_double_prime) / (2.0 * l_prime);
//         let numerator = w - l ;
//         let w_next = numerator / denominator;
    
//         // check for convergence
//         if (w_next - w).abs() < 1e-9 {
//             return Ok(w_next);
//         }
//         w = w_next;
//     }

//     Err(anyhow::anyhow!("Failed to converge"))
// }

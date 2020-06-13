enum Rate {
    Monthly,
    Yearly
}

struct Investment {
    principal: f64,
    timestamp: f64,
    rate: f64,
    compounded: Rate,
}

impl Investment {

    fn compound(&self, time_passed: f64) -> f64 {
        // formula: A = P (1 + r/n) ^ n*t
        let mut return_value  = self.principal;

        if time_passed <= self.timestamp {
            return 0.0;
        }
        
        let times_compounded = match self.compounded {
            Rate::Yearly => {
                (time_passed - self.timestamp) / 365.0
            },
            Rate::Monthly => {
                (time_passed - self.timestamp) / 30.0
            }
        };

        for _ in 0..(1*(times_compounded as i32)) {
            return_value *=  1.0 + (self.rate / 1.0);
        }

        return_value
    }

}

fn main() {
    
    let contribution_per_interval = 400.0;
    let percent_return = 0.08;
    let invest_rate = Rate::Monthly;
    let years_to_invest = 20.0;

    let times_invested_per_year = match invest_rate {
        Rate::Monthly => {
            1.0 / 12.0
        },
        Rate::Yearly => {
            1.0
        }
    };

    let invest_duration = years_to_invest / times_invested_per_year;

    let mut investments = vec![];

    for interval in 0..(invest_duration as i32) {
        investments.push(Investment{ principal: contribution_per_interval, timestamp: (365.0 * (interval as f64) * times_invested_per_year), rate: percent_return,  compounded: Rate::Yearly });
    }

    for year in (5..=(years_to_invest as i32)).step_by(5) {
        let total: f64 = investments.iter().map(|investment| investment.compound(365.0 * year as f64)).sum();
        println!("amount contributed after {} year(s): {}", (year as i32), (year as f64 * contribution_per_interval / times_invested_per_year));
        println!("total saved after {} year(s): {}", (year as i32), total);
    }

}

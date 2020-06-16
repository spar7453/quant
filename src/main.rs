mod calculator;
use crate::calculator::option_type::OptionType;
use crate::calculator::market_variables::MarketVariables;
use crate::calculator::pricing::*;
//use crate::calculator::digital;
use crate::calculator::european;

fn main() {
    println!("Hello, world!");

    /*
    s = 250 / r = 0.02 / q = 0.01 / vol = 0.2
    call: analytic = 6.42293 / monte = 6.44517 / binom = 6.41768 / strike = 250, m = 0.1
    put : analytic = 14.5728 / monte = 14.5681 / binom = 14.5744  / strike = 260, m = 0.2
    */

    let mut mv = MarketVariables::new(0.0, 0.01, 0.02, 0.2);
    println!("Change!!");
    mv.set_s(250.0);
    println!("{:?}", mv);

    let call = european::European::new(OptionType::Call, 250.0, 0.1);
    let put = european::European::new(OptionType::Put, 260.0, 0.2);

    println!("call payoff = {:?}", call.get_payoff(&mv));
    println!("put payoff = {:?}", put.get_payoff(&mv));

    mv.set_s(260.0);
    println!("Change!!");
    println!("{:?}", mv);

    println!("call payoff = {:?}", call.get_payoff(&mv));
    println!("put payoff = {:?}", put.get_payoff(&mv));

    println!("call binomial theo = {:?}", call.binomial_tree_theo(&mv, 1000));
    println!("put binomial theo = {:?}", put.binomial_tree_theo(&mv, 1000));
    println!("");
    println!("call analytic theo = {:?}", call.analytic_theo(&mv));
    println!("put analytic theo = {:?}", put.analytic_theo(&mv));
    println!("");
    println!("call monte carlo theo = {:?}", call.montecarlo_theo(&mv, 1000));
    println!("put monte carlo theo = {:?}", put.montecarlo_theo(&mv, 1000));

}

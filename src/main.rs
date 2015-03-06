use std::env;

struct TipResult {
    amt: f64,
    tip: f64,
}

fn main() {
    let args: Vec<_> = env::args().skip(1).filter_map(|i| i.parse().ok()).collect();

    let tip = match &args[..] {
        [ref amt] => Some(get_tip(*amt, 0.15f64)),
        [ref amt, ref pct] => Some(get_tip(*amt, *pct)),
        _ => None,
    };

    match tip {
        Some(tip) => print_tip(&tip, env::args().any(|i| &i[..] == "-s")),
        None => println!("USAGE: {} AMOUNT <decimal> [TIP PERCENT] <integer> [-s] <activate script mode>", env::args().nth(0).unwrap()),
    }
}

fn print_tip(tip: &TipResult, script: bool) {
    if script {
        println!("{}", tip.tip);
    } else {
        println!("${:.2} on ${:.2}", tip.tip, tip.amt);
    }
}

fn get_tip(amt: f64, pct: f64) -> TipResult {
    TipResult {
        amt: amt,
        tip: amt * pct / 100f64,
    }
}

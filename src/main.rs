struct TipResult {
    amt: f64,
    tip: f64,
}

fn main() {
    let args: Vec<_> = std::env::args().collect();

    let tip = match &args[..] {
        [_, ref amt] => get_tip(amt.parse().ok(), Some(0.15f64)),
        [_, ref amt, ref pct] => get_tip(amt.parse().ok(), pct.parse().ok()),
        _ => None,
    };

    match tip {
        Some(tip) => println!("{:.2}", tip.tip),
        None => println!("USAGE: {} AMOUNT <decimal> [TIP PERCENT] <integer>", args[0]),
    }
}

fn get_tip(amt: Option<f64>, pct: Option<f64>) -> Option<TipResult> {
    if amt.is_none() || pct.is_none() {
        return None;
    }

    let amt = amt.unwrap();
    let pct = pct.unwrap() / 100f64;

    Some(TipResult {
        amt: amt,
        tip: amt * pct,
    })
}

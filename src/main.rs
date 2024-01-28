type Result<'a, T> = std::result::Result<T, &'a str>;


fn convert_imperial_to_metric(value: f64, unit_measure: String) -> f64 {
    let value_end: f64;
    let feet_to_cm: f64 = 30.48;
    let lb_to_kg: f64 = 0.454;
    if unit_measure == "" {
        println!("value not provided to transform");
        return 0.0;
    } else if unit_measure == "to_meter".to_string() {
        value_end = value * feet_to_cm;
    } else if unit_measure == "to_kg".to_string() {
        value_end = value * lb_to_kg;
    } else {
        println!("unmatched value");
        return 0.0;
    }
    return value_end;
}


fn imc(h: f64, w: f64, unit: String) -> Result<'static, f64> {
    let imc: f64;
    let u = unit.to_string();
    if u == "" {
        println!("unit not provided");
        return Err("unit not provided");
    } else if u == "metric".to_string() {
        imc = w/ (h.powf(2.0));
    } else if u == "imperial".to_string() {
        let height = convert_imperial_to_metric(h, "to_meter".to_string()); 
        let weight = convert_imperial_to_metric(w, "to_kg".to_string());
        imc = weight/ (height.powf(2.0));
    } else {
        println!("wrong unit given, unmatched: {{u}} ");
        return Err("no valid unit provided");
    }
   // return Response::imc 
   return Ok(imc) 
}

fn main() {
    let v: f64;
    match imc(1.85, 89.5, "metric".to_string()) {
        Ok(value) => v = value,
        Err(e) => v = 0.0,
    }
    println!("value_end imc {:?}", v);
}

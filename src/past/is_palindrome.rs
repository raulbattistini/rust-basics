
// using vec for convenience

fn fk(s: String) -> bool {
    let mut b: bool = false;
    let t = s.clone();
    let vec_o = Vec::from(s);
    let vec_t = Vec::from(t);
    let arr_o = vec_o.into_boxed_slice();
    let mut arr_t = vec_t.into_boxed_slice();
    arr_t.reverse();
    println!("values {:?}, {:?}", arr_o, arr_t);
    if arr_o == arr_t {
       b = true 
    }
    return b 

}
fn main() {
    let first_str = "arara";
    let second_str = "umapalavradiferente";
    let b1 = fk(first_str.to_string());
    let b2 = fk(second_str.to_string());
    println!("values {}, {}", b1, b2);
}

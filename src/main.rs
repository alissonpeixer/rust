fn main() {
    let nuns: Vec<i32> = vec![1,2,3,4,5,6,7,8,9];


    for nun in nuns {
        match nun {
            x if x <= 5 && x != 5 => {
                println!("{} é menor que 5",nun);
            }
            x if x  == 5 => {
                println!("{} é 5",nun);
            }
            x if x >= 5 => {
                println!("{} é maior que 5",nun);
            }
            _ => {
                println!("other")
            }
        }   
    
    }
   
}

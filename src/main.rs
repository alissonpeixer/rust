use std::io;
use std::any::type_name;


fn main() {
    let mut l_new_item_array = String::new();


    println!("Olá deseja adicionar mais numeros ao array? [y/n]");
    io::stdin().read_line(&mut l_new_item_array).unwrap();
    
    let l_new_item_array = l_new_item_array.trim();
    
    if l_new_item_array == "y" {
        let new_numer: Vec<i32> = vec![];



    } else if l_new_item_array == "n"{
        numbers(false,None);
    } else {
        println!("Opção inválida");
    }
}




fn numbers(bool_item_array: bool, new_numer: Option<Vec<i32>>){
    let mut nuns: Vec<i32> = vec![1,2,3,4,5,6,7,8,9]; 

    if bool_item_array {
        if let Some(new_numer) = new_numer {
            if new_numer.len() > 0  {
                for num in new_numer {
                    nuns.push(num);
                }
            }
            
        }
    }


    num_analysis(nuns)
}

fn num_analysis(nuns: Vec<i32>){
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
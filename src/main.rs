use std::io;

fn main() {
    let mut l_new_item_array = String::new();

    println!("Olá deseja adicionar mais numeros ao array? [y/n]");
    println!("===============================================");
    io::stdin().read_line(&mut l_new_item_array).unwrap();

    let l_new_item_array = l_new_item_array.trim();

    if l_new_item_array == "y" {
        let mut new_numer: Vec<i32> = vec![];
        let mut c_quantity = String::new();

        println!("Qauntos numeros que adicionar:?");
        println!("===============================================");
        let _ = io::stdin().read_line(&mut c_quantity);
        let n_quantity: Result<i32, _> = c_quantity.trim().parse();

        match n_quantity {
            Ok(number) => {
                let mut i = 0;

                while i < number {
                    let mut c_number = String::new();

                    println!("Digite o {}º numero", i + 1);
                    let _ = io::stdin().read_line(&mut c_number);

                    let n_number: Result<i32, _> = c_number.trim().parse();
                    match n_number {
                        Ok(numer) => {
                            new_numer.push(numer);
                        }
                        Err(_) => {
                            println!("Número inválido");
                            main();
                        }
                    }

                    i += 1;
                }

                if i == number {
                    numbers(true, Some(new_numer.clone()));
                }
            }
            Err(_) => {
                println!("Número inválido");
                main();
            }
        }
    } else if l_new_item_array == "n" {
        numbers(false, None);
    } else {
        println!("Opção inválida");
        main();
    }
}

fn numbers(bool_item_array: bool, new_numer: Option<Vec<i32>>) {
    let mut nuns: Vec<i32> = vec![1];

    if bool_item_array {
        if let Some(new_numer) = new_numer {
            if new_numer.len() > 0 {
                for num in new_numer {
                    nuns.push(num);
                }
            }
        }
    }

    num_analysis(nuns)
}

fn num_analysis(nuns: Vec<i32>) {
    for nun in nuns {
        match nun {
            x if x <= 5 && x != 5 => {
                println!("{} é menor que 5", nun);
            }
            x if x == 5 => {
                println!("{} é 5", nun);
            }
            x if x >= 5 => {
                println!("{} é maior que 5", nun);
            }
            _ => {
                println!("other")
            }
        }
    }

    main();
}

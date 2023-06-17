#!/bin/bash

echo "========================"
echo "Hello! "
echo "Deseja o que?"
echo "========================"
echo "1 - Instalar Rust Pack"
echo "========================"
echo "2 - Install cargo-watch"
echo "========================" 

read quest

if [ "$quest" -eq 1 ]; then
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
elif [ "$quest" -eq 2 ]; then  
    cargo install cargo-watch
else
    echo "valor invalido"    
fi


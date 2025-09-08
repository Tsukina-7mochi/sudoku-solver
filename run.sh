cargo build --release

echo -e "
1__4
_43_
___2
4___
" | sed -z 's/\n//' | ./target/release/sudoku

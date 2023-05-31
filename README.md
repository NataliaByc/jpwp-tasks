# ML oraz Język Rust

Autorzy: Przemysław Sosna, Natalia Być

#### Źródła:

[The Rust Programming Language - book](https://doc.rust-lang.org/book/)

[Neural Networks and Deep Learning](http://neuralnetworksanddeeplearning.com/chap1.html)

## Przygotowanie środowiska

Zalecam korzystać z jakiegoś IDE. Najlepszy do tego byłby Clion, ale VS Code z pluginem rustowym również się nada.

#### Użytkownicy systemu Windows

Pobieramy exe ze [strony rusta](https://www.rust-lang.org/learn/get-started), które pobierze nam całe środowisko.

#### Użytkownicy systemów Unixowych

Wchodzimy na [stronę rusta](https://www.rust-lang.org/learn/get-started) i kopiujemy polecenie konsolowe curl, które
instaluje nam rustup. Następnie za pomocą rustupa instalujemy ekosystem.

### Używanie

Po zainstalowaniu rusta projekt jesteśmy w stanie uruchomić za pomocą komendy **cargo run** w folderze zadania.
Nowe projekty możemy tworzyć za pomocą **cargo init**. Za pomocą **cargo check** możemy sprawdzić, czy kod się
kompiluje.
Możemy użyć **rustup component add rustfmt,** by dodać fmt, po czym używać **cargo fmt,** by sformatować kod.

## Zadania

### Zadanie 1 - neurony

W tym zadaniu nie będziemy dotykać kodu. Tutaj celem jest powtórka z sieci neuronowych. Zakładając, że neuron się
aktywuje, tj. ma wartość 1, jeśli suma (krawędzi wchodzących * ich wagi + biasu) jest większa od 0, stwórz bramkę NAND
(patrz slajd 5).
Narysuj ją w paint albo innym edytorze graficznym.

### Zadanie 2 - rust

Celem tego zadania będzie uzupełnienie gry w zgadywanie liczb tak by działała. W tym celu należy uzupełnić luki w kodzie
w folderze zadanie 2. Kod jest na bazie rustbooka. 

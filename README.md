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

W tych zadaniach będziemy uzupełniać kod, na bazie przykładów z rust booka. W przypadku zadań programistycznych należy
przesłać screenshot kodu oraz działania w konsoli.

### Zadanie 1: (ml) neurony

W tym zadaniu nie będziemy dotykać kodu. Tutaj celem jest powtórka z sieci neuronowych. Zakładając, że neuron się
aktywuje, tj. ma wartość 1, jeśli suma (krawędzi wchodzących * ich wagi + biasu) jest większa od 0, stwórz bramkę NAND
(patrz slajd 5).
Narysuj ją w paint albo innym edytorze graficznym.

### Zadanie 2: (rust) zgadywanie liczb

Celem tego zadania będzie uzupełnienie gry w zgadywanie liczb tak by działała. W tym celu należy uzupełnić luki w kodzie
w folderze zadanie 2. Komputer losuje jakąś liczbę, a my mamy odgadnąć, jaka to liczba. Kod jest na bazie rustbooka.

### Zadanie 3: (rust) minigrep cz.1

Celem tego zadania będzie uzupełnienie kodu programu minigrep. Program ten ma za zadanie wyszukiwać w plikach tekstowych
podany przez nas wzorzec. W funkcji main wywołujemy funkcje, które są w osobnym pliku lib.rs. Pierwszą z nich jest
konfiguracja na bazie przekazanych parametrów, a drugą jest uruchomienie wyszukiwania z tą konfiguracją. W tym zadaniu
musimy
uzupełnić kod w funkcji config, tak by zwracała odpowiedni config.

W pliku poem.txt umieszczono Inwokację z Pana Tadeusza.
Po odpaleniu programu w ten sposób: **cargo run -- Ty poem.txt** powinien nam zwrócić linie:
> I w Ostrej świecisz Bramie! Ty, co gród zamkowy  
> Tymczasem, przenoś moją duszę utęsknioną

### Zadanie 4: (rust) minigrep cz.2

Jest to konuuacja zadania 3. W tym zadaniu chcemy odczytać zmienną środowiskową IGNORE_CASE i na jej podstawie ustawić
tą wartosć w configu (do structa który go przechowuje musimy dodać odpowiednie pole). Następnie, na podstawie tego pola
chcemy zadecydować jaką metodę wywołać, używając match (preferowany sposób) lub np. if else.

Po odpaleniu programu w ten sposób: **IGNORE_CASE=1 cargo run -- Ty poem.txt** powinien nam zwrócić linie:
> Litwo, Ojczyzno moja! ty jesteś jak zdrowie;  
> Ile cię trzeba cenić, ten tylko się dowie,  
> I w Ostrej świecisz Bramie! Ty, co gród zamkowy  
> I zaraz mogłem pieszo, do Twych świątyń progu  
> Tymczasem, przenoś moją duszę utęsknioną  
> Do tych pagórków leśnych, do tych łąk zielonych,  
> Do tych pól malowanych zbożem rozmaitem,  
> Gdzie bursztynowy świerzop, gryka jak śnieg biała,

(Jeśli korzystamy z Powershella musimy to wywołać w ten sposób **$Env:IGNORE_CASE=1; cargo run -- to poem.txt**
zmienną możemy potem usunąć za pomocą **Remove-Item Env:IGNORE_CASE** ) 

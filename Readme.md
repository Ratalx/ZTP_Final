# Uberry
## Opis
Aplikacja "UBerry" do wybierania owoców przez pracowników firmy. Całość backendu został napisany w Ruscie za pomocą frameworku rocket.
## Kompilacja
### Rust
1. Do kompilacji potrzeba zaistalować [**rustup**](https://www.rust-lang.org/tools/install)
1. Ustawić rustup na warsję *nightly*
    ```
    rustup default nightly
    ```
### Baza danych
1. Należy pobrać dockera lub docker desktop
1. następnie w katalogu projektu wykonać komendę 
    ```bash
    docker-compose up -d
    ```
### Diesel
1. Należy pobrać diesel cli poprzez
    ```dotnetcli
    cargo install diesel_cli
    ```
1. Niestety może w tym miejscu nastąpić problem w brakujących bibliotekach diesla, jeśli by się tak stało należy wkleić zawrtość folderu bug_libs pod lokację 
    ```dotnetcli
    C:\Users\User\.rustup\toolchains\nightly-x86_64-pc-windows-msvc\lib\rustlib\x86_64-pc-windows-msvc\lib
    ```

2. następnie dodać zmienną środowiskową
    ```dotnetcli
    export DATABASE_URL=postgres://uberry:uberry_pass@localhost/uberry
    ```
3. w ostatnim kroku uruchomić migracje za pomocą komendy
    ```dotnetcli
    diesel migration run
    ```

### Kompilacja
projekt kompiluje się oraz uruchamia poprzez polecenie (należy je wywołać w folderze uberry_back)
```dotnetcli
cargo run
```

### Działanie
Projekt realizuje założenia diagramu:
```mermaid
graph TD
A[Start] --> B[Strona Startowa]
B --> C[Logowanie]
C-->D{Sukces?}
D-->|Tak| E[Dashboard]
D-->|Nie| C
E-->|Wyloguj|B
E-->|Post/Delete| F{Admin?}
E--> |Panel Zleceń| L{Admin?}
L--> |Nie| I
L--> |Tak| Z[Panel Zleceń]
E--> |Dodanie owocu do ulubionych| E
E--> Y[Lista ulubionych owoców]
F-->|Tak| G[Dodany Nowy Owoc]
G-->E
F-->|Nie| I[Popup braku uprawnien]
F-->E
B-->J{użytkownik w kontekscie}
J-->|Tak| E
J-->|Nie| C
C-->K[Rejestracja]
K-->C

```
Projekt nie zawiera warstwy frontendowej.
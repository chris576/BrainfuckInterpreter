---
marp: true
title: Präsentation
author: Christopher Hübner
theme: uncover
---

# Rust

---

<style scoped>
{
    font-size: 33px;
}
</style>
    
## Themen

1. Fundamentals
1.1 Basis-Datentypen
1.2 Arithmetik
1.3 Compund Types
1.4 Structs, Enum, Trais und Generics

2. Ownership und Borrowing
2.1 RAII 
2.2 Kontrast zu C++
2.3 Ownership, Borrowing

3. Closures

---

## Fundamental Types

--- 

## Mutability 

Alle Werttypen, Objekte und Referenzen sind in Rust standardmäßig Immutable. `mut` sorgt für Mutability.

---

### Integer Types

| Größe   | Signed | Unsigned |
| ------- | ------ | -------- |
| 8-bit   | i8     | u8       |
| 16-bit  | u16    | i16      |
| 32-bit  | i32    | u32      |
| 64-bit  | i64    | u64      |
| 128-bit | i128   | u128     |
| arch    | isize  | usize    |

Arch: Abhängig von der Architektur des Computers.

---

### Integer Literals 

| Number Literals | Beispiel    |
| --------------- | ----------- |
| Decimal         | 98_222      |
| Hex             | 0xff        |
| Octal           | 0o77        |
| Binary          | 0b1111_0000 |
| Byte            | b'A'        |

---

### Arithmetic und Übersprünge

<style scoped>
table {
    font-size: 23px;
}
</style>

| Name        | Beschreibung                                                                                                                                                                             | Beispiel                          |
| ----------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | --------------------------------- |
| Checked     | Gibt eine `Option` zurück, die den Wert `Some(v)`, oder `None` annehmen kann.                                                                                                            | ```10_u8.checked_add(20)```       |
| Saturating  | Gibt einen Wert zurück, der möglichst nahe am mathematisch korrektem Ergebnis liegt. Mit anderen Worten, im Falle eine overflow, den höchsten, oder niederwertigsten Wert im Zahlenraum. | `100_i16.saturating_add(10)`      |
| Overflowing | Gibt einen Tuple zurück. Der erste Wert des Tuples entspricht dem Ergebniss der Wrapping Arithmetik. Der zweite gibt an, ob ein Overflow passiert ist.                                   | `255_u8.overflowing_add(2)`       |
| Wrapping    | Gibt einen Wert zurück, equivalent zum mathematisch korrektem Ergebnis,  modulo der Größe des Zahlenraums. $Result \equiv func(a,b)\: mod\:n$                                            | ``` 100_u16.wrapping_add(200) ``` |

---

## Compound Types

---

### Tuple

Ein Tuple ist eine Sammlung von Werten, mit fester Größe und festem Datentyp. Er fasst Werte mit unterschiedlichen Datentypen in einer Datenstruktur zusammen. 

```rust
let tup: (i32, f64, u8) = (500, 6.4, 1);
```

Ein Element in einem Tupel kann über seinen Index referenziert werden. 

```rust
let i = tup.0;
```

---

### Array

Ein Array ist eine Sammlung von Werten mit festgelegter Größe und Datentyp. Im Unterschied zum Tupel, hat jedes Element im Array denselben Datentyp. 

```rust
let a = [1, 2, 3, 4, 5, 6];
```

---

Ein Array kann mit demselben Wert folgend definiert werden. 

```rust
// Ein Array mit den 5 Elementen, gesetzt auf 3.
let a = [3; 5]
```

Ein Array Element kann über den Index-Operator referenziert werden.

```rust
let first = a[0];
```

--- 

### Vector

Ein Vector ist eine Sammlung an Werten mit variabler Größe und festem Datentyp. Die Werte werden auf dem Heap dynamisch allokiert.

```Rust
let mut vec = Vec::new();
```

---

### Initialisierung

```rust
let mut vec = Vec::new();
```

```rust
// Ein Vektor mit 5 Elementen auf 0 gesetzt.
let vec = vec![0; 5]; 
```

---

### Slice

Mit einer Slice lässt sich ein Unterraum einer Seqzenz referenzieren, statt der ganzen Collection. Über eine Slice Referenziert werden können Strings, Arrays und Vektoren. Da es sich bei der Slice um eine Referenz handelt, übernimmt sie nicht die Ownership. 

```rust
let s = String::from("hello world");
// Startindex = 0; Länge der Slize = 5
let hello = &s[0..5];
```

---

Vereinfacht kann auch geschrieben werden: 

 ```rust
 let hello = &s[..5];
 ```

 Sofern der ganze String ab einem spezifischen Index referenziert werden soll, kann geschrieben werden: 

 ```rust
 let s = &s[3..];
 ```

Oder noch einfacher, ab Index 0: 

```rust
let s = &s[..];
```

---

### Structs

---

### Field Name Structs

```rust 
struct User {
    username: String, 
    email: String
};
```

---

### Tuple Like Structs

```rust
struct User(String, String);
```

---

### Unit Like Structs

```rust
struct User;
```

---

#### Initialisierung

```rust
let user1 = User {
    active: true,
    username: String::from("someusername123"),
    email: String::from("someone@example.com"),
    sign_in_count: 1,
};
```

---

#### Field Init Shorthand

```rust
fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}
```

---

#### Creating an Instance from another

```rust
fn main() {
    // --snip--
    let user2 = User {
        email: String::from("another@example.com"),
        ..user1
    };
}
```

---

### Implementierungen

```rust
impl User {
    fn getMailDomain(&self) -> String {
        // TODO: Implementierung um die E-Mail Domain zu extrahieren.
    }
}
```

---

### Enum

```rust

enum IpAddr {
    V4(u8, u8, u8, u8),
    V6 {
        addr: String
    }
}

impl IpAddr {
    fn doSomething(&self) {
        // TODO
    }
}

let home = IpAddr::V4(127, 0, 0, 1);

```

---

### Kontrast zu C++

```C++
/** 
  * Java style *state Pattern*.
  */
class IpAddr {
    // like Java Interface
}

/**
 * V4 is a concrete IpAddr Implementation
 */
class V4: IpAddr {
}
```

```C++

IpAddr* addr = new V4();

```

---

### Match

```rust
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}
```

---

### Traits und Generics

```rust
pub fn notify<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}
```

---

```rust
/**
 * Wird nicht funktionieren, da nicht für alle T, der 
 * Vergleichsoperator existiert. T muss einen Typen definieren, der 
 * einen Trait für den Vergleich implementiert.
 */
fn largest<T>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
} 
```

---

## Monomorphization 

Der Prozess, bei dem generischer Code in konkrete Typen umgewandelt wird, indem die konkreten Typen eingefügt werden, die zur Compilezeit ermittelt sind.

---

```rust
fn main() {
    let integer = Option<i32>::Some(5);
    let float = Option<f64>::Some(5.0);
}
```

```rust 
enum Option_i32 {
    Some(i32),
    None,
}

enum Option_f64 {
    Some(f64),
    None,
}

fn main() {
    let integer = Option_i32::Some(5);
    let float = Option_f64::Some(5.0);
}
```

---

### RAII (Resource Acquisition Is Initialization)

Grundgedanke: Ressourcen, wie Speicher, werden allokiert, wenn sie benötigt werden, und automatisch freigegeben, wenn das Objekt, das sie nutzt, zerstört wird. Die Lebensdauer der allokierten Ressourcen ist eng an die Lebensdauer des verwendenden Objekts gebunden.

---

### C++ Beispiel 

```C++
class Bob
{
    int *n;
    public: 
        Bob(int x)
        {
            this->n = new int[x]; 
        }
        ~Bob()
        {
            delete this->n;
        }
        int* get_n() {
            return this->n;
        }
}
```

---

### Einschränkung von RAII:

Es ist wichtig zu beachten, dass RAII keine Lösung für Probleme mit hängenden Zeigern bietet. Wenn Speicher von mehreren Instanzen gemeinsam genutzt und verwaltet wird, kann es im schlimmsten Fall nicht mehr garantiert werden, dass ein Zeiger immer auf gültigen Speicher zeigt. Hierfür stellt das Ownership-Prinzip in Rust Lösungen bereit.

---

### Rust Ownership & Borrowing

Der Eigentümer eines Objektes entscheidet über die Allokation und Deallokation des Speichers hinter einer Variablen.

```Rust
struct Bob {
    n: Vec<i32>
}

impl Bob {
    fn new() -> Bob {
        Bob { n: Vec::new() }
    }
}
```

---

### Move Semantics

```Rust
fn main() {
    let n = Bob::new(); 
    let m = n;
}
```

---


### Borrowing

```Rust
fn f(bob: &Bob) {
    // do something
}

fn main() {
    let n = Bob::new();
    f(&n);
}
```

So lange der Inhalt nicht modifiziert werden soll, kann Speicher von vielen Akteuren geliehen werden. 

---

### Mutable Borrowing

```Rust
fn f2 (bob: &mut Bob) {
    // modify something
} 
```

In diesem Fall unterbindet Rust allerdings das Leihen von immutablen Speicher. Denn dieser könnte sich jederzeit während des Lesens verändern.

---

## Anonyme Funktionen

---

| Sprache    | Beispiel                                   |
| ---------- | ------------------------------------------ |
| php        | ``$do = function($some) use ($outer) {};`` |
| kotlin     | `val do = { i: Int -> i + 1 }`             |
| typescript | `const do = () => { return "done" }`       |
| C++        | `auto do = []() { return "done"; }`        |

---

## Closures

1. Closures that borrow
2. Closures that steal
3. Closures that kill

---

## Closures that Borrow

```rust 
fn sort_by_statistics(cities: $mut Vec<City>, stat: Statistics) {
    cities.sort_by_key(|city| -city.get_statistic(stat));
}
```

---

## Closures that steal

```rust
fn start_sorting_thread(mut cities: Vec<Citiy>, stat: Statistic) {
    let key_fn = move |city: &City| -> i64 { -city.get_statistic(stat) };

    thread::spawn(move || {
        cities.sort_by_key(key_fn);
        cities
    })
}
```

---

# Ausblick 

<style scoped>
{
    font-size: 34px;
}
</style>

* Lifetimes (Special Generics)
* Parralelisierung
* Objektorientierung 
* Unicode und Strings
* Error Handling
* Cargo 
* Modularität und Crates
* Iterators
* Collections
* Macros
* Unsafe Blöcke
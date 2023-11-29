# Ausarbeitung Rust

## Gliederung
1 [Fundamental Types](#fundamental-types)
1.1 [Primitive Types](#primitive-types)
1.1 [Integer Types](#integer-types)
1.1 [Integer Literals](#integer-literals)
1.2[Float](#floating-point-types)
1.3 [Arithmetic](#arithmetic-übersprünge)
2. [Compound Types](#compound-types)
2.1 [Compound Types](#tuple)
2.2 [Compound Types](#array)
2.3 [Compound Types](#vector)
2.4 [Compound Types](#slices)
3. [Structs](#structs)
4. [Enums](#enum)
5. [Generics](#generics)
6. [Traits](#traits)
7. [Memory Management](#memory-management)
7.1 [RAII](#raii-ressourcenbelegung-ist-initialisierung)
7.2 [Beispiel aus C++](#beispiel-aus-c)
7.3 [Rust Ownership](#rust-ownership--borrowing)
7.4 [Move Semantics](#move-semantics)
7.5 [Borrowing](#borrowing)
8. [Closures](#closures)
8.1[Closures that borrow](#)
8.2[Closures that steal](#)

## Fundamental Types

## Primitive Types

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

#### Integer Literals

Ein Literal in Rust ist eine feste, konkrete Darstellung eines Wertes wie Zahlen, Zeichen oder boolesche Werte, die direkt im Quellcode angegeben werden und unveränderlich sind. Beispielhaft hier an Integern gezeigt: 

| Number Literals | Beispiel    |
| --------------- | ----------- |
| Decimal         | 98_222      |
| Hex             | 0xff        |
| Octal           | 0o77        |
| Binary          | 0b1111_0000 |
| Byte            | b'A'        |

#### Floating-Point Types

| Größe  | Typ |
| ------ | --- |
| 32-bit | f32 |
| 64-bit | f64 |

`f64` ist der default Typ für floating-point Werte, da dieser mit der selben Geschwindigkeit verwertet werden kann, wie sein kleiner Bruder `f32`, während er eine wesentlich größere Genauigkeit erlaubt.

### Arithmetic und Übersprünge

| Name        | Beschreibung                                                                                                                                                                             | Beispiel                          |
| ----------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | --------------------------------- |
| Checked     | Gibt eine `Option` zurück, die den Wert `Some(v)`, oder `None` annehmen kann.                                                                                                            | ```10_u8.checked_add(20)```       |
| Saturating  | Gibt einen Wert zurück, der möglichst nahe am mathematisch korrektem Ergebnis liegt. Mit anderen Worten, im Falle eine overflow, den höchsten, oder niederwertigsten Wert im Zahlenraum. | `100_i16.saturating_add(10)`      |
| Overflowing | Gibt einen Tuple zurück. Der erste Wert des Tuples entspricht dem Ergebniss der Wrapping Arithmetik. Der zweite gibt an, ob ein Overflow passiert ist.                                   | `255_u8.overflowing_add(2)`       |
| Wrapping    | Gibt einen Wert zurück, equivalent zum mathematisch korrektem Ergebnis,  modulo der Größe des Zahlenraums. $Result \equiv func(a,b)\: mod\:n$                                            | ``` 100_u16.wrapping_add(200) ``` |

## Compound Types

### Tuple

Ein Tupel ist eine Sammlung an Wertem, mit fester Größe und festen Datentypen. Er fasst Werte mit unterschiedlichen Datentypen in einer Datenstruktur zusammen. 

```rust
let tup: (i32, f64, u8) = (500, 6.4, 1);
```

Ein Element in einem Tupel kann über seinen Index referenziert werden. 

```rust
let i = tup.0;
```

### Array

Ein Array ist eine Sammlung von Werten mit festgelegter Größe und Datentyp. Im Unterschied zum Tupel, hat jedes Element im Array denselben Datentyp. 

```rust
let a = [1, 2, 3, 4, 5, 6];
```

Ein Array kann mit demselben Wert folgend definiert werden. 

```rust
// Ein Array mit den 5 Elementen, gesetzt auf 3.
let a = [3; 5]
```

Ein Array Element kann über den Index-Operator referenziert werden.

```rust
let first = a[0];
```

### Vector

Rust's Vector `Vec<T>` ermöglicht die Speicherung mehrerer Werte desselben Typs in einer Datenstruktur, die sich nebeneinander im Speicher befinden. Um einen leeren Vektor zu erstellen, wird Vec::new() verwendet und bei der Erstellung von Werten kann der vec!-Macro genutzt werden, um den Typ zu inferieren. Mittels push fügt man Werte hinzu. Der Zugriff auf Elemente erfolgt über Indizes oder die get-Methode, wobei letztere ein Option<&T> zurückgibt, um Indexfehler zu handhaben.

Es gibt zwei Möglichkeiten, auf Elemente zuzugreifen: Indexierung und die get-Methode, welche Option<&T> zurückgibt, um Fehler zu handhaben. Die Unterscheidung zwischen ihnen liegt in der Handhabung von Indexfehlern: `index` panikt, wenn der Index außerhalb des Vektors liegt, get(index) gibt None zurück.

Es ist wichtig, den Unterschied zwischen mutablen und unveränderlichen Referenzen zu verstehen, da das Hinzufügen von Elementen zu einem Vektor möglicherweise eine Neu-Allokation von Speicher erfordert, was unveränderliche Referenzen beeinflussen kann.

Iteration über Vektoren erfolgt mittels for-Schleifen, wobei sowohl immutable als auch mutable Referenzen möglich sind. Die Verwendung eines Enums erlaubt es, verschiedene Typen im selben Vektor zu speichern, was jedoch im Voraus bekannt sein muss.

Das sind die Hauptpunkte des Vectors in Rust: Erstellen, Modifizieren, Zugriff, Typenhandling und Lebensdauer.

##### Slices

Mit einer Slice lässt sich ein Unterraum einer Seqzenz referenzieren, statt der ganzen Collection. Über eine Slice Referenziert werden können Strings, Arrays und Vektoren. Da es sich bei der Slice um eine Referenz handelt, übernimmt sie nicht die Ownership. 

```rust
let s = String::from("hello world");
// Startindex = 0; Länge der Slize = 5
let hello = &s[0..5];
```

Vereinfacht kann auch geschrieben werden: 

 ```rust
 let hello = &s[..5];
 ```

 Sofern der ganze String ab einem spezifischen Index referenziert werden soll, kann geschrieben werden: 

 ```rust
 let s = &[3..];
 ```

Oder noch einfacher, ab Index 0: 

```rust
let s = &[..];
```

### Structs

| Kategorie          | Beispiel                        | Beschreibung                                                                                  |
| ------------------ | ------------------------------- | --------------------------------------------------------------------------------------------- |
| Field Name Structs | `struct User { name:String  };` | Bündelt Werte unter einem Namen.                                                              |
| Tuple like Structs | `struct User(String);`          | Adäquat zu Tuples. Mit dem Unterschied, dass diese einen Namen haben.                         |
| Unit like Structs  | `struct User;`                  | EIn Name, ohne Wert, also auch ohne allokierten Speicher. Adäquat zu `void` in Java oder C ++ |

Im Allgemeinem haben Structs kein Verhalten. Sie kapseln in erster Linie Daten. Das Verhalten einer Struct wird durch eine `Impl` definiert.

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

### Implementations

Im unterem Beispiel definiert die `impl` das Verhalten eines `User`. Vererbung zwischen Interfaces und Structs gibt es nicht. Abstraktion und Objektorientierung gibt es in Rust nicht.

Die Implementierung muss denselben Namen wie die Struct aufweisen. 

```rust
impl User {
    fn getMailDomain(&self) -> String {
        // TODO: Implementierung um die E-Mail Domain zu extrahieren.
    }
}
```

### Generics

Generics in Rust ermöglichen flexible Definitionen für Funktionen, Strukturen und Enums, die mit verschiedenen konkreten Datentypen arbeiten können. Mit Generics in Funktionsdefinitionen können wir Code flexibler gestalten und Duplikation vermeiden, indem wir eine Funktion für verschiedene Datentypen nutzen.

Zum Beispiel können wir eine Funktion schreiben, die den größten Wert in einem Slice findet, und diese Funktion für verschiedene Datentypen verwenden, anstatt separate Funktionen für jede Art von Wert zu schreiben. Wir können auch generische Methoden in Strukturen implementieren, um auf unterschiedliche Datentypen zuzugreifen und Einschränkungen wie std::cmp::PartialOrd verwenden, um sicherzustellen, dass bestimmte Operationen unterstützt werden.

Bei der Verwendung von Generics in Rust gibt es keinen Laufzeitoverhead, da der Compiler den generischen Code in spezifischen Code umwandelt, der zur Kompilierzeit spezialisiert wird. Dieser Prozess, genannt Monomorphisierung, macht Rusts Generics effizient und verursacht keine zusätzlichen Laufzeitkosten.

### Traits

Traits in Rust sind wie "Verhaltens-Sets" für verschiedene Typen. Sie erlauben das Festlegen gemeinsamer Funktionalitäten, die von unterschiedlichen Typen geteilt werden können. Mit Trait-Bounds spezifiziert man, welche Verhaltensweisen ein generischer Typ haben muss. Das Implementieren von Traits ermöglicht die Definition spezifischer Verhaltensweisen für verschiedene Typen.

Man kann Funktionen schreiben, die viele verschiedene Typen akzeptieren, indem man Traits verwendet. Die Verwendung von impl Trait in Rückgabewerten erlaubt es, dass Funktionen jeden Typen zurückgeben können, der ein bestimmtes Trait implementiert.

Um die Lesbarkeit zu verbessern, können Trait-Bounds in where-Klauseln nach der Funktionssignatur platziert werden. Dadurch wird die Implementierung von Methoden bedingt für Typen, die bestimmte Traits erfüllen. Das gibt dir die Möglichkeit, flexibleren und robusteren Code zu schreiben, der zur Compile-Zeit Fehler aufdeckt und die Performanz verbessert.

### Lifetimes 

Lifetimes in Rust sind dazu da, um sicherzustellen, dass Referenzen gültig bleiben, solange sie gebraucht werden. Jede Referenz in Rust hat eine sogenannte "Lifetime", die den Gültigkeitsbereich definiert. Meistens werden diese Lifetimes automatisch erkannt, aber in unklaren Situationen sind explizite Annotationen notwendig.

Ihr Hauptzweck ist es, "dangling references" zu verhindern, also Fälle, in denen ein Programm auf Daten zugreift, die nicht mehr gültig sind. Stell dir vor, eine Referenz verweist auf Daten, die außerhalb ihres Gültigkeitsbereichs liegen – das ist ein typisches Problem, das Rusts Borrow Checker entdeckt und verhindert.

Rust verwendet drei Regeln für die automatische Zuweisung von Lifetimes:

* Jeder Parameter bekommt eine einzigartige Lifetime.
* Wenn es nur eine Input-Lifetime gibt, wird sie allen Output-Lifetimes zugewiesen.
* Bei Methoden wird die Lifetime von self allen Output-Lifetimes zugewiesen, wenn es mehrere Input-Lifetimes gibt.

Das Hinzufügen von Lifetimes als Annotationen hilft Rust, die Sicherheit des Speicherzugriffs während der Kompilierung zu gewährleisten. Strukturen können Referenzen enthalten und benötigen dann Lifetime-Annotationen für diese Referenzen. 'static bezeichnet eine Referenz, die während des gesamten Programmablaufs gültig ist.

In Methodensignaturen innerhalb von impl-Blöcken sind explizite Lifetime-Annotationen nicht immer notwendig, da Rust in vielen Fällen automatisch die richtigen Lifetimes erkennt und zuweist.


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

Enums sind in Rust nicht nur einfache, dumme Typen, die einen oder mehrere Werte kapseln. Enums können Verhalten implementieren. Ihre Ausprägungen können komplexere Typen enthalten. Wie im obigen Beispiel eine V4 und V6 Ausprägung einer IpAddr, die sowohl ihre eigenen Werte kapselt, als auch eigenes Verhalten implementieren kann. 

Letzteres habe ich in der offiziellen Dokumentation nicht gefunden, wird aber in Reddit und Stack-Overflow heiß diskutiert.

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
/**
 * Angenommen man wollte später die genaue Ausprägung von addr kennen, dann müsste diese zeit und ressourcenintensiv innerhalb von if-else schleifen ermittelt und dementsprechend abgehalten werden. Rust-Enums ermöglicht dasselbe Pattern, aber mit einem deutlichen Geschwindigkeitsvorteil, weil eben dieser Teil in einem einfachen match statement ( adäquat zu switch ) abgehandelt werden kann.
 */
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


### Memory Management

#### RAII (Resource Acquisition Is Initialization)

Grundgedanke: Ressourcen, wie Speicher, werden allokiert, wenn sie benötigt werden, und automatisch freigegeben, wenn das Objekt, das sie nutzt, zerstört wird. Die Lebensdauer der allokierten Ressourcen ist eng an die Lebensdauer des verwendenden Objekts gebunden.

RAII dient hauptsächlich zur Vermeidung von sogenannten Speicherlecks.

Es ist jedoch wichtig zu beachten, dass RAII keine Lösung für Probleme mit hängenden Zeigern bietet. Wenn Speicher von mehreren Instanzen gemeinsam genutzt und verwaltet wird, kann es im schlimmsten Fall nicht mehr garantiert werden, dass ein Zeiger immer auf gültigen Speicher zeigt. Hierfür stellt das Ownership-Prinzip in Rust Lösungen bereit.

#### Beispiel aus C++

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

#### Rust Ownership & Borrowing
 
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

##### Move Semantics

Rust kopiert den Wert von `n` **nicht** zu `m`. Im Hintergrund wird ein `move` ausgeführt, wodurch der Eigentümer des Speichers hinter `n` zu `m` wechselt. `m` ist es nicht mehr erlaubt auf den Speicher zuzugreifen. Andernfalls wird zur Compile-Zeit ein Fehler geworfen. 

```Rust
fn main() {
    let n = Bob::new(); 
    let m = n;
}
```

##### Borrowing

Speicher kann in Rust geliehen werden. Die Idee dahinter ist, dass Speicher nur für einen kurzen Zeitraum den Eigentümer wechselt. Beispielsweise innerhalb einer Funktion. Repräsentiert wird dies über Referenzen (`&`);

```Rust
fn f(bob: &Bob) {
    // do something
}
fn main() {
    let n = Bob::new();
    f(&n);
}
```
So lange der Inhalt nicht modifiziert werden soll, kann Speicher von vielen Akteuren geliehen werden. Standardmäßig ist geliehener Speicher nicht modifizierbar. Eine Erlaubnis zur Modifizierung von Werten muss extra deklariert werden. 

```Rust
fn f2 (bob: &mut Bob) {
    // modify something
} 
```

In diesem Fall unterbindet Rust allerdings das Leihen von immutablen Speicher. Denn dieser könnte sich jederzeit während des Lesens verändern.


### Closures

#### Closures that borrow

## Closures that Borrow

```rust 
fn sort_by_statistics(cities: $mut Vec<City>, stat: Statistics) {
    cities.sort_by_key(|city| -city.get_statistic(stat));
}
```

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
# Splitter

**Splitter** este un utilitar CLI (Command Line Interface) scris în **Rust**, care permite împărțirea fișierelor mari în bucăți mai mici pentru transfer sau stocare ușoară și reasamblarea lor ulterioară, cu verificarea integrității datelor.

---

## Funcționalități

### Split (Împărțire)

- Preia un fișier mare din folderul `tests/`.
- Împarte fișierul în bucăți de dimensiunea specificată (ex: `10MB`, `500kb`).
- Calculează hash-ul **SHA256** pentru fiecare bucată pentru a garanta integritatea și securitatea datelor.
- Salvează bucățile și un fișier `manifest.json` într-un folder dedicat:  
  `nume_fisier.extensie_parts/`.

### Unsplit (Reasamblare)

- Citește fișierul `manifest.json` pentru a determina ordinea corectă a bucăților.
- Verifică hash-ul fiecărei bucăți înainte de reasamblare.
- Dacă o bucată este coruptă sau modificată, procesul se oprește automat.
- Reface fișierul original în folderul `tests/` cu prefixul `restored_`.

---

## Cum să rulezi proiectul

### Pregătire

Asigură-te că ai **Rust** instalat. Creează un folder `tests` în rădăcina proiectului și adaugă un fișier pentru testare (de exemplu `poza.jpg` sau `test.txt`).

```bash
mkdir tests
# Pune un fișier în folderul tests/
```

---

### Comanda Split

Pentru a tăia un fișier în bucăți de **100KB**:

```bash
cargo run -- split nume_fisier.extensie -s 100kb
```

**Exemplu:**

```bash
cargo run -- split poza.jpg -s 500kb
```

Această comandă va crea folderul:

```text
tests/poza.jpg_parts/
```

care va conține:
- bucățile (`part0001.split`, `part0002.split`, etc.)
- fișierul `manifest.json`

---

### Comanda Unsplit

Pentru a reface fișierul original din bucăți:

```bash
cargo run -- unsplit nume_fisier.extensie
```

**Exemplu:**

```bash
cargo run -- unsplit poza.jpg
```

Aplicația va:
- localiza automat folderul `*_parts/`
- verifica integritatea fiecărei bucăți
- crea fișierul restaurat:

```text
tests/restored_poza.jpg
```

---

## Structura Proiectului

```text
.
├── src/
│   └── main.rs        # Codul sursă principal
├── tests/             # Folder pentru fișierele de intrare/ieșire
├── Cargo.toml         # Configurație și dependențe
└── README.md
```

**Dependențe principale:**
- `clap` – parsarea argumentelor CLI
- `serde` – serializare/deserializare (manifest.json)
- `sha2` – calcul SHA256
- `anyhow` – gestionarea erorilor

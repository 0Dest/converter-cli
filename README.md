# Converter CLI

**Converter CLI** is a simple command-line tool for converting units directly from the terminal. It supports length, weight, volume, and time units.

---

## Supported Units

### Length
- mm, cm, dm, m, km, in (inch), ft (foot), yd (yard), mi (mile)

### Weight
- mg, g, kg, t, oz (ounce), lb (pound), st (stone), cwt (UK hundredweight)

### Volume
- ml, cl, dl, l

### Time
- ns (nanoseconds), us (microseconds), ms (milliseconds), s (seconds), min (minutes), h (hours), d (days)

### Digital Storage
- b (byte), kib (kibibyte), mib (mebibyte), gib (gibibyte), tib (tebibyte) and kb (kilobyte), mb (megabyte), gb (gigabyte), tb (terabyte)

---

## Usage

Run the program with three arguments: `number`, `from unit`, and `to unit`.  

```bash
$ ./target/release/convert 100 cm m
100 cm = 1 m
```

## Building and Running

### Clone the repository
```bash
git clone https://github.com/0Dest/converter-cli.git
cd converter-cli
```
### Build the project in release mode
```bash
cargo build --release
```
### Run the executable

From the **root of the project**, run the binary in `target/release/`:
```bash
./target/release/convert 100 cm m
```
### (Optional) Add to PATH

To run convert from anywhere, add the release directory to your PATH:
```bash
export PATH=$PATH:/path/to/your/project/target/release
```

Now you can simply run:
```bash
convert 100 cm m
```

(ReadMe is generated with ChatGPT, i dont know english very good)
# 🚗 TSP Solver dengan Dynamic Programming  
# PR Strategi Algoritma IF2211

## 📌 Deskripsi  
Program ini adalah **solver** untuk **Traveling Salesman Problem (TSP)** yang menggunakan algoritma **dynamic programming dengan bitmasking** (Held-Karp algorithm) untuk mencari rute optimal.  
Program membaca input dari file `.txt`, melakukan validasi input, menyelesaikan TSP menggunakan pemrograman dinamis, serta menampilkan solusi optimal dalam bentuk teks dan visualisasi grafik `.png`.

## 🛠 Struktur Program
Berikut adalah struktur program tugas kecil ini:
```sh
/PR_IF2211_13523118
├── /src                    # Source code program
│   ├── main.rs            # Program utama dan CLI interface
│   ├── tsp_solver.rs      # Implementasi algoritma Dynamic Programming
│   ├── input_parser.rs    # Parser dan validasi file input
│   └── visualizer.rs      # Generator visualisasi grafik
├── /input                  # Folder file input test case
├── /output                 # Hasil output visualisasi program
├── Cargo.toml             # Konfigurasi dependencies Rust
└── README.md              # Dokumentasi projek
```

## 🌐 Getting Started
Berikut instruksi instalasi dan penggunaan program

### Prerequisites
Pastikan anda sudah memiliki:
- **Rust 1.70 atau lebih baru** (termasuk Cargo)
- **Windows/Linux/macOS** dengan terminal/command prompt

### Installation

1. **Install Rust** (jika belum terinstall)
```bash
# Untuk Windows/Linux/macOS
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Atau download installer dari https://rustup.rs/
```

2. **Clone repository ke dalam suatu folder**
```bash
git clone <repository-url>
```

3. **Pergi ke directory /PR_IF2211_13523118**
```bash
cd PR_IF2211_13523118
```

4. **Build program**
```bash
cargo build --release
```

5. **Jalankan program**
```bash
cargo run -- --input input1.txt
```

## 📌 Cara Penggunaan

### Format Input
File input harus mengikuti salah satu format berikut:

**Format 1: Nama kota di baris pertama, diikuti matriks jarak**
```
A B C D
0 10 15 20
10 0 35 25
15 35 0 30
20 25 30 0
```

**Format 2: Daftar kota per baris, diikuti matriks jarak**
```
Jakarta
Bandung  
Surabaya
Medan
0 150 800 1400
150 0 750 1500
800 750 0 1600
1400 1500 1600 0
```

### Penggunaan Program

1. **Jalankan program** melalui terminal atau command prompt
2. File input **harus** berada di dalam folder `input/` dan output akan disimpan dalam folder `output/`
3. **Gunakan command line interface:**

```bash
# Penggunaan dasar
cargo run -- --input nama_file.txt

# Dengan output kustom
cargo run -- --input test_medium.txt --output solusi_eropa

# Dengan mode verbose (menampilkan detail proses)
cargo run -- --input test_large.txt --verbose

# Melihat bantuan
cargo run -- --help
```

4. Program akan:
   - Membaca dan memvalidasi format input
   - Menyelesaikan TSP menggunakan dynamic programming
   - Menampilkan rute optimal dan total jarak
   - Menyimpan visualisasi ke folder `output/` dengan nama unik

### Contoh Output
![image](https://github.com/user-attachments/assets/c047a8fb-44e2-4d26-998d-3826cc8b24b3)
![image](https://github.com/user-attachments/assets/55d8b624-f072-464d-8a7c-e57a28eae8df)
![image](https://github.com/user-attachments/assets/ced22f50-b6f0-4d66-b94a-bec72af26c88)

## 🧮 Algoritma yang Digunakan

### Dynamic Programming dengan Bitmasking (Held-Karp Algorithm)

**Kompleksitas Waktu:** O(n²2ⁿ)  
**Kompleksitas Ruang:** O(n2ⁿ)

#### Representasi State:
- `dp[mask][i]` = biaya minimum untuk mengunjungi semua kota dalam `mask` dan berakhir di kota `i`
- `mask` adalah bitmask dimana bit ke-j bernilai 1 jika kota j sudah dikunjungi

#### Recurrence Relation:
```
dp[mask][i] = min(dp[mask without i][j] + distance[j][i])
```
untuk semua kota `j` dalam mask saat ini

#### Base Case:
- `dp[{0}][0] = 0` (mulai di kota 0 dengan hanya kota 0 yang dikunjungi)

#### Solusi Akhir:
- `min(dp[all_cities][i] + distance[i][0])` untuk semua kota `i`

## 🎨 Fitur Visualisasi

Program menghasilkan visualisasi grafik yang menampilkan:
- **Posisi kota** dalam bentuk lingkaran dengan label
- **Rute optimal** dengan garis merah dan panah arah
- **Informasi total jarak** di judul grafik
- **Nama file unik** untuk menghindari overwrite

Contoh nama file output:
- `output/tsp_solution.png` (pertama kali)
- `output/tsp_solution_1.png` (kedua kali)
- `output/tsp_solution_2.png` (ketiga kali)

## ⚙️ Konfigurasi dan Batasan

### Batasan Program:
- **Maksimum 20 kota** (karena kompleksitas eksponensial)
- **Minimum 2 kota** untuk masalah TSP yang valid
- **Matriks jarak harus simetrik** dan non-negatif
- **Diagonal matriks harus bernilai 0**

### Error Handling:
- Validasi format file input
- Pengecekan eksistensi file
- Validasi matriks jarak
- Penanganan memory overflow untuk input besar

## ✍️ Author
**👤 Farrel Athalla Putra**  
**NIM 13523118**  
**Kelas K2**

fn main() {
    println!("Hello, world!");
}

/*
    Memory Management

    A. Garbage Collection (GC)
    GC adalah metode manajemen memori otomatis pada bahasa pemograman. GC memimili suatu unit yang disebut dengan garbage collector.
    Collector tersebut aktif memonitor program, dan pada periode atau event tertentu ia akan beruaha untuk mengambil kembali (reclaim) memory
    yang sebelumnya telah di alokasikan dengan catatan memori tersebut sudah tidak lagi digunakan. Proses ini disebut dengan dealokasi memory

    Proses dealokasi pada GC terjadi di belakang layar secara asynchronous
    Beberapa bahasa pemograman yang menerapkan GC di antara adalah Java, C#, GO, Lisp dan lainnya

    B. Automatic Reference Counting (ARC)
    ARC adalah metode manajemen memori yang diterapkan pada bahasa Objective-C dan Swift. Cara ARC memanage memory adalah dengan
    mencatat reference object dan segala aktifitas yang terjadi pada object tersebut

    Di ARC, ada satuan yang disebut dengan retain count yang merupakan representasi jumlah banyaknya variabel atau object yang memegang
    suatu reference. Ketika reference sudah pindah ke luar scope atau dihapus isinya dan dilihat pada catatan rupanya tidak ada variabel
    yang memegang reference tersebut, maka dilakukan proses dealokasi memory.

    Dalam bahasa yang menerapkan ARC, programmer dianjurkan untuk perhatian dan bijak dalam pengalokasian variabel beserta nilainya.
    Mana data yang diperlukan untuk di-retain secara strong dan mana yang tidak, harus pas sesuai dengan kebutuhan.
    Jika tidak hati-hati maka program mempunyai resiko lebih tinggi untuk menemui error deadlocks ataupun memory leaks

    C. Manual memory management
    Manual memory management berarti programmer dibebani secara penuh dalam hal manajemen memori, mengharuskan programmer untuk
    super hati-hati dalam pengalokasian memory, kapan waktunya, di mana alokasinya (apakah heap atau stack), dan kapan harus melakukan operasi
    dealokasi memory.

    Metode manajemen memori ini dipakai dalam system programming contohnya bahasa C dan C++.

    D. Ownership rules
    Manajemen memori yang dilakukan dengan menerapkan konsep ownership beserta aturan-aturannya. Metode manajemen memori ini adalah yang digunakan di Rust.


    # Stack memory
    Data disimpan dalam stack memory dalam bentuk stack. Karakteristik dari stack:
    - Di Rust programming, stack digunakan sebagai default tempat alokasi memori
    - Data yang terakhir masuk adalah yang pertama akan keluar (LIFO)
    - Data yang disimpan diketahui size/ukurannya, dan memiliki batas
    - Alokasi bersifat lokal terhadap pemanggilan fungsi
    - Kecepatan pengaksesan data sangat tinggi

    Data untuk tipe primitif (seperti i32, bool, dll) disimpan di stack

    # Heap memory
    Heap adalah salah satu tempat alokasi memory selain stack. Karakteristik dari heap:
    - Heap digunakan untuk alokasi data yang sifatnya dinamis, tidak diketahui size-nya, atau bisa berubah size-nya
    - Data di heap tidak memiliki pattern tertentu
    - Alokasi dan dealokasi data di heap bisa dilakukan kapanpun
    - Kecepatan pengaksesan data di heap lebih lambat dibanding stack
*/

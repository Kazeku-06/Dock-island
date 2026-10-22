# Product Requirements Document (PRD)

## TopDock --- Dynamic Island Produktivitas untuk Windows

**Versi dokumen:** 1.0\
**Status:** Draft untuk implementasi\
**Platform target:** Windows 10/11, prioritas Windows 11\
**Bahasa antarmuka awal:** Bahasa Indonesia\
**Pendekatan teknis:** Aplikasi desktop native dengan Rust\
**Nama proyek sementara:** TopDock

------------------------------------------------------------------------

## 1. Ringkasan Produk

TopDock adalah aplikasi desktop Windows berbasis Rust yang menyediakan
floating bar kecil di bagian atas layar. Bar ini berfungsi sebagai pusat
akses cepat untuk aplikasi, file, musik, notifikasi, pengingat, dan
daftar tugas.

Konsep interaksinya terinspirasi dari Dynamic Island: antarmuka tetap
kecil saat tidak digunakan, kemudian melebar atau menampilkan panel
kontekstual ketika ada aktivitas atau ketika pengguna berinteraksi
dengannya.

TopDock bukan pengganti taskbar Windows. Aplikasi ini merupakan lapisan
produktivitas tambahan yang selalu mudah dijangkau tanpa harus berpindah
dari pekerjaan aktif.

### 1.1 Masalah yang ingin diselesaikan

Pengguna sering perlu: - Berpindah aplikasi atau membuka folder dan file
berulang kali. - Memeriksa musik yang sedang diputar dan mengontrol
playback. - Mengingat tugas, tenggat waktu, dan agenda kecil sepanjang
hari. - Menyadari notifikasi tanpa kehilangan fokus terlalu lama. -
Menaruh file atau shortcut di tempat yang cepat diakses.

TopDock menggabungkan kebutuhan tersebut ke dalam satu antarmuka ringkas
yang berada di bagian atas layar.

### 1.2 Tujuan produk

1.  Menyediakan akses cepat ke aplikasi, file, folder, dan tindakan
    produktivitas.
2.  Mengurangi gangguan dengan UI yang ringkas dan hanya melebar saat
    diperlukan.
3.  Membaca metadata sesi media Windows yang kompatibel dan menyediakan
    kontrol playback.
4.  Menyediakan to-do list dan reminder lokal yang tetap tersimpan
    setelah aplikasi ditutup.
5.  Mendukung drag-and-drop file dari Windows File Explorer.
6.  Menampilkan notifikasi secara kontekstual dengan kontrol privasi
    yang jelas.
7.  Menjaga konsumsi resource tetap wajar untuk aplikasi yang berjalan
    di background.

### 1.3 Bukan tujuan produk

Versi pertama tidak bertujuan untuk: - Mengganti taskbar atau Start Menu
Windows. - Menjadi pemutar musik atau layanan streaming sendiri. -
Menjamin akses ke seluruh notifikasi dari setiap aplikasi pihak
ketiga. - Menyediakan sinkronisasi cloud, akun pengguna, atau kolaborasi
tim. - Menjalankan file atau perintah secara otomatis tanpa tindakan
pengguna. - Menjadi platform plugin publik pada rilis awal.

------------------------------------------------------------------------

## 2. Pengguna Sasaran

### 2.1 Pengguna utama

Pelajar, developer, dan pekerja desktop yang: - Sering membuka IDE,
terminal, browser, File Explorer, dan aplikasi komunikasi. -
Mendengarkan musik saat bekerja. - Membutuhkan pengingat dan to-do list
sederhana. - Menginginkan antarmuka desktop minimalis dan dapat
dikustomisasi. - Mengutamakan kontrol lokal dan aplikasi yang tidak
terlalu membebani sistem.

### 2.2 Skenario penggunaan utama

**Skenario A --- Sedang coding** Pengguna membuka IDE dan terminal.
TopDock tetap kecil di atas layar. Pengguna mengarahkan cursor ke bar,
lalu memilih shortcut project atau membuka terminal.

**Skenario B --- Musik diputar** Pengguna memutar musik melalui aplikasi
yang kompatibel dengan Windows Media Session. TopDock mendeteksi
metadata yang tersedia, lalu dapat menampilkan judul, artis, status
playback, dan kontrol media.

**Skenario C --- Ada tugas yang mendekati deadline** Pengingat muncul
pada waktu yang ditentukan. Pengguna dapat menandai tugas selesai atau
menundanya tanpa membuka aplikasi penuh.

**Skenario D --- Memindahkan file** Pengguna menyeret file dari File
Explorer ke area drop TopDock. UI menampilkan tindakan yang tersedia,
misalnya membuka file, menyimpan shortcut, atau menambahkannya ke daftar
file cepat.

**Skenario E --- Menerima notifikasi** TopDock menampilkan ringkasan
notifikasi yang berhasil diterima melalui mekanisme integrasi yang
didukung. Pengguna dapat membuka detail atau menonaktifkan tampilan
konten sensitif.

------------------------------------------------------------------------

## 3. Prinsip UX dan Desain

### 3.1 Prinsip utama

-   **Compact by default:** bar sekecil mungkin saat idle.
-   **Expand on intent:** bar membesar saat cursor mendekat, pengguna
    mengeklik, atau event penting terjadi.
-   **Context-aware:** tampilan berubah sesuai aktivitas, misalnya
    musik, reminder, atau notifikasi.
-   **Keyboard-friendly:** tindakan penting dapat dijangkau lewat
    shortcut keyboard.
-   **Non-destructive:** drag-and-drop tidak langsung menghapus atau
    memindahkan file sumber.
-   **Privacy-first:** pengguna mengontrol apakah judul lagu, isi
    notifikasi, atau reminder ditampilkan.
-   **Local-first:** data to-do, reminder, dan konfigurasi disimpan
    secara lokal pada versi awal.

### 3.2 Tata letak

-   Posisi default: tengah atas layar utama.
-   Posisi dapat diubah melalui pengaturan, minimal mendukung tengah,
    kiri, atau kanan.
-   Bar memiliki mode compact dan expanded.
-   Mode compact berisi indikator ringkas dan ikon modul.
-   Mode expanded dapat menampilkan panel musik, reminder, to-do,
    shortcut, atau notifikasi.
-   UI tidak boleh menutupi kontrol penting aplikasi lain secara
    permanen.
-   Dukungan multi-monitor harus dipertimbangkan sejak desain awal.

### 3.3 Interaksi pointer

-   Hover pada bar memicu ekspansi atau animasi icon sesuai preferensi.
-   Hover magnification tidak boleh menyebabkan layout bergetar atau
    berpindah secara tiba-tiba.
-   Klik di luar panel menutup panel sementara, kecuali ada input yang
    belum disimpan.
-   Pengguna dapat menonaktifkan animasi atau mengurangi motion.
-   Bar harus dapat digunakan dengan mouse dan keyboard.

### 3.4 Aksesibilitas

-   Kontras teks dan ikon harus memadai.
-   Status tidak boleh hanya dibedakan melalui warna.
-   Ukuran target klik harus nyaman.
-   Animasi dapat dikurangi atau dimatikan.
-   Kontrol inti memiliki label aksesibilitas yang sesuai.
-   Jangan mengandalkan hover sebagai satu-satunya cara membuka fitur.

------------------------------------------------------------------------

## 4. Ruang Lingkup Fitur

### Prioritas

-   **P0 --- Wajib untuk MVP:** tanpa ini produk belum memenuhi fungsi
    inti.
-   **P1 --- Penting:** masuk setelah fondasi MVP stabil.
-   **P2 --- Pengembangan lanjutan:** ditunda sampai kebutuhan dan
    integrasi jelas.

### 4.1 Floating bar dan window management --- P0

**Deskripsi:** Floating bar native yang berada di bagian atas layar,
tanpa chrome window standar.

**Persyaratan:** - Aplikasi berjalan sebagai window desktop native. -
Window mendukung posisi top-of-screen dan konfigurasi alignment. - Mode
compact dan expanded tersedia. - Window dapat tetap berada di atas
window biasa sesuai pengaturan. - Aplikasi mendukung minimize-to-tray
atau berjalan di background. - Klik area luar dapat menutup panel yang
sedang terbuka. - Window tidak boleh merebut fokus secara tidak perlu
saat event otomatis terjadi. - Aplikasi menangani perubahan resolusi dan
perpindahan monitor dengan aman. - Pengguna dapat memilih monitor target
jika tersedia beberapa monitor. - Perilaku saat aplikasi fullscreen
harus dapat dikonfigurasi atau mengikuti batasan Windows secara
eksplisit. - Pengguna dapat menonaktifkan always-on-top jika mengganggu.

**Kriteria penerimaan:** - Bar muncul di bagian atas layar setelah
aplikasi dijalankan. - Bar tidak memiliki border/title bar standar yang
terlihat. - Bar tetap pada posisi yang benar setelah resolusi berubah. -
Panel dapat dibuka dan ditutup tanpa membuat aplikasi aktif kehilangan
input secara tidak sengaja. - Aplikasi dapat ditutup secara normal dari
menu tray atau kontrol aplikasi.

### 4.2 Animasi dan hover magnification --- P0

**Persyaratan:** - Bar memiliki ukuran compact saat idle. - Hover memicu
transisi menuju tampilan expanded atau pembesaran ikon. - Event musik,
reminder, atau notifikasi dapat memicu ekspansi singkat berdasarkan
pengaturan. - Animasi harus memiliki durasi dan easing yang konsisten. -
Terdapat opsi untuk mematikan animasi atau mengaktifkan reduced
motion. - Animasi tidak boleh berjalan terus-menerus saat tidak
dibutuhkan.

**Kriteria penerimaan:** - Transisi compact ke expanded tidak
menyebabkan flicker yang terlihat. - Mouse tetap dapat mengakses kontrol
selama animasi. - Pengguna dapat mematikan efek pembesaran melalui
Settings.

### 4.3 Launcher aplikasi, file, dan folder --- P0

**Deskripsi:** Shortcut untuk membuka aplikasi, file, dan folder.

**Persyaratan:** - Pengguna dapat menambahkan shortcut melalui pemilih
file Windows atau drag-and-drop. - Pengguna dapat menghapus dan
mengurutkan shortcut. - Shortcut menyimpan target yang valid dan label
tampilan. - Klik shortcut membuka target melalui mekanisme Windows yang
sesuai. - Jika target sudah tidak tersedia, tampilkan pesan yang jelas
dan opsi memperbaiki atau menghapus shortcut. - Jangan menjalankan
perintah shell dari input pengguna tanpa validasi dan konfirmasi. -
Pengguna dapat memilih apakah shortcut ditampilkan di compact bar atau
hanya di expanded panel.

**Kriteria penerimaan:** - Shortcut yang valid dapat dibuka. - Shortcut
rusak tidak membuat aplikasi crash. - Urutan shortcut tetap tersimpan
setelah aplikasi dimulai ulang.

### 4.4 Drag-and-drop file --- P0

**Deskripsi:** Pengguna dapat menyeret file atau folder dari File
Explorer ke TopDock.

**Persyaratan:** - Terima path file/folder lokal yang diberikan melalui
mekanisme drag-and-drop Windows. - Berikan indikator visual saat pointer
berada di atas target drop yang valid. - Setelah file dijatuhkan,
tampilkan menu aksi yang jelas. - Aksi MVP: 1. Buka file/folder. 2.
Simpan sebagai shortcut. 3. Tambahkan ke daftar akses cepat atau daftar
kerja. 4. Batal. - Operasi drop tidak boleh memindahkan atau menghapus
file sumber secara default. - Tangani path tidak valid, file yang sudah
dihapus, dan file yang tidak dapat diakses. - Tangani beberapa file
dalam satu drop dengan batas yang wajar. - Jangan mengeksekusi file
secara otomatis hanya karena file dijatuhkan. - Jika aksi dapat mengubah
file sumber atau lokasi file, wajib minta konfirmasi eksplisit.

**Kriteria penerimaan:** - File yang dijatuhkan memicu menu aksi, bukan
langsung dieksekusi. - Menyimpan shortcut tidak menggandakan atau
memindahkan file. - File yang tidak didukung menghasilkan pesan yang
dapat dipahami. - Pengguna dapat membatalkan aksi tanpa efek samping.

### 4.5 Deteksi musik dan kontrol media --- P0

**Deskripsi:** Mendeteksi sesi media yang didukung Windows dan
menampilkan informasi musik.

**Persyaratan:** - Menggunakan Windows Global System Media Transport
Controls (GSMTC) atau API Windows Runtime yang sesuai. - Membaca
metadata yang tersedia, misalnya judul, artis, album, artwork, status
playback, dan posisi media jika tersedia. - Mendukung perubahan sesi
aktif jika beberapa aplikasi media digunakan. - Menyediakan kontrol
play/pause dan next/previous jika sesi menyediakan command terkait. -
Jika tidak ada sesi media yang kompatibel, tampilkan keadaan kosong yang
informatif. - Metadata yang tidak tersedia tidak boleh diganti dengan
data rekaan. - Pengguna dapat menonaktifkan deteksi media. - Aplikasi
tidak boleh mengklaim kompatibilitas dengan setiap pemutar musik sebelum
diuji.

**Kriteria penerimaan:** - Metadata diperbarui ketika lagu berubah, jika
disediakan oleh sesi. - Kontrol media hanya aktif ketika didukung sesi
yang dipilih. - Tidak adanya metadata tidak menyebabkan crash. -
Perubahan sesi media ditangani tanpa harus memulai ulang aplikasi.

### 4.6 Tampilan lirik --- P1

**Deskripsi:** Menampilkan lirik lagu yang cocok dengan sesi media,
termasuk sinkronisasi waktu jika tersedia.

**Persyaratan:** - Menggunakan sumber lirik yang memiliki API dan izin
penggunaan yang sesuai. - Mencocokkan lagu berdasarkan metadata seperti
judul dan artis. - Mendukung lirik tersinkronisasi jika sumber
menyediakannya. - Jika hanya lirik biasa tersedia, tampilkan tanpa
animasi sinkronisasi. - Menampilkan keadaan "lirik tidak tersedia" jika
pencarian gagal. - Menyediakan pengaturan untuk menonaktifkan pencarian
lirik. - Hindari menyimpan lirik secara permanen kecuali diizinkan oleh
ketentuan sumber. - Menangani rate limit, koneksi gagal, dan metadata
yang tidak lengkap. - Tidak menjanjikan dukungan untuk semua lagu atau
layanan streaming.

**Kriteria penerimaan:** - Lirik hanya ditampilkan ketika hasil
pencocokan cukup meyakinkan. - Kegagalan layanan lirik tidak memengaruhi
kontrol musik. - UI tidak menampilkan lirik lagu sebelumnya setelah sesi
berubah tanpa memberi status pemuatan yang jelas.

### 4.7 To-do list --- P0

**Deskripsi:** Daftar tugas lokal yang dapat diakses langsung dari
TopDock.

**Persyaratan:** - Tambah, edit, tandai selesai, dan hapus tugas. -
Setiap tugas minimal memiliki judul. - Tugas dapat memiliki deskripsi
opsional, prioritas, deadline, dan status selesai. - Mendukung filter
semua, aktif, dan selesai. - Mendukung pencarian sederhana jika jumlah
tugas cukup besar. - Data disimpan di SQLite lokal. - Perubahan
tersimpan secara konsisten. - Penghapusan tugas harus dapat dibatalkan
setidaknya melalui konfirmasi atau mekanisme undo yang sesuai. - Tugas
selesai tidak dihapus otomatis tanpa pengaturan.

**Kriteria penerimaan:** - Tugas tetap tersedia setelah aplikasi ditutup
dan dibuka kembali. - Status selesai tersimpan dengan benar. - Input
kosong ditolak dengan pesan yang jelas. - Kegagalan database ditangani
tanpa kehilangan data diam-diam.

### 4.8 Reminder dan pengingat --- P0

**Deskripsi:** Mengingatkan pengguna tentang tugas atau pengingat
mandiri pada waktu tertentu.

**Persyaratan:** - Membuat reminder dengan judul, waktu, dan pesan
opsional. - Reminder dapat terhubung ke to-do atau berdiri sendiri. -
Mendukung reminder satu kali. - Mendukung pengulangan dasar, minimal
harian dan mingguan, jika implementasi MVP memungkinkan tanpa menunda
fitur inti. - Mendukung tindakan "Selesai", "Tunda", dan "Buka". -
Durasi snooze dapat dikonfigurasi, dengan pilihan awal 5, 10, dan 15
menit. - Reminder disimpan secara persisten di SQLite. - Reminder yang
sudah jatuh tempo saat aplikasi tidak berjalan harus diperiksa ketika
aplikasi dimulai lagi. - Perilaku saat perangkat sleep, aplikasi tidak
aktif, atau Windows restart harus dijelaskan secara eksplisit. - Gunakan
notifikasi Windows yang sesuai untuk menampilkan reminder. - Pengguna
dapat mengaktifkan atau menonaktifkan suara jika implementasi
mendukungnya. - Pengguna dapat mengatur jam tenang atau quiet hours. -
Reminder yang terlambat tidak boleh memunculkan ratusan notifikasi
sekaligus; lakukan pengelompokan atau aturan catch-up yang jelas. - Zona
waktu dan perubahan jam sistem harus ditangani dengan aman.

**Kriteria penerimaan:** - Reminder muncul pada waktu yang ditentukan
ketika aplikasi dan mekanisme notifikasi tersedia. - Reminder yang
terlewat diperiksa setelah aplikasi dimulai kembali. - Tombol snooze
menjadwalkan ulang reminder dengan benar. - Reminder yang sudah selesai
tidak muncul kembali. - Kegagalan menampilkan notifikasi tidak menghapus
reminder dari database.

**Catatan implementasi:** Ketepatan waktu tidak dapat dijamin ketika
Windows mati atau perangkat sleep. Aplikasi harus menjelaskan perilaku
catch-up saat sistem aktif kembali. Untuk reminder yang harus tetap
bekerja saat aplikasi ditutup, gunakan mekanisme background/startup atau
penjadwalan Windows yang sesuai; jangan hanya mengandalkan timer dalam
UI.

### 4.9 Notifikasi --- P1

**Deskripsi:** Menampilkan event dan notifikasi secara ringkas di bar.

**Jenis notifikasi:** - Notifikasi internal TopDock, seperti reminder. -
Status musik atau perubahan sesi media. - Notifikasi aplikasi pihak
ketiga jika tersedia melalui integrasi Windows yang didukung.

**Persyaratan:** - Notifikasi internal harus dapat ditampilkan tanpa
mengandalkan integrasi pihak ketiga. - Untuk notifikasi aplikasi lain,
lakukan validasi teknis terhadap API dan batasan izin Windows sebelum
menjanjikan fitur. - Jangan mengasumsikan WinRT Toast API dapat membaca
seluruh notifikasi yang diterima Windows. - Jika integrasi membutuhkan
Notification Listener atau akses khusus, jelaskan izin, konfigurasi, dan
keterbatasannya. - Pengguna dapat mengaktifkan/menonaktifkan kategori
notifikasi. - Pengguna dapat menyembunyikan isi sensitif dan hanya
menampilkan nama aplikasi atau indikator. - Notifikasi berulang dapat
dikelompokkan. - Pengguna dapat menutup notifikasi atau membuka tindakan
yang didukung. - Jangan membaca atau menyimpan isi notifikasi tanpa
persetujuan dan kebutuhan yang jelas.

**Kriteria penerimaan:** - Notifikasi internal berfungsi meski integrasi
eksternal tidak dikonfigurasi. - Jika akses notifikasi pihak ketiga
tidak tersedia, aplikasi menjelaskan batasan tersebut. - Pengaturan
privasi dihormati secara konsisten.

### 4.10 System tray dan startup --- P0

**Persyaratan:** - Menyediakan ikon system tray. - Menu tray minimal:
buka/tutup panel, pengaturan, nonaktifkan sementara, dan keluar. -
Menyediakan opsi mulai otomatis saat login Windows. - Startup otomatis
harus opt-in atau dijelaskan dengan jelas saat onboarding. - Keluar dari
aplikasi benar-benar menghentikan proses background, kecuali pengguna
memilih mode tertentu. - Aplikasi tidak boleh membuat beberapa instance
tanpa penanganan yang jelas.

**Kriteria penerimaan:** - Pengguna dapat mengakses aplikasi dari system
tray. - Preferensi startup tersimpan. - Menjalankan aplikasi dua kali
tidak membuat beberapa dock bertumpuk.

### 4.11 Pengaturan --- P1

Pengaturan minimal: - Posisi horizontal: kiri, tengah, kanan. - Monitor
target. - Ukuran compact. - Perilaku expand dan hover. - Durasi
animasi. - Reduced motion. - Always-on-top. - Perilaku saat
fullscreen. - Modul yang ditampilkan. - Preferensi musik dan lirik. -
Preferensi notifikasi dan privasi. - Quiet hours. - Startup saat
login. - Tema gelap/terang atau mengikuti sistem.

------------------------------------------------------------------------

## 5. Alur Pengguna Utama

### 5.1 Pertama kali menjalankan aplikasi

1.  Pengguna membuka TopDock.
2.  Aplikasi menampilkan onboarding ringkas.
3.  Pengguna memilih posisi dock dan preferensi startup.
4.  Aplikasi meminta izin atau konfigurasi tambahan hanya ketika fitur
    terkait membutuhkan akses.
5.  Bar muncul dalam mode compact.
6.  Pengguna dapat membuka Settings untuk mengatur shortcut, reminder,
    dan modul.

### 5.2 Menambahkan file ke dock

1.  Pengguna menyeret file dari File Explorer ke TopDock.
2.  Dock menunjukkan indikator drop.
3.  Pengguna melepas file.
4.  Menu aksi tampil: Buka, Simpan shortcut, Tambah ke akses cepat, atau
    Batal.
5.  Pengguna memilih aksi.
6.  Aplikasi memvalidasi path dan menyimpan perubahan jika diperlukan.
7.  Hasil aksi diberitahukan secara ringkas.

### 5.3 Membuat reminder

1.  Pengguna membuka panel reminder.
2.  Pengguna memasukkan judul dan waktu.
3.  Pengguna mengatur pengulangan jika diperlukan.
4.  Aplikasi memvalidasi input dan menyimpan reminder.
5.  Scheduler memantau waktu reminder melalui mekanisme background yang
    sesuai.
6.  Pada waktu yang ditentukan, TopDock menampilkan event dan/atau
    Windows notification.
7.  Pengguna memilih Selesai, Tunda, atau Buka.

### 5.4 Menggunakan musik dan lirik

1.  Pengguna memutar lagu pada pemutar media yang kompatibel.
2.  TopDock mendeteksi atau memperbarui sesi media.
3.  Panel musik menampilkan metadata yang tersedia.
4.  Aplikasi mencari lirik jika fitur diaktifkan.
5.  Jika lirik tersinkronisasi tersedia, bar memperbarui baris sesuai
    waktu.
6.  Jika pencarian gagal, kontrol musik tetap berfungsi dan UI
    menampilkan status yang sesuai.

### 5.5 Menangani notifikasi

1.  Event diterima melalui sumber yang didukung.
2.  TopDock memeriksa pengaturan kategori dan privasi.
3.  UI menampilkan ringkasan sesuai preferensi.
4.  Pengguna dapat menutup atau menjalankan tindakan yang tersedia.
5.  Event yang tidak dapat ditindaklanjuti hanya menampilkan informasi,
    bukan tombol palsu.

------------------------------------------------------------------------

## 6. Persyaratan Fungsional Ringkas

  ID       Persyaratan                                            Prioritas
  -------- ------------------------------------------------------ -----------
  FR-001   Floating bar native di bagian atas layar               P0
  FR-002   Mode compact dan expanded                              P0
  FR-003   Hover magnification dan animasi                        P0
  FR-004   Launcher aplikasi, file, dan folder                    P0
  FR-005   Drag-and-drop dengan menu aksi                         P0
  FR-006   Deteksi sesi media Windows yang kompatibel             P0
  FR-007   Kontrol playback yang didukung sesi                    P0
  FR-008   To-do list dengan penyimpanan lokal                    P0
  FR-009   Reminder persisten dan snooze                          P0
  FR-010   System tray dan keluar yang benar                      P0
  FR-011   Startup otomatis opsional                              P0
  FR-012   Lirik dari sumber eksternal                            P1
  FR-013   Notifikasi pihak ketiga melalui integrasi yang valid   P1
  FR-014   Pengaturan lengkap untuk UI dan privasi                P1
  FR-015   Dukungan multi-monitor yang matang                     P1
  FR-016   Pengulangan reminder lanjutan dan quiet hours          P1
  FR-017   Riwayat notifikasi atau aktivitas                      P2
  FR-018   Sinkronisasi cloud                                     P2

------------------------------------------------------------------------

## 7. Persyaratan Nonfungsional

### 7.1 Performa

-   Penggunaan CPU saat idle harus rendah dan tidak melakukan polling
    agresif tanpa kebutuhan.
-   Hindari loop rendering tanpa batas ketika tidak ada perubahan UI.
-   Perubahan metadata media dan timer harus berbasis event jika API
    memungkinkan.
-   Target awal: UI tetap responsif pada komputer Windows kelas
    menengah.
-   Penggunaan memori harus diukur melalui profiling; jangan menetapkan
    angka final sebelum prototype diuji.

### 7.2 Stabilitas

-   Kegagalan layanan lirik atau media tidak boleh menyebabkan dock
    utama crash.
-   Kesalahan database harus ditangani dan dilaporkan.
-   Aplikasi harus pulih dari monitor disconnect, resolusi berubah, dan
    sesi media berakhir.
-   Logging tidak boleh menyimpan konten sensitif secara default.

### 7.3 Keamanan dan privasi

-   Simpan data lokal di direktori data aplikasi pengguna.
-   Jangan mengunggah to-do, reminder, atau file path ke server
    eksternal.
-   Jaringan hanya digunakan untuk layanan yang memang memerlukannya,
    seperti pencarian lirik.
-   Jelaskan sumber data dan kebijakan penyimpanannya.
-   Jangan menjalankan file atau command secara otomatis dari drop.
-   Validasi path, argumen, dan input eksternal.
-   Hindari menyimpan kredensial API dalam source code.
-   Berikan kontrol untuk menonaktifkan integrasi online.
-   Isi notifikasi sensitif harus bisa disembunyikan.

### 7.4 Kompatibilitas

-   Target utama: Windows 11 64-bit.
-   Windows 10 dapat dipertimbangkan jika API yang diperlukan tersedia
    dan sudah diuji.
-   Versi minimum Windows harus diputuskan berdasarkan API native dan
    dependency yang dipilih.
-   Fitur yang tidak didukung pada versi tertentu harus dinonaktifkan
    dengan pesan yang jelas.

### 7.5 Reliabilitas reminder

-   Reminder harus tersimpan sebelum UI mengonfirmasi bahwa reminder
    berhasil dibuat.
-   Saat startup, sistem memeriksa reminder yang terlewat.
-   Aturan penanganan reminder terlewat harus konsisten dan
    terdokumentasi.
-   Perubahan zona waktu dan jam sistem harus diuji.

------------------------------------------------------------------------

## 8. Rekomendasi Arsitektur Teknis

### 8.1 Prinsip

Aplikasi menggunakan Rust sebagai bahasa implementasi utama. Tidak
menggunakan React, Electron, atau Tauri.

### 8.2 Komponen yang disarankan

  -----------------------------------------------------------------------
  Komponen                            Peran
  ----------------------------------- -----------------------------------
  `windows` crate                     Akses API Win32 dan Windows Runtime

  `winit`                             Window creation dan event loop,
                                      jika cocok dengan kebutuhan native

  `wgpu`                              Rendering UI kustom jika dipilih

  Direct2D/DirectComposition          Alternatif rendering native untuk
                                      efek visual Windows

  `rusqlite`                          Database SQLite

  `chrono` atau tipe waktu yang       Representasi waktu dan tanggal
  sesuai                              

  `serde`                             Serialisasi konfigurasi

  `tracing`                           Logging dan diagnostik

  Windows Toast API                   Menampilkan notifikasi Windows

  Windows GSMTC / WinRT media APIs    Metadata dan kontrol sesi media
  -----------------------------------------------------------------------

**Catatan:** Pemilihan `winit` + `wgpu` versus Win32 +
Direct2D/DirectComposition harus diputuskan melalui prototype kecil.
Jangan mencampur beberapa sistem windowing/rendering tanpa kebutuhan
jelas. Fitur seperti click-through, always-on-top, acrylic/blur, dan
integrasi drag-and-drop perlu diuji pada Windows target.

### 8.3 Struktur modul yang diusulkan

``` text
topdock/
├── Cargo.toml
├── src/
│   ├── main.rs
│   ├── app.rs
│   ├── config/
│   │   ├── mod.rs
│   │   └── settings.rs
│   ├── ui/
│   │   ├── mod.rs
│   │   ├── window.rs
│   │   ├── state.rs
│   │   ├── animation.rs
│   │   └── components/
│   ├── platform/
│   │   └── windows/
│   │       ├── mod.rs
│   │       ├── window_manager.rs
│   │       ├── drag_drop.rs
│   │       ├── media_session.rs
│   │       ├── toast.rs
│   │       ├── startup.rs
│   │       └── tray.rs
│   ├── features/
│   │   ├── launcher/
│   │   ├── reminders/
│   │   ├── todo/
│   │   ├── music/
│   │   ├── lyrics/
│   │   └── notifications/
│   ├── storage/
│   │   ├── mod.rs
│   │   ├── database.rs
│   │   └── migrations.rs
│   └── services/
│       ├── scheduler.rs
│       └── lyrics_provider.rs
├── migrations/
├── assets/
├── tests/
└── README.md
```

Struktur ini adalah rancangan awal. Modul boleh disederhanakan selama
tanggung jawabnya tetap terpisah dan testable.

### 8.4 Penyimpanan data

Gunakan SQLite untuk data terstruktur.

Tabel awal yang disarankan:

**`todos`** - `id` - `title` - `description` - `priority` - `due_at` -
`completed_at` - `created_at` - `updated_at`

**`reminders`** - `id` - `todo_id` nullable - `title` - `message` -
`scheduled_at` - `recurrence_rule` nullable - `status` - `snooze_until`
nullable - `created_at` - `updated_at`

**`shortcuts`** - `id` - `label` - `target_path` - `item_type` -
`sort_order` - `created_at`

**`settings`** - `key` - `value` - `updated_at`

**`notification_preferences`** - `category` - `enabled` -
`hide_content` - `updated_at`

Simpan waktu secara konsisten, misalnya sebagai UTC untuk timestamp
absolut, lalu konversi ke zona waktu lokal untuk tampilan. Aturan untuk
reminder berulang perlu menyimpan informasi zona waktu atau definisi
jadwal lokal yang cukup agar perilakunya tidak berubah secara
mengejutkan.

------------------------------------------------------------------------

## 9. Integrasi Windows dan Batasan yang Diketahui

### 9.1 Media session

GSMTC dapat menyediakan metadata dan kontrol untuk sesi media yang
kompatibel. Dukungan berbeda menurut aplikasi pemutar dan informasi yang
dipublikasikan oleh sesi.

### 9.2 Notifikasi pihak ketiga

Windows Toast API untuk mengirim notifikasi tidak otomatis memberikan
akses untuk membaca semua notifikasi aplikasi lain. Integrasi listener
memiliki batasan dan dapat memerlukan konfigurasi atau izin khusus.
Fitur ini harus diprototipe dan divalidasi sebelum masuk ke komitmen
rilis.

### 9.3 Drag-and-drop

Implementasi native harus menangani format data drop Windows dan path
yang valid. Drag-and-drop lintas hak akses/integrity level dapat
memiliki batasan. Jangan menjalankan file secara otomatis.

### 9.4 Background dan startup

Aplikasi desktop yang hanya berjalan sebagai window tidak cukup untuk
menjamin reminder ketika window ditutup. Tentukan apakah aplikasi tetap
berjalan di tray, menggunakan background process, atau mendaftarkan
tugas terjadwal Windows. Pastikan pengguna memiliki kontrol untuk keluar
sepenuhnya.

### 9.5 Lirik

Lirik membutuhkan provider terpisah. Pilih provider berdasarkan API,
lisensi, rate limit, kualitas pencocokan, dan kebijakan penyimpanan.
Jangan menganggap semua lirik dapat diambil secara gratis atau disimpan
tanpa batas.

------------------------------------------------------------------------

## 10. Model Status UI

### 10.1 Status utama

-   `CompactIdle`: bar kecil, tidak ada panel aktif.
-   `Hovered`: cursor berada di area bar, ikon dapat membesar.
-   `Expanded`: panel dibuka oleh hover atau klik.
-   `MusicActive`: sesi media tersedia.
-   `ReminderAlert`: reminder perlu ditampilkan.
-   `NotificationPreview`: notifikasi yang diizinkan sedang dipratinjau.
-   `DropTarget`: file sedang diseret di atas area drop.
-   `SettingsOpen`: panel pengaturan terbuka.

### 10.2 Prioritas event

Jika beberapa event terjadi bersamaan, gunakan prioritas yang dapat
diprediksi. Rekomendasi awal: 1. Input aktif pengguna dan dialog
konfirmasi. 2. Reminder yang jatuh tempo. 3. Preview notifikasi yang
diizinkan. 4. Interaksi musik. 5. Hover/animasi biasa.

Jangan mengganti panel yang sedang diedit pengguna secara tiba-tiba.
Event baru dapat masuk ke antrean atau ditampilkan sebagai indikator
kecil.

------------------------------------------------------------------------

## 11. Metrik Keberhasilan

Metrik berikut merupakan target evaluasi awal, bukan klaim performa yang
sudah terbukti.

-   Pengguna dapat membuka shortcut dari bar dengan cepat tanpa membuka
    menu tambahan.
-   Reminder yang disimpan tetap tersedia setelah restart aplikasi.
-   Tidak ada kehilangan data to-do yang diketahui dalam pengujian
    normal.
-   Drag-and-drop menampilkan aksi yang benar dan tidak mengubah file
    sumber tanpa izin.
-   Metadata media diperbarui untuk sesi yang kompatibel.
-   Aplikasi tidak menghasilkan notifikasi berulang yang tidak
    terkendali.
-   UI tetap responsif saat aplikasi idle dan saat panel dibuka.
-   Pengguna dapat menonaktifkan modul yang tidak dibutuhkan.

Pengukuran performa dan konsumsi resource dilakukan pada prototype dan
build release di beberapa konfigurasi Windows.

------------------------------------------------------------------------

## 12. Strategi Pengujian

### 12.1 Unit test

-   Validasi data to-do.
-   Perhitungan waktu reminder dan snooze.
-   Aturan reminder berulang.
-   Migrasi database.
-   Validasi konfigurasi dan shortcut.
-   Prioritas event UI.

### 12.2 Integration test

-   CRUD to-do dan reminder melalui SQLite.
-   Startup dan pemulihan reminder yang terlewat.
-   Pembaruan metadata sesi media.
-   Pengiriman notifikasi internal.
-   Penanganan kegagalan provider lirik.
-   Penanganan target shortcut yang tidak lagi tersedia.

### 12.3 Manual test di Windows

-   Multi-monitor dan perubahan resolusi.
-   Taskbar berada di atas atau menggunakan auto-hide.
-   Aplikasi berjalan dengan skala DPI berbeda.
-   Mode fullscreen.
-   Sleep/wake dan restart Windows.
-   Drag-and-drop satu file dan beberapa file.
-   File dengan nama/path non-ASCII.
-   Sesi musik berhenti atau berganti.
-   Notifikasi saat aplikasi sedang digunakan.
-   Startup otomatis aktif dan nonaktif.
-   Keluar total melalui tray.

### 12.4 Uji keamanan dasar

-   Drop file tidak mengeksekusi file otomatis.
-   Path tidak valid tidak menyebabkan crash.
-   Input reminder tidak dapat memicu command shell.
-   Log tidak menyimpan isi notifikasi sensitif secara default.
-   Kegagalan jaringan tidak merusak data lokal.

------------------------------------------------------------------------

## 13. Tahapan Implementasi

### Fase 0 --- Validasi teknis

-   Buat proof of concept floating window native.
-   Uji top positioning, transparency, always-on-top, hover, dan
    drag-and-drop.
-   Uji pendekatan rendering yang dipilih.
-   Uji akses GSMTC dari Rust.
-   Dokumentasikan keterbatasan Windows dan versi minimum.

**Output:** keputusan arsitektur yang tervalidasi.

### Fase 1 --- MVP dock

-   Floating bar dan panel expanded.
-   Animasi hover.
-   System tray.
-   Launcher shortcut.
-   Drag-and-drop dengan menu aksi.
-   Pengaturan dasar.

**Output:** dock native yang bisa dipakai sehari-hari.

### Fase 2 --- Produktivitas

-   To-do list.
-   Reminder persisten.
-   Snooze dan notifikasi internal.
-   SQLite dan migrasi database.
-   Penanganan reminder terlewat saat startup.

**Output:** fitur produktivitas lokal berfungsi.

### Fase 3 --- Media

-   Deteksi sesi media.
-   Metadata lagu dan artwork jika tersedia.
-   Kontrol playback.
-   Integrasi provider lirik.
-   Tampilan lirik biasa dan tersinkronisasi jika tersedia.

**Output:** panel musik kontekstual.

### Fase 4 --- Notifikasi dan pemolesan

-   Validasi integrasi notifikasi pihak ketiga.
-   Kontrol privasi.
-   Multi-monitor dan DPI.
-   Reduced motion.
-   Optimasi performa.
-   Pengujian installer dan startup.

**Output:** kandidat rilis yang stabil.

### Fase 5 --- Fitur lanjutan

-   Riwayat aktivitas.
-   Widget tambahan.
-   Shortcut keyboard lanjutan.
-   Sinkronisasi cloud hanya jika dibutuhkan dan dirancang dengan
    keamanan yang memadai.

------------------------------------------------------------------------

## 14. Risiko dan Mitigasi

  -----------------------------------------------------------------------
  Risiko                  Dampak                  Mitigasi
  ----------------------- ----------------------- -----------------------
  API native dan          Pengembangan lebih lama Validasi pada Fase 0
  rendering lebih                                 sebelum membangun semua
  kompleks dari perkiraan                         fitur

  Tidak semua pemutar     Panel musik tidak       Tampilkan status yang
  menyediakan metadata    lengkap                 jelas dan uji aplikasi
  media                                           target

  Akses notifikasi pihak  Fitur tidak universal   Jadikan integrasi
  ketiga terbatas                                 sebagai P1 dan validasi
                                                  API/izin terlebih
                                                  dahulu

  Sumber lirik tidak      Lirik gagal ditampilkan Gunakan provider yang
  stabil atau dibatasi                            sesuai ketentuan dan
  lisensi                                         isolasikan kegagalannya

  Reminder terlewat saat  Pengingat tidak muncul  Persistensi database
  perangkat mati          tepat waktu             dan aturan catch-up
                                                  saat startup

  Floating bar mengganggu Pengalaman buruk        Pengaturan fullscreen
  aplikasi fullscreen                             dan pengujian perilaku
                                                  window

  Penggunaan CPU          Mengganggu kerja        Event-driven updates,
  berlebihan                                      kurangi polling, dan
                                                  profiling

  Path shortcut berubah   Shortcut rusak          Validasi saat dibuka
                                                  dan sediakan
                                                  perbaikan/penghapusan
  -----------------------------------------------------------------------

------------------------------------------------------------------------

## 15. Keputusan yang Perlu Ditutup Sebelum Rilis

Keputusan ini tidak menghalangi pembuatan prototype, tetapi perlu
ditentukan sebelum implementasi final: 1. Versi minimum Windows yang
didukung. 2. Renderer final: `winit` + `wgpu` atau Win32 +
Direct2D/DirectComposition. 3. Provider lirik dan ketentuan
penggunaannya. 4. Mekanisme resmi yang layak untuk membaca notifikasi
pihak ketiga. 5. Kebijakan reminder saat aplikasi ditutup dan ketika
Windows restart. 6. Perilaku dock pada fullscreen dan multi-monitor. 7.
Apakah semua data hanya lokal untuk versi pertama.

Default yang disarankan untuk MVP: Windows 11 64-bit, data lokal, tidak
perlu akun, integrasi lirik opsional, notifikasi internal sebagai
baseline, dan integrasi notifikasi pihak ketiga hanya setelah validasi.

------------------------------------------------------------------------

## 16. Definisi Selesai (Definition of Done)

MVP dinyatakan selesai jika: - \[ \] Floating bar native muncul di
bagian atas layar. - \[ \] Mode compact dan expanded bekerja. - \[ \]
Hover dan animasi bisa diatur atau dinonaktifkan. - \[ \] Shortcut
aplikasi/file/folder dapat ditambah, dibuka, diurutkan, dan dihapus. -
\[ \] Drag-and-drop memunculkan menu aksi dan tidak menjalankan file
otomatis. - \[ \] Sesi media yang kompatibel dapat dideteksi. - \[ \]
Kontrol media yang tersedia dapat digunakan. - \[ \] To-do dapat dibuat,
diedit, diselesaikan, dan disimpan lokal. - \[ \] Reminder dapat dibuat,
dijadwalkan, ditunda, dan diselesaikan. - \[ \] Reminder yang terlewat
ditangani setelah aplikasi dimulai kembali. - \[ \] System tray
menyediakan kontrol aplikasi yang jelas. - \[ \] Startup otomatis dapat
diaktifkan dan dinonaktifkan. - \[ \] Kesalahan layanan eksternal tidak
menjatuhkan aplikasi utama. - \[ \] Pengujian manual dasar Windows telah
dilakukan. - \[ \] Dokumentasi instalasi, konfigurasi, batasan, dan
pengembangan tersedia.

------------------------------------------------------------------------

## 17. Ringkasan MVP

Prioritas awal adalah membangun fondasi yang stabil, bukan memasukkan
semua fitur sekaligus.

**MVP TopDock:** 1. Floating bar native di atas layar. 2.
Compact/expanded UI dengan hover animation. 3. Launcher shortcut. 4.
Drag-and-drop file dengan pilihan aksi yang aman. 5. To-do list lokal.
6. Reminder persisten dengan snooze. 7. Deteksi sesi media Windows dan
kontrol playback dasar. 8. System tray dan startup opsional.

Lirik tersinkronisasi dan pembacaan notifikasi pihak ketiga masuk
setelah fondasi inti stabil, karena keduanya bergantung pada provider,
API, dan batasan platform yang perlu divalidasi.

Tujuan akhir: satu aplikasi Windows native berbasis Rust yang kecil,
cepat, kontekstual, dan membantu pekerjaan sehari-hari tanpa menjadi
sumber gangguan baru.

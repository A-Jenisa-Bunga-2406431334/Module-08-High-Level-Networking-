# Module 8 - High Level Networking: Rust gRPC

## Reflection

**1. Apa perbedaan utama antara unary, server streaming, dan bi-directional streaming RPC, dan dalam skenario apa masing-masing paling cocok digunakan?**

Unary RPC adalah komunikasi satu request dan satu response, cocok untuk operasi sederhana seperti autentikasi atau pembayaran di mana client hanya butuh satu jawaban. Server streaming RPC memungkinkan server mengirim banyak response atas satu request, cocok untuk kasus seperti mengambil riwayat transaksi atau data besar yang dikirim bertahap. Bi-directional streaming memungkinkan client dan server saling berkirim pesan secara bersamaan, cocok untuk aplikasi real-time seperti chat, game online, atau monitoring sistem.

**2. Apa pertimbangan keamanan yang perlu diperhatikan dalam implementasi gRPC di Rust, terutama terkait autentikasi, otorisasi, dan enkripsi data?**

Untuk autentikasi, gRPC sebaiknya menggunakan TLS/SSL untuk mengenkripsi koneksi dan token berbasis JWT atau OAuth untuk memverifikasi identitas. Untuk otorisasi, setiap RPC method perlu memeriksa hak akses user sebelum memproses request. Dalam implementasi Rust dengan Tonic, TLS dapat dikonfigurasi menggunakan `tonic::transport::ServerTlsConfig`. Selain itu, metadata gRPC bisa digunakan untuk membawa token autentikasi yang divalidasi di sisi server menggunakan interceptor.

**3. Apa tantangan atau isu yang mungkin muncul saat menangani bidirectional streaming di Rust gRPC, terutama dalam skenario seperti aplikasi chat?**

Tantangan utama adalah manajemen konkurensi karena Rust menggunakan ownership system yang ketat, sehingga berbagi state antar thread memerlukan penggunaan Arc dan Mutex. Selain itu, penanganan error pada stream harus dilakukan dengan hati-hati karena jika salah satu sisi stream terputus, sisi lainnya harus dapat mendeteksi dan menangani kondisi tersebut. Back-pressure juga menjadi isu ketika client mengirim pesan lebih cepat dari kemampuan server memprosesnya, sehingga perlu konfigurasi buffer channel yang tepat.

**4. Apa keuntungan dan kerugian menggunakan `tokio_stream::wrappers::ReceiverStream` untuk streaming response di Rust gRPC?**

Keuntungannya adalah ReceiverStream memudahkan konversi dari Tokio MPSC channel menjadi stream yang kompatibel dengan Tonic, sehingga kode menjadi lebih bersih dan mudah dipahami. Penggunaan channel juga memungkinkan pemisahan antara logika produksi data dan pengiriman ke client. Kerugiannya adalah adanya overhead dari channel MPSC itu sendiri, dan buffer size yang tidak tepat bisa menyebabkan memory berlebih atau blocking. Selain itu, error handling menjadi lebih kompleks karena melibatkan dua layer yaitu channel dan stream.

**5. Bagaimana kode Rust gRPC dapat distrukturisasi untuk memudahkan reuse dan modularitas, serta mendukung maintainability dan extensibility seiring waktu?**

Kode sebaiknya dipisahkan ke dalam modul-modul yang jelas, misalnya setiap service diletakkan di file terpisah. Trait implementation untuk setiap service dapat dipisahkan dari logika bisnis utama, sehingga logika bisnis dapat diuji secara independen. Penggunaan dependency injection juga membantu agar service tidak tightly coupled. Selain itu, shared types dan utilities dapat diletakkan di modul terpisah agar dapat digunakan ulang oleh berbagai service.

**6. Dalam implementasi `MyPaymentService`, langkah tambahan apa yang mungkin diperlukan untuk menangani logika pemrosesan pembayaran yang lebih kompleks?**

Implementasi saat ini hanya mengembalikan `success: true` tanpa validasi apapun. Untuk logika yang lebih kompleks, diperlukan validasi input seperti memastikan amount tidak negatif dan user_id valid. Integrasi dengan database untuk mencatat transaksi juga diperlukan. Selain itu, perlu ada mekanisme idempotency untuk mencegah double payment, penanganan timeout, serta integrasi dengan payment gateway eksternal. Error handling yang lebih detail juga perlu ditambahkan untuk memberi feedback yang jelas kepada client.

**7. Apa dampak adopsi gRPC sebagai protokol komunikasi terhadap arsitektur dan desain sistem terdistribusi, terutama dalam hal interoperabilitas dengan teknologi dan platform lain?**

gRPC mendorong arsitektur yang lebih terstruktur dengan contract-first design menggunakan Protocol Buffers. Hal ini meningkatkan konsistensi antar service namun memerlukan semua pihak untuk menggunakan tooling protobuf. Interoperabilitas dengan sistem non-gRPC memerlukan tambahan layer seperti gRPC-gateway untuk mengekspos REST API. Di sisi positif, gRPC mendukung banyak bahasa pemrograman sehingga microservices dapat ditulis dalam bahasa berbeda namun tetap dapat berkomunikasi dengan lancar.

**8. Apa keuntungan dan kerugian menggunakan HTTP/2 sebagai protokol dasar gRPC dibandingkan HTTP/1.1 dengan WebSocket untuk REST API?**

HTTP/2 pada gRPC memiliki keuntungan berupa multiplexing yang memungkinkan banyak request dalam satu koneksi TCP, header compression yang mengurangi overhead, serta dukungan native untuk streaming. Namun HTTP/2 lebih kompleks untuk di-debug dan tidak semua infrastruktur mendukungnya dengan baik. WebSocket pada REST lebih sederhana dan lebih luas didukung, namun tidak memiliki fitur multiplexing dan memerlukan implementasi protokol komunikasi sendiri di atas koneksi WebSocket.

**9. Bagaimana model request-response REST API berbeda dengan kemampuan bidirectional streaming gRPC dalam hal komunikasi real-time dan responsivitas?**

REST API bersifat stateless dan half-duplex, artinya setiap interaksi memerlukan koneksi baru dan hanya bisa request-response. Untuk real-time, REST memerlukan teknik seperti polling atau long-polling yang tidak efisien. Sebaliknya, gRPC bidirectional streaming memungkinkan komunikasi full-duplex di mana client dan server dapat mengirim pesan kapan saja tanpa menunggu, sehingga jauh lebih responsif dan efisien untuk skenario real-time seperti chat, notifikasi, atau live data feed.

**10. Apa implikasi pendekatan berbasis skema gRPC menggunakan Protocol Buffers dibandingkan dengan sifat JSON yang lebih fleksibel dan schema-less dalam payload REST API?**

Protocol Buffers menghasilkan payload yang lebih kecil dan parsing yang lebih cepat dibandingkan JSON, serta memberikan type safety yang kuat karena skema didefinisikan secara eksplisit. Namun kelemahannya adalah kurang fleksibel karena perubahan skema memerlukan regenerasi kode dan koordinasi antar tim. JSON lebih mudah dibaca manusia dan lebih fleksibel untuk perubahan, namun rentan terhadap inkonsistensi tipe data dan ukuran payload yang lebih besar. Untuk sistem berskala besar dengan banyak service, pendekatan schema-first seperti Protocol Buffers lebih menguntungkan dalam jangka panjang.
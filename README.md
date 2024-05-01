1. What are the key differences between unary, server streaming, and bi-directional streaming
   RPC (Remote Procedure Call) methods, and in what scenarios would each be most suitable?

Unary mechanism adalah ketika client mengirim satu request dan menerima satu respons, sedangkan server mechanism adalah ketika
server mampu memberikan beberapa respons ke client, dan bidirectional adalah ketika server dan client saling memberikan beberapa
respons secara kontinu satu sama lain. Unary digunakan ketika ingin menerima satu respons seperti autentikasi atau sebuah kalkulasi, 
server digunakan ketika server ingin mengirimkan banyak informasi yang beruntun seperti berita atau kumpulan artikel, dan bidirectional
digunakan dalam situasi yang terus berlangsung seperti event real time.

2. What are the potential security considerations involved in implementing a gRPC service in
   Rust, particularly regarding authentication, authorization, and data encryption?

gRPC memiliki banyak kelebihan yang membuatnya menjadi opsi yang sering dipakai oleh developer, namun gRPC memiliki beberapa kelemahan
jika tidak diimplementasikan dengan benar. Sebagai contoh, gRPC service umumnya tergabung dengan beberapa komponen, yang membuatnya rawan
terhadap injection attacks jika data tidak divalidasi dengan baik, terutama untuk input autentikasi dan enkripsi data. Untuk memitigasi hal ini,
gRPC menyediakan library yang mampu menvalidasi data yaitu protovalidate untuk memasang limitasi terhadap input data.

3. What are the potential challenges or issues that may arise when handling bidirectional
   streaming in Rust gRPC, especially in scenarios like chat applications?

Karena bidirectional streaming adalah mekanisme yang menerima dan mengirim dua koneksi sekaligus secara terus menerus, maka bidirectional mekanisme
umumnya memerlukan tenaga komputasi yang lebih besar dan bisa saja lebih lambat dari koneksi satu arah. 

4. What are the advantages and disadvantages of using the
   tokio_stream::wrappers::ReceiverStream for streaming responses in Rust gRPC services?

ReceiverStream adalah utilitas dari tokio_stream::wrappers yang mengkonversi tokio receiver menjadi stream yang bisa digunakan oleh tonic, lalu
stream tersebut diwrap oleh Response dan dikembalikan. Kelebihannya adalah ini dapat digunakan untuk mengirim value dari tokio ke stream asinkronus, 
sedangkan kekurangannya adalah karena ReceiverStream mengandalkan backpressure handling mechanism, dalam kondisi jika producer lebih cepat dari consumer,
maka hal ini mungkin bisa memenuhi memori atau bahkan membuat producer macet.

5. In what ways could the Rust gRPC code be structured to facilitate code reuse and modularity,
   promoting maintainability and extensibility over time?

Kita bisa melakukan beberapa hal seperti memisahkan service interface dan implementasinya, memisahkan komponen menjadi modul sendiri
jika digunakan dalam beberapa service sekaligus, menambahkan error handling untuk menghindari error yang tidak diketahui, dan lainnya. 
Selain itu, menggunakan dependency tertentu yang berfungsi untuk membuat kode lebih rapi dan efisien juga bisa dilakukan.

6. In the MyPaymentService implementation, what additional steps might be necessary to
   handle more complex payment processing logic?

Walaupun sudah menggunakan Result<Response<PaymentResponse>, Status> untuk memvalidasi status dan respons, sepertinya tidak ada skenario
tambahan untuk jika sebuah error terjadi. Mungkin sebuah respons tambahan atau kondisi Err bisa ditambahkan untuk melengkapi fungsi tersebut.

7. What impact does the adoption of gRPC as a communication protocol have on the overall
   architecture and design of distributed systems, particularly in terms of interoperability with
   other technologies and platforms?

Salah satu kelebihan gRPC adalah ia bisa digunakan dalam banyak bahasa yang berbeda, abstraksi dan simplifikasi dalam implementasi,
serta kemampuannya dalam menyediakan komunikasi antar server dan client yang lebih cepat, mudah, dan efisien membuatnya menjadi opsi yang sering dipakai oleh para developer. Walaupun server web pada umumnya
masih menggunakan REST API, gRPC juga menjadi pilihan yang semakin populer.

8. What are the advantages and disadvantages of using HTTP/2, the underlying protocol for
   gRPC, compared to HTTP/1.1 or HTTP/1.1 with WebSocket for REST APIs?

Keuntungan HTTP/2 adalah introduksi multiplexing, header compression, dan server push yang membuat komunikasi antar server dan client
lebih mudah. Kekurangannya adalah umumnya penggunaan HTTP/2 memakan lebih banyak bandwidth, walaupun memberikan hasil yang lebih cepat dari HTTP/1.

9. How does the request-response model of REST APIs contrast with the bidirectional streaming
   capabilities of gRPC in terms of real-time communication and responsiveness?

Model request-response REST API berarti client mengirim request berisi informasi dan server merespon terhadap request tersebut secara sinkronus, sedangkan 
dalam bidirectional, client dan server saling mengirim message secara asinkronus. Mengenai kecepatan dan efisiensi, bidirectional
streaming grpc lebih baik karena menggunakan http/2, namun mengenai kompatibilitas, rest api masih lebih sering digunakan karena web 
server pada umumnya mengakomodasi rest api, namun belum tentu grpc.

10. What are the implications of the schema-based approach of gRPC, using Protocol Buffers,
    compared to the more flexible, schema-less nature of JSON in REST API payloads?
Penggunaan protocol buffer tentunya membuat pemakaian grpc lebih mudah, efisien, dan cepat namun lebih terlimitasi dan terbatas
berdasarkan penggunaannya, sedangkan json dalam rest api menyediakan fleksibilitas dan didukung dalam berbagai sistem operasi sehingga
jauh lebih bebas.

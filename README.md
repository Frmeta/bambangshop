# BambangShop Publisher App
Tutorial and Example for Advanced Programming 2024 - Faculty of Computer Science, Universitas Indonesia

---

## About this Project
In this repository, we have provided you a REST (REpresentational State Transfer) API project using Rocket web framework.

This project consists of four modules:
1.  `controller`: this module contains handler functions used to receive request and send responses.
    In Model-View-Controller (MVC) pattern, this is the Controller part.
2.  `model`: this module contains structs that serve as data containers.
    In MVC pattern, this is the Model part.
3.  `service`: this module contains structs with business logic methods.
    In MVC pattern, this is also the Model part.
4.  `repository`: this module contains structs that serve as databases and methods to access the databases.
    You can use methods of the struct to get list of objects, or operating an object (create, read, update, delete).

This repository provides a basic functionality that makes BambangShop work: ability to create, read, and delete `Product`s.
This repository already contains a functioning `Product` model, repository, service, and controllers that you can try right away.

As this is an Observer Design Pattern tutorial repository, you need to implement another feature: `Notification`.
This feature will notify creation, promotion, and deletion of a product, to external subscribers that are interested of a certain product type.
The subscribers are another Rocket instances, so the notification will be sent using HTTP POST request to each subscriber's `receive notification` address.

## API Documentations

You can download the Postman Collection JSON here: https://ristek.link/AdvProgWeek7Postman

After you download the Postman Collection, you can try the endpoints inside "BambangShop Publisher" folder.
This Postman collection also contains endpoints that you need to implement later on (the `Notification` feature).

Postman is an installable client that you can use to test web endpoints using HTTP request.
You can also make automated functional testing scripts for REST API projects using this client.
You can install Postman via this website: https://www.postman.com/downloads/

## How to Run in Development Environment
1.  Set up environment variables first by creating `.env` file.
    Here is the example of `.env` file:
    ```bash
    APP_INSTANCE_ROOT_URL="http://localhost:8000"
    ```
    Here are the details of each environment variable:
    | variable              | type   | description                                                |
    |-----------------------|--------|------------------------------------------------------------|
    | APP_INSTANCE_ROOT_URL | string | URL address where this publisher instance can be accessed. |
2.  Use `cargo run` to run this app.
    (You might want to use `cargo check` if you only need to verify your work without running the app.)

## Mandatory Checklists (Publisher)
-   [x] Clone https://gitlab.com/ichlaffterlalu/bambangshop to a new repository.
-   **STAGE 1: Implement models and repositories**
    -   [x] Commit: `Create Subscriber model struct.`
    -   [x] Commit: `Create Notification model struct.`
    -   [x] Commit: `Create Subscriber database and Subscriber repository struct skeleton.`
    -   [x] Commit: `Implement add function in Subscriber repository.`
    -   [x] Commit: `Implement list_all function in Subscriber repository.`
    -   [x] Commit: `Implement delete function in Subscriber repository.`
    -   [x] Write answers of your learning module's "Reflection Publisher-1" questions in this README.
-   **STAGE 2: Implement services and controllers**
    -   [x] Commit: `Create Notification service struct skeleton.`
    -   [X] Commit: `Implement subscribe function in Notification service.`
    -   [X] Commit: `Implement subscribe function in Notification controller.`
    -   [X] Commit: `Implement unsubscribe function in Notification service.`
    -   [X] Commit: `Implement unsubscribe function in Notification controller.`
    -   [X] Write answers of your learning module's "Reflection Publisher-2" questions in this README.
-   **STAGE 3: Implement notification mechanism**
    -   [ ] Commit: `Implement update method in Subscriber model to send notification HTTP requests.`
    -   [ ] Commit: `Implement notify function in Notification service to notify each Subscriber.`
    -   [ ] Commit: `Implement publish function in Program service and Program controller.`
    -   [ ] Commit: `Edit Product service methods to call notify after create/delete.`
    -   [ ] Write answers of your learning module's "Reflection Publisher-3" questions in this README.

## Your Reflections
This is the place for you to write reflections:

### Mandatory (Publisher) Reflections

#### Reflection Publisher-1

In the Observer pattern diagram explained by the Head First Design Pattern book, Subscriber is defined as an interface. Explain based on your understanding of Observer design patterns, do we still need an interface (or trait in Rust) in this BambangShop case, or a single Model struct is enough?
- Penggunaan interface (atau trait di Rust) digunakan apabila objek Subscriber kita terdiri dari beberapa tipe yang memiliki behaviour yang berbeda sendiri. Namun karena pada kasus BambangShop semua subscribers dianggap sama, maka interface tersebut tidak diperlukan

id in Product and url in Subscriber is intended to be unique. Explain based on your understanding, is using Vec (list) sufficient or using DashMap (map/dictionary) like we currently use is necessary for this case?
- Apabila id pada Product dan url pada subscriber diapastikan selalu unik, maka DashMap (map/dictionary) lebih cocok digunakan daripada Vec (list). Hal ini dikarenakan DashMap yang menggunakan variabel yang unik tersebut sebagai key dapat mempercepat lookup, insertion, dan deletion dengan kompleksitas waktu rata-ratanya adalah O(n). Sedangkan penggunaan Vec (list) lebih lambat karena kompleksitas rata-rata waktunya O(n).

When programming using Rust, we are enforced by rigorous compiler constraints to make a thread-safe program. In the case of List of Subscribers (SUBSCRIBERS) static variable, we used the DashMap external library for thread-safe HashMap. Explain based on your understanding of design patterns, do we still need DashMap or we can implement Singleton pattern instead?
- Dalam Rust, DashMap dan HashMap sama-sama digunakan untuk menyimpan data berupa pasangan key dan value. Namun perbedaanya, DashMap mendukung thread safety dimana dapat diakses dan dimodifikasi oleh banyak threads secara bersamaan tanpa mengakibatkan data races.
- Pada kasus ini kita membutuhkan DashMap dan Singleton pattern karena mereka berdua saling mendukung satu sama lain. Singleton pattern memastikan bahwa suatu class hanya memiliki satu instance, hal ini sudah diterapkan dengan `lazy_static`. Meskipun sudah menerapkan singleton, data SUBSCRIBERS akan diakses dan dimodifikasi oleh banyak thread sekaligus, sehingga maish diperlukan struktur data yang thread-safe, seperti DashMap.


#### Reflection Publisher-2

In the Model-View Controller (MVC) compound pattern, there is no “Service” and “Repository”. Model in MVC covers both data storage and business logic. Explain based on your understanding of design principles, why we need to separate “Service” and “Repository” from a Model?
- Menurut prinsip desain 'Seperatin of Concerns', tiap bagian sistem software sebaiknya dipisahkan berdasarkan daerah fokusnya sehingga apabila ada perubahan hanya mengubah bagian tersebut saja. "Repository" fokus ke bagian penyimpanan dan pengambilan data sedangkan "Service" menangani business logic.

What happens if we only use the Model? Explain your imagination on how the interactions between each model (Program, Subscriber, Notification) affect the code complexity for each model?
- Apabila kita hanya menggunakan model untuk menangani penyimpanan data dan business logic, dapat terjadi high coupling (setiap model harus mengetahui informasi tentang model lain, perubahan pada satu model dapat menyebabkan perubahan pada model lain pula). Akibatnya kode menjadi sulit dipelihara.

Have you explored more about Postman? Tell us how this tool helps you to test your current work. Maybe you want to also list which features in Postman you are interested in or feel like it’s helpful to help your Group Project or any of your future software engineering projects.
- Postman adalah alat yang digunakan untuk melakukan uji coba API. Fitur yang saya anggap paling menarik adalah HTTP Request dimana saya dapat mengatur sendiri jenis request (GET, POST, DELETE, dll) serta membagikan collection of requests hanya menggunakan copy paste text. 


#### Reflection Publisher-3
Observer Pattern has two variations: Push model (publisher pushes data to subscribers) and Pull model (subscribers pull data from publisher). In this tutorial case, which variation of Observer Pattern that we use?
- Pada kasus ini kita menggunakan variasi Push Model karena ketika create/delete/publish product baru, semua subscriber yang berlangganan product_type tersebut akan menerima notifikasi.

What are the advantages and disadvantages of using the other variation of Observer Pattern for this tutorial case? (example: if you answer Q1 with Push, then imagine if we used Pull)
- Pull Model memiliki keuntungan berupa informasi subscriber tidak ter-expose oleh publisher. Kekurangan dari Pull Model adalah pull model harus menggunakan proses update yang memakan banyak resource untuk memeriksa terus menerus terhadap kondisi publisher.Akibatnya akan meningkatkan network traffic dan latency.

Explain what will happen to the program if we decide to not use multi-threading in the notification process.
- Tanpa multi-threading, notifikasi akan dikirimkan kepada subscriber secara satu persatu secara sinkronus. Hal ini membuat proses notifikasi menjaid sangat lambat, akiatnya user experience menjadi jelek.

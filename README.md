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

You can download the Postman Collection JSON here: [Link removed for security]

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
    -   [x] Commit: `Implement subscribe function in Notification service.`
    -   [x] Commit: `Implement subscribe function in Notification controller.`
    -   [x] Commit: `Implement unsubscribe function in Notification service.`
    -   [x] Commit: `Implement unsubscribe function in Notification controller.`
    -   [x] Write answers of your learning module's "Reflection Publisher-2" questions in this README.
-   **STAGE 3: Implement notification mechanism**
    -   [x] Commit: `Implement update method in Subscriber model to send notification HTTP requests.`
    -   [x] Commit: `Implement notify function in Notification service to notify each Subscriber.`
    -   [x] Commit: `Implement publish function in Program service and Program controller.`
    -   [x] Commit: `Edit Product service methods to call notify after create/delete.`
    -   [x] Write answers of your learning module's "Reflection Publisher-3" questions in this README.

## Your Reflections
This is the place for you to write reflections:

### Mandatory (Publisher) Reflections

#### Reflection Publisher-1
###### 1. In the Observer pattern diagram explained by the Head First Design Pattern book, Subscriber is defined as an interface. Explain based on your understanding of Observer design patterns, do we still need an interface (or trait in Rust) in this BambangShop case, or a single Model struct is enough?
- A single model struct is enough because the publisher does exactly one uniform thing for every subscriber: it sends an HTTP POST request to a URL. Because the data required for this action is always exactly the same (a url and a name), a single struct containing those fields is sufficient. We would only need a trait if our Rocket app had to handle completely different types of internal subscribers doing different local tasks.

###### 2. id in Program and url in Subscriber is intended to be unique. Explain based on your understanding, is using Vec (list) sufficient or using DashMap (map/dictionary) like we currently use is necessary for this case
- Because it is necessary for performance and integrity, for example, the perfomance of delete will be faster using Map because the app can instantly jump to that specific URL and removes it (O(1) time complexity). On the contrary, in a `Vec` it has to check every single item one by one until it finds a match (O(n) complexity). 

###### 3. When programming using Rust, we are enforced by rigorous compiler constraints to make a thread-safe program. In the case of the List of Subscribers (SUBSCRIBERS) static variable, we used the DashMap external library for thread safe HashMap. Explain based on your understanding of design patterns, do we still need DashMap or, we can implement Singleton pattern instead?
Yes, we need `DashMap` because the Singleton pattern and `DashMap` solve two completely different problem. 
- Singleton ensures that there is exactly one database instance shared across the entire application. It prevents the app from accidentally creating a blank subscriber list every time a new request comes in.


- Meanwhile `DashMap` provides internal locks so that multiple threads can safely interact with that single, shared instance without crashing. Because rocket is a multithreaded web framework, this means multiple HTTP request are processed at the same time. If the Singleton was a regular `HashMap` and two threads tried to write to it simultaneously, race condition may occur which Rust strictly forbid.

#### Reflection Publisher-2

###### 1. In the Model-View Controller (MVC) compound pattern, there is no “Service” and “Repository”. Model in MVC covers both data storage and business logic. Explain based on your understanding of design principles, why we need to separate “Service” and “Repository” from a Model?
Because we want to apply one of the most important rules in software engineering: Single Responsibility Principle. By separating them, we create a layered architecture which consist of:

- Model: acts as a simple data container or blueprint. It doesn't know how it gets saved or what business rules apply to it.
- Repository: Strictly handle database logic. It's only job is to CRUD records. It doesn't care about business rules.
- Service: Strictly handle the business logic. It tells the repository to fetch data, apply some rule, and hand it back to controller.

This makes the code easier to maintain and to test

###### 2. What happens if we only use the Model? Explain your imagination on how the interactions between each model (Program, Subscriber, Notification) affect the code complexity for each model?
There will be high coupling because for example when we create a `Product` that handles all the logic. Then it has to save itself to the DB, Look up to `Subscriber` DB to find who wants to know about it, createes a Notification Object, and finally executes an HTTP network request to send that notification.

The `Product` model suddenly has to import network libraries, database libraries, and know the  details of how Subscriber and Notification work. If we change how a notification is formatted, we have to open and modify the Product file. Testing becomes more complex because we can't just test the product creation without triggering real network requests.

###### 3. Have you explored more about Postman? Tell us how this tool helps you to test your current work. You might want to also list which features in Postman you are interested in or feel like it is helpful to help your Group Project or any of your future software engineering projects.

1. Testing current work: it allows us to simulate the exact HTTP request the frontend will eventually take, verifying that the endpoints will return the correct JSON without needing a complete frontend.
2. API Documentation: it creates an interactive web based documentation which helps people reading the document understand how to interact with the system

I can definitely see myself in the future using the API Collection Documentation

#### Reflection Publisher-3

###### 1. Observer Pattern has two variations: Push model (publisher pushes data to subscribers) and Pull model (subscribers pull data from publisher). In this tutorial case, which variation of Observer Pattern that we use?
We use `Push model`, because when a product is created or deleted the Publisher immediately constructs a `Notification` payload and executes `subscriber_clone.update(payload_clone)`. The publisher is actively packaging the data and "pushing" it directly to the subscribers' URLs over HTTP. The subscriber doesn't have to ask for data, it is handed to them as soon as the event happens.  

###### 2. What are the advantages and disadvantages of using the other variation of Observer Pattern for this tutorial case? (example: if you answer Q1 with Push, then imagine if we used Pull
Disadvantages of Pull in this case:

- Wasted resources / Network Spam: the receiver would have to send request every few seconds to check for updates. 99% of the time the answer would be "No". Which wastes network bandwidth.
- Latency: if the receiver checks every 5 minutes, and a product is created/deleted one second after a check, then the receiver won't find out about it for another 4 minutes 59 seconds.

Advantages of Pull in this case:

- Less published responsibility: the publisher doesn't need to keep a `SUBSCRIBER` database or manage HTTP requests to lots of external URLs.
- Subscriber pacing: the receiver can only pull data only when it has the resources e.g. CPU/memory to handle it.

###### 3. Explain what will happen to the program if we decide to not use multi-threading in the notification process.

This is what would happen to the program without multi-threading:

- When a user creates a product via API, the notify loop starts.

- The loop sends an HTTP POST request to Subscriber 1. The entire Publisher server stops and waits for Subscriber 1 to receive it and send back an "OK" response.

- If Subscriber 1's server is offline or lagging (e.g., takes 10 seconds to timeout), the Publisher is stuck waiting.

- Only after finishing with Subscriber 1 will it move to Subscriber 2, and so on.

- The original user who clicked "Create Product" will be stuck staring at a loading screen for a massive amount of time because the HTTP response won't return until every single subscriber has been notified sequentially.
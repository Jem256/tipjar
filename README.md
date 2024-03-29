# Tipjar

Tipjar is a platform designed to empower content creators by enabling audiences to support them with small tips through Lightning payments. This repository contains the source code for the Tipjar project.

## Features

- **Lightning Payment Integration**: Seamlessly generate and process Lightning payment links.
- **Global Support**: Enable supporters from around the world to tip their favorite creators.
- **Content Creator Dashboard**: A dashboard for creators to manage and track their earnings.

## Technologies Used

- **Backend**: Rust, Diesel
- **Frontend**: Next.js

## Backend Setup

1. **Install Rust**: If you haven't installed Rust, you can do so by following the instructions on [rustup.rs](https://rustup.rs/).
2. **Install Diesel CLI**: 
    ```bash
    cargo install diesel_cli --no-default-features --features postgres
    ```
3. **Database Setup**: 
    - Create a `.env` file and configure your database connection string.
    - Run migrations using Diesel:
      ```bash
      diesel migration run
      ```
4. **Run Backend**: 
    ```bash
    cargo run
    ```

## Frontend Setup

1. **Install Node.js**: If you haven't installed Node.js, download and install it from [nodejs.org](https://nodejs.org/).
2. **Install Dependencies**: 
    ```bash
    cd frontend
    npm install
    ```
3. **Run Frontend**: 
    ```bash
    npm run dev
    ```

## Contributing

We welcome contributions to Tipjar! To contribute, please follow these steps:

1. Fork the repository.
2. Create your feature branch (`git checkout -b feature/new-feature`).
3. Commit your changes (`git commit -am 'Add new feature'`).
4. Push to the branch (`git push origin feature/new-feature`).
5. Open a pull request.

## Team Members

- **Joseph Kipkogei**
  - GitHub: [github.com/joseguru](https://github.com/joseguru)
  - Role: Backend Developer
  
- **Nagasha Jemimah**
  - GitHub: [github.com/Jem256](https://github.com/Jem256)
  - Role: Frontend Developer

- **Collins Okafor**
  - GitHub: [github.com/collins-okafor](https://github.com/collins-okafor)
  - Role: Backend Developer & Infrastructure

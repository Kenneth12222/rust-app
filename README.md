# Rust-JavaScript Full-Stack Project

This project demonstrates the integration of a Rust-based backend with a JavaScript-based frontend. The backend is built using Rust's **Actix Web** framework to handle API requests, and the frontend uses vanilla JavaScript for interactivity.

## Features
- **Create Posts**: Add new posts with a title and content.
- **Read Posts**: Fetch and display all posts.
- **Update Posts**: Edit existing posts.
- **Delete Posts**: Remove posts from the list.

## Tech Stack
### Backend
- **Language**: Rust
- **Framework**: Actix Web
- **Data Management**: In-memory storage using `Mutex<Vec<Post>>`

### Frontend
- **Language**: JavaScript
- **HTML/CSS**: Basic styling and structure

## Project Structure
### Backend (Rust)
```
rust_backend/
├── src/
│   ├── main.rs  # Main application logic
├── Cargo.toml    # Dependency and project configuration
```

### Frontend (JavaScript)
```
frontend/
├── index.html   # Frontend structure
├── script.js    # Main JavaScript logic for API interaction
├── style.css    # Optional styling (if included)
```

## Getting Started

### Prerequisites
- **Rust**: Install Rust from [https://www.rust-lang.org/](https://www.rust-lang.org/)
- **Node.js** (Optional): For managing frontend dependencies if needed

### Backend Setup
1. Clone the repository and navigate to the backend directory:
   ```bash
   git clone <repository-url>
   cd rust_backend
   ```

2. Install dependencies and build the project:
   ```bash
   cargo build
   ```

3. Run the server:
   ```bash
   cargo run
   ```
   The backend will be running on [http://127.0.0.1:5000](http://127.0.0.1:5000).

### Frontend Setup
1. Open the `index.html` file located in the `frontend/` directory in your preferred browser.

2. Ensure the backend is running to fetch and interact with posts.

## API Endpoints
### GET `/posts`
Fetch all posts.
- **Response**: JSON array of posts

### POST `/posts`
Create a new post.
- **Request Body**:
  ```json
  {
    "title": "Post Title",
    "content": "Post Content"
  }
  ```
- **Response**: Updated list of posts

### PUT `/posts/{id}`
Update an existing post by ID.
- **Request Body**:
  ```json
  {
    "title": "Updated Title",
    "content": "Updated Content"
  }
  ```
- **Response**: Updated post object

### DELETE `/posts/{id}`
Delete a post by ID.
- **Response**: Confirmation message

## How to Extend the Project
### Backend
- **Add Database**: Integrate SQLite, PostgreSQL, or MySQL for persistent storage.
- **Authentication**: Implement user authentication using JWT or session-based auth.

### Frontend
- **Styling**: Use CSS frameworks like Bootstrap or TailwindCSS for improved UI.
- **Frontend Framework**: Convert the vanilla JavaScript code to React, Vue, or Angular.

## Contributing
1. Fork the repository.
2. Create a feature branch: `git checkout -b feature-name`
3. Commit your changes: `git commit -m 'Add feature-name'`
4. Push to the branch: `git push origin feature-name`
5. Open a pull request.

## License
This project is licensed under the MIT License. See the `LICENSE` file for details.

## Acknowledgments
- Rust and Actix Web documentation
- Mozilla Developer Network (MDN) for JavaScript resources

Enjoy building with Rust and JavaScript! 🚀


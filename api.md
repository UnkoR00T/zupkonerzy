# API Endpoints

This document outlines the available API endpoints, their functionalities, request/response formats, and authentication requirements.

## Authentication

### 1. Register Client

-   **HTTP Method:** `POST`
-   **Endpoint:** `/api/clients/register`
-   **Description:** Registers a new client (user) with the system.
-   **Request Body:** `application/json`
    ```json
    {
      "username": "string",
      "email": "string",
      "password": "string"
    }
    ```
-   **Response Body:** `application/json`
    -   **Success (201 Created):**
        ```json
        {
          "token": "string" // JWT token for authentication
        }
        ```
    -   **Error (500 Internal Server Error):**
        ```json
        {
          "error": "string" // e.g., "Database error", "Failed to create token"
        }
        ```

### 2. Login Client

-   **HTTP Method:** `POST`
-   **Endpoint:** `/api/clients/login`
-   **Description:** Authenticates a client and returns a JWT token for subsequent requests.
-   **Request Body:** `application/json`
    ```json
    {
      "email": "string",
      "password": "string"
    }
    ```
-   **Response Body:** `application/json`
    -   **Success (200 OK):**
        ```json
        {
          "token": "string", // JWT token for authentication
          "account_details": {
            "username": "string",
            "email": "string"
          }
        }
        ```
    -   **Error (401 Unauthorized):**
        ```json
        {
          "error": "string" // e.g., "Account not found", "Password is not correct."
        }
        ```
    -   **Error (500 Internal Server Error):**
        ```json
        {
          "error": "string" // e.g., "Database error", "Failed to parse password", "Failed to create token"
        }
        ```

## Room Management

**Authentication:** All room management endpoints require a valid JWT token in the `Authorization: Bearer <token>` header. The client making the request must be the owner of the room.

### 1. Create Room

-   **HTTP Method:** `POST`
-   **Endpoint:** `/api/rooms`
-   **Description:** Creates a new room for the authenticated client.
-   **Request Body:** `application/json`
    ```json
    {
      "name": "string"
    }
    ```
-   **Response Body:** `application/json`
    -   **Success (201 Created):**
        ```json
        {
          "id": "uuid" // ID of the newly created room
        }
        ```
    -   **Error (400 Bad Request):**
        ```json
        {
          "error": "string" // e.g., "Invalid client ID"
        }
        ```
    -   **Error (500 Internal Server Error):**
        ```json
        {
          "error": "string" // e.g., "Database error"
        }
        ```

### 2. Get All Rooms

-   **HTTP Method:** `GET`
-   **Endpoint:** `/api/rooms`
-   **Description:** Retrieves a list of all rooms owned by the authenticated client.
-   **Request Body:** None
-   **Response Body:** `application/json`
    -   **Success (200 OK):**
        ```json
        [
          {
            "id": "uuid",
            "name": "string",
            "owner": "uuid"
          }
        ]
        ```
    -   **Error (400 Bad Request):**
        ```json
        {
          "error": "string" // e.g., "Invalid client ID"
        }
        ```
    -   **Error (500 Internal Server Error):**
        ```json
        {
          "error": "string" // e.g., "Database error"
        }
        ```

### 3. Update Room

-   **HTTP Method:** `PUT`
-   **Endpoint:** `/api/rooms/:room_id`
-   **Description:** Updates the name of a specific room owned by the authenticated client.
-   **Path Parameters:**
    -   `room_id`: The UUID of the room to update.
-   **Request Body:** `application/json`
    ```json
    {
      "name": "string"
    }
    ```
-   **Response Body:** `application/json`
    -   **Success (200 OK):**
        ```json
        {
          "message": "Room updated"
        }
        ```
    -   **Error (400 Bad Request):**
        ```json
        {
          "error": "string" // e.g., "Invalid room ID", "Invalid client ID"
        }
        ```
    -   **Error (404 Not Found):**
        ```json
        {
          "error": "string" // e.g., "Room not found or access denied"
        }
        ```
    -   **Error (500 Internal Server Error):**
        ```json
        {
          "error": "string" // e.g., "Database error"
        }
        ```

### 4. Delete Room

-   **HTTP Method:** `DELETE`
-   **Endpoint:** `/api/rooms/:room_id`
-   **Description:** Deletes a specific room owned by the authenticated client.
-   **Path Parameters:**
    -   `room_id`: The UUID of the room to delete.
-   **Request Body:** None
-   **Response Body:** `application/json`
    -   **Success (200 OK):**
        ```json
        {
          "message": "Room deleted"
        }
        ```
    -   **Error (400 Bad Request):**
        ```json
        {
          "error": "string" // e.g., "Invalid room ID", "Invalid client ID"
        }
        ```
    -   **Error (404 Not Found):**
        ```json
        {
          "error": "string" // e.g., "Room not found or access denied"
        }
        ```
    -   **Error (500 Internal Server Error):**
        ```json
        {
          "error": "string" // e.g., "Database error"
        }
        ```

## Question Management

**Authentication:** All question management endpoints require a valid JWT token in the `Authorization: Bearer <token>` header. The client making the request must be the owner of the room to which the question belongs.

### 1. Create Question

-   **HTTP Method:** `POST`
-   **Endpoint:** `/api/rooms/:room_id/questions`
-   **Description:** Creates a new question within a specified room.
-   **Path Parameters:**
    -   `room_id`: The UUID of the room where the question will be created.
-   **Request Body:** `application/json`
    ```json
    {
      "question": "string",
      "img_url": "string | null",
      "video_url": "string | null",
      "answers": ["string"],
      "correct": "integer" // 0-based index of the correct answer
    }
    ```
-   **Response Body:** `application/json`
    -   **Success (201 Created):**
        ```json
        {
          "id": "uuid" // ID of the newly created question
        }
        ```
    -   **Error (400 Bad Request):**
        ```json
        {
          "error": "string" // e.g., "Invalid room ID", "Invalid client ID"
        }
        ```
    -   **Error (404 Not Found):**
        ```json
        {
          "error": "string" // e.g., "Room not found or access denied"
        }
        ```
    -   **Error (500 Internal Server Error):**
        ```json
        {
          "error": "string" // e.g., "Database error"
        }
        ```

### 2. Get All Questions

-   **HTTP Method:** `GET`
-   **Endpoint:** `/api/rooms/:room_id/questions`
-   **Description:** Retrieves all questions belonging to a specific room owned by the authenticated client.
-   **Path Parameters:**
    -   `room_id`: The UUID of the room to retrieve questions from.
-   **Request Body:** None
-   **Response Body:** `application/json`
    -   **Success (200 OK):**
        ```json
        [
          {
            "id": "uuid",
            "question": "string",
            "img_url": "string | null",
            "video_url": "string | null",
            "answers": ["string"],
            "correct": "integer"
          }
        ]
        ```
    -   **Error (400 Bad Request):**
        ```json
        {
          "error": "string" // e.g., "Invalid room ID", "Invalid client ID"
        }
        ```
    -   **Error (404 Not Found):**
        ```json
        {
          "error": "string" // e.g., "Room not found or access denied"
        }
        ```
    -   **Error (500 Internal Server Error):**
        ```json
        {
          "error": "string" // e.g., "Database error"
        }
        ```

### 3. Update Question

-   **HTTP Method:** `PUT`
-   **Endpoint:** `/api/rooms/:room_id/questions/:question_id`
-   **Description:** Updates a specific question within a room.
-   **Path Parameters:**
    -   `room_id`: The UUID of the room containing the question.
    -   `question_id`: The UUID of the question to update.
-   **Request Body:** `application/json`
    ```json
    {
      "question": "string",
      "img_url": "string | null",
      "video_url": "string | null",
      "answers": ["string"],
      "correct": "integer" // 0-based index of the correct answer
    }
    ```
-   **Response Body:** `application/json`
    -   **Success (200 OK):**
        ```json
        {
          "message": "Question updated"
        }
        ```
    -   **Error (400 Bad Request):**
        ```json
        {
          "error": "string" // e.g., "Invalid room ID", "Invalid question ID", "Invalid client ID"
        }
        ```
    -   **Error (404 Not Found):**
        ```json
        {
          "error": "string" // e.g., "Room not found or access denied", "Question not found"
        }
        ```
    -   **Error (500 Internal Server Error):**
        ```json
        {
          "error": "string" // e.g., "Database error"
        }
        ```

### 4. Delete Question

-   **HTTP Method:** `DELETE`
-   **Endpoint:** `/api/rooms/:room_id/questions/:question_id`
-   **Description:** Deletes a specific question from a room.
-   **Path Parameters:**
    -   `room_id`: The UUID of the room containing the question.
    -   `question_id`: The UUID of the question to delete.
-   **Request Body:** None
-   **Response Body:** `application/json`
    -   **Success (200 OK):**
        ```json
        {
          "message": "Question deleted"
        }
        ```
    -   **Error (400 Bad Request):**
        ```json
        {
          "error": "string" // e.g., "Invalid room ID", "Invalid question ID", "Invalid client ID"
        }
        ```
    -   **Error (404 Not Found):**
        ```json
        {
          "error": "string" // e.g., "Room not found or access denied", "Question not found"
        }
        ```
    -   **Error (500 Internal Server Error):**
        ```json
        {
          "error": "string" // e.g., "Database error"
        }
        ```

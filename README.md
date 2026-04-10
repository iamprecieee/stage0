# HNG Stage 0 - Name Categorization API

This is a Rust-based API that categorizes names using the Genderize API. It is built using Axum and Tokio for high performance and reliability.

## Requirements

- Rust (Latest stable version)
- Cargo

## Setup and Installation

1. Clone the repository:

   ```bash
   git clone https://github.com/iamprecieee/stage0
   cd stage0
   ```

2. Run the server:
   ```bash
   cargo run
   ```
   The server will start at `http://0.0.0.0:3000`.

## API Documentation

### Classify Name

**Endpoint**: `GET /api/classify`

**Query Parameters**:

- `name` (required): The name to categorize. Must be a valid string (not purely numeric).

**Example Request**:

```bash
curl "http://0.0.0.0:3000/api/classify?name=john"
```

**Example Successful Response (200 OK)**:

```json
{
  "status": "success",
  "data": {
    "name": "john",
    "gender": "male",
    "probability": 0.99,
    "sample_size": 1785,
    "is_confident": true,
    "processed_at": "2026-04-10T20:45:12Z"
  }
}
```

**Example Error Response (422 Unprocessable Entity)**:

```json
{
  "status": "error",
  "message": "name is not a string"
}
```

## Deployment

The application is designed to be easily deployable on any platform supporting Rust (e.g., Render, Railway, AWS, etc.). Ensure the `0.0.0.0` address is used for external accessibility.

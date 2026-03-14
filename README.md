# OTP Manager

One-Time Password (OTP) management system built with Go backend and Rust microservices using Docker Compose for easy deployment and security.

## Features

- **TOTP/HOTP Generation**: Generate time-based and counter-based OTP tokens
- **OTP Verification**: Validate OTP tokens with configurable time windows
- **Encryption**: Rust-based encryption service for secure key storage
- **REST API**: Go-based REST API for easy integration
- **Docker Compose**: Complete containerized deployment
- **Production-Ready**: Built with security best practices

## Architecture

```
┌─────────────────┐
│   Client App    │
└────────┬────────┘
         │ HTTP
         ↓
┌─────────────────────────────────────────┐
│   Go Backend (API Server)               │
│   - OTP Generation                      │
│   - Token Verification                  │
│   - User Management                     │
│   - Database Integration                │
└────────┬────────────────────────────────┘
         │ gRPC
         ↓
┌─────────────────────────────────────────┐
│   Rust Microservice (Encryption)        │
│   - Key Encryption/Decryption           │
│   - Secure Storage                      │
│   - HMAC Operations                     │
└─────────────────────────────────────────┘
```

## Project Structure

```
otp-manager/
├── backend/               # Go backend service
│   ├── cmd/
│   │   └── server/
│   │       └── main.go
│   ├── internal/
│   │   ├── handler/
│   │   ├── service/
│   │   └── models/
│   ├── go.mod
│   ├── go.sum
│   └── Dockerfile
├── crypto-service/       # Rust encryption service
│   ├── src/
│   │   └── main.rs
│   ├── Cargo.toml
│   └── Dockerfile
├── docker-compose.yml    # Container orchestration
├── .gitignore
└── README.md
```

## Quick Start

### Prerequisites

- Docker
- Docker Compose

### Running with Docker Compose

```bash
# Clone the repository
git clone https://github.com/eentost/otp-manager.git
cd otp-manager

# Start all services
docker-compose up -d

# Check service status
docker-compose ps
```

## API Endpoints

### Generate OTP
```
POST /api/v1/otp/generate
Content-Type: application/json

{
  "user_id": "user123",
  "secret_key": "base32-encoded-secret"
}

Response:
{
  "otp_token": "123456",
  "expires_at": 1234567890
}
```

### Verify OTP
```
POST /api/v1/otp/verify
Content-Type: application/json

{
  "user_id": "user123",
  "otp_token": "123456"
}

Response:
{
  "valid": true,
  "message": "OTP verified successfully"
}
```

## Configuration

Environment variables can be set in `.env` file or in `docker-compose.yml`:

```env
# Go Backend
GO_PORT=8080
DATABASE_URL=postgres://user:password@db:5432/otp_db
RUST_SERVICE_URL=http://crypto-service:9090

# Rust Crypto Service
RUST_PORT=9090
LOG_LEVEL=info
```

## Development

### Building Locally

#### Go Backend
```bash
cd backend
go build -o server ./cmd/server
./server
```

#### Rust Service
```bash
cd crypto-service
cargo build --release
cargo run --release
```

## Security Considerations

- **Secret Key Storage**: All secret keys are encrypted using the Rust service
- **Time Window**: OTP verification uses configurable time windows (default 30 seconds)
- **Rate Limiting**: Built-in rate limiting for OTP verification attempts
- **HTTPS**: Always use HTTPS in production
- **Environment Variables**: Sensitive data should be managed via environment variables

## Testing

```bash
# Run unit tests (Go)
cd backend
go test ./...

# Run unit tests (Rust)
cd crypto-service
cargo test
```

## Technologies

- **Go**: REST API, business logic
- **Rust**: Encryption, secure operations
- **Docker**: Containerization
- **Docker Compose**: Service orchestration
- **PostgreSQL**: Data persistence
- **gRPC**: Inter-service communication

## License

MIT License - see LICENSE file for details

## Author

eentost - Security Researcher

## Contributing

Contributions are welcome! Please feel free to submit pull requests.

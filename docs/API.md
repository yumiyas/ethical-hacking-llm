# API Documentation

## Base URL
`http://localhost:3000`

## Endpoints

### Health Check
`GET /health`

Returns the health status of the service.

**Response:**
```json
{
  "status": "healthy",
  "version": "0.1.0",
  "timestamp": "2024-01-01T00:00:00Z"
}

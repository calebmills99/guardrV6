# Port Configuration

## Current Port Allocation

- **Port 3000**: ntopng (network monitoring dashboard)
  - Access: http://localhost:3000
  - Credentials: admin / admin

- **Port 3001**: Another Next.js instance (can be stopped if not needed)

- **Port 3002**: Guardr Website (Next.js) ✅
  - Access: http://localhost:3002
  - This is your active Guardr frontend

- **Port 5000**: Guardr API Backend (Rust) ✅
  - Access: http://localhost:5000
  - Health check: http://localhost:5000/health

## Restart Backend to Pick Up CORS Changes

The backend needs to be restarted to allow requests from port 3002:

```bash
# Find and kill the current backend process
ps aux | grep guardr-api | grep -v grep
kill <PID>

# Or use pkill
pkill guardr-api

# Restart the backend
cd ~/guardrV6
cargo run --bin guardr-api
```

## Frontend Configuration

The frontend is configured to connect to the backend:

**File:** `website/.env.local`
```bash
NEXT_PUBLIC_API_URL=http://localhost:5000
```

## Clean Up Extra Next.js Instances (Optional)

If you don't need the instance on port 3001:
```bash
# Find Next.js processes
ps aux | grep next-server

# Kill the one on port 3001
lsof -ti:3001 | xargs kill
```

## CORS Settings

The backend now allows CORS from all three Next.js ports:
```toml
cors_allowed_origins = ["http://localhost:3000", "http://localhost:3001", "http://localhost:3002"]
```

## Testing the Connection

After restarting the backend:

1. Visit: http://localhost:3002
2. Try the demo search
3. It should now successfully connect to the API on port 5000

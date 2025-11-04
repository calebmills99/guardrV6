# Async OSINT API Usage Guide

The Guardr platform now uses **async job processing** for OSINT analysis to handle the 2-5 minute execution time without timeouts.

## Architecture

```
Frontend → POST /api/check-async → Returns job_id immediately (202 Accepted)
         ↓
         Poll GET /api/job/{job_id} every 5-10 seconds
         ↓
         Receive completed results with full OSINT analysis
```

## API Endpoints

### 1. Submit Async Job

**Endpoint:** `POST /kallisto-osinter/api/check-async`

**Request:**
```json
{
  "name": "Jane Smith",
  "location": "Chicago",
  "username": "janesmith99",  // optional
  "email": "jane@example.com" // optional
}
```

**Response (202 Accepted):**
```json
{
  "job_id": "9e0d2019-80e4-40a2-9627-931c58cf1e9b",
  "status": "pending",
  "message": "OSINT analysis started. Poll /api/job/{job_id} for results.",
  "estimated_time": "2-5 minutes"
}
```

### 2. Poll Job Status

**Endpoint:** `GET /kallisto-osinter/api/job/{job_id}`

**Response (Processing):**
```json
{
  "job_id": "9e0d2019-80e4-40a2-9627-931c58cf1e9b",
  "status": "processing",
  "created_at": "2025-11-03T23:55:13.166171",
  "updated_at": "2025-11-03T23:55:13.175877"
}
```

**Response (Completed):**
```json
{
  "job_id": "9e0d2019-80e4-40a2-9627-931c58cf1e9b",
  "status": "completed",
  "created_at": "2025-11-03T23:55:13.166171",
  "updated_at": "2025-11-03T23:58:42.523198",
  "completed_at": "2025-11-03T23:58:42.523198",
  "result": {
    "name": "Jane Smith",
    "location": "Chicago",
    "risk_level": "MEDIUM",
    "risk_score": 55,
    "person_verification": "AI analysis report...",
    "username_verification": [...],
    "recommendations": [...],
    "safety_tips": [...]
  }
}
```

**Response (Failed):**
```json
{
  "job_id": "9e0d2019-80e4-40a2-9627-931c58cf1e9b",
  "status": "failed",
  "created_at": "2025-11-03T23:55:13.166171",
  "updated_at": "2025-11-03T23:56:30.123456",
  "error": "Error message describing what went wrong"
}
```

## Frontend Implementation Example

```typescript
async function performOSINTCheck(name: string, location: string) {
  // 1. Submit job
  const submitResponse = await fetch('https://guardr.app/kallisto-osinter/api/check-async', {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify({ name, location })
  });

  const { job_id } = await submitResponse.json();

  // 2. Poll for results
  let status = 'pending';
  let result = null;

  while (status === 'pending' || status === 'processing') {
    await new Promise(resolve => setTimeout(resolve, 5000)); // Wait 5 seconds

    const statusResponse = await fetch(`https://guardr.app/kallisto-osinter/api/job/${job_id}`);
    const jobData = await statusResponse.json();

    status = jobData.status;

    if (status === 'completed') {
      result = jobData.result;
      break;
    } else if (status === 'failed') {
      throw new Error(jobData.error);
    }

    // Show progress message to user
    console.log('OSINT analysis in progress...');
  }

  return result;
}
```

## React Hook Example

```tsx
import { useState, useEffect } from 'react';

function useOSINTAnalysis(name: string, location: string) {
  const [status, setStatus] = useState('idle');
  const [result, setResult] = useState(null);
  const [error, setError] = useState(null);

  const startAnalysis = async () => {
    try {
      setStatus('submitting');

      // Submit job
      const res = await fetch('https://guardr.app/kallisto-osinter/api/check-async', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ name, location })
      });

      const { job_id } = await res.json();
      setStatus('processing');

      // Poll for results
      const pollInterval = setInterval(async () => {
        const statusRes = await fetch(`https://guardr.app/kallisto-osinter/api/job/${job_id}`);
        const jobData = await statusRes.json();

        if (jobData.status === 'completed') {
          clearInterval(pollInterval);
          setResult(jobData.result);
          setStatus('completed');
        } else if (jobData.status === 'failed') {
          clearInterval(pollInterval);
          setError(jobData.error);
          setStatus('failed');
        }
      }, 5000);

    } catch (err) {
      setError(err.message);
      setStatus('failed');
    }
  };

  return { status, result, error, startAnalysis };
}
```

## Benefits

✅ **No timeouts** - API returns immediately (202 Accepted)
✅ **Better UX** - Show progress indicators and safety tips during wait
✅ **Reliable** - Long-running OSINT operations don't hit HTTP timeout limits
✅ **Scalable** - Background jobs can be distributed across workers

## Status Flow

```
idle → submitting → pending → processing → completed
                               ↓
                            failed
```

## Testing

```bash
# Submit job
JOB_ID=$(curl -s -X POST https://guardr.app/kallisto-osinter/api/check-async \
  -H 'Content-Type: application/json' \
  -d '{"name":"Test User","location":"New York"}' \
  | jq -r '.job_id')

echo "Job ID: $JOB_ID"

# Poll status (repeat until completed)
curl -s https://guardr.app/kallisto-osinter/api/job/$JOB_ID | jq '.status'

# Get results
curl -s https://guardr.app/kallisto-osinter/api/job/$JOB_ID | jq .
```

## Notes

- **Polling interval:** Recommended 5-10 seconds
- **Analysis time:** Typically 2-5 minutes depending on complexity
- **Job storage:** In-memory (jobs cleared on server restart - use Redis in production)
- **Rate limiting:** Same limits apply to job submission

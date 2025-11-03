# Frontend Timeout Fix - November 2, 2025

## Problem

The frontend was throwing errors when users performed searches because:
1. **No timeout on fetch requests** - Browser default timeout (~30 seconds) was too short
2. **OSINT operations take 60-120 seconds** - Intelligence X, breach databases, dark web scans
3. **Poor error messages** - Users didn't know what went wrong
4. **No user expectation management** - Users didn't know searches would take 1-2 minutes

## Solution

### 1. Added 2-Minute Timeout to API Calls

**File:** `website/src/app/page.tsx`

```typescript
const response = await fetch(`${apiUrl}/api/check`, {
  method: 'POST',
  headers: { 'Content-Type': 'application/json' },
  body: JSON.stringify({ name: demoName, location: demoLocation || undefined }),
  // 2-minute timeout for long-running OSINT searches
  signal: AbortSignal.timeout(120000)
});
```

### 2. Improved Error Handling

Added specific error handling for timeout vs network errors:

```typescript
catch (error) {
  if (error instanceof Error) {
    if (error.name === 'TimeoutError' || error.name === 'AbortError') {
      setDemoResults({
        error: 'Search timed out. OSINT operations can take up to 2 minutes. Please try again.'
      });
    } else {
      setDemoResults({ error: `Failed to connect to Guardr API: ${error.message}` });
    }
  }
}
```

### 3. Set User Expectations in UI

Added warning text above the demo form:

```
⏱️ Deep OSINT analysis takes 60-120 seconds. Please be patient while we
verify identity, check breaches, and scan the dark web.
```

Changed loading button text:
- **Before:** "Verifying Profile..."
- **After:** "Analyzing... (1-2 min)"

Added live progress message during loading:

```
🔍 Deep scanning in progress... This takes 60-120 seconds as we check
breach databases, scan the dark web, and verify identity claims.
```

### 4. Created Environment Configuration

**File:** `website/.env.local` (created)

```bash
NEXT_PUBLIC_API_URL=http://localhost:5000
```

This ensures the frontend knows where to find the backend API.

### 5. Additional Improvements

- Clear previous results when starting new search
- Check response status and throw meaningful errors
- Better error messages include the actual error details

## Testing

To test the fix:

```bash
# Terminal 1 - Start backend
cd ~/guardrV6
cargo run --bin guardr-api

# Terminal 2 - Start frontend
cd ~/guardrV6/website
npm run dev
```

Visit http://localhost:3000 and try the demo search. It should:
1. Show clear messaging that it takes 1-2 minutes
2. Wait up to 2 minutes for the API response
3. Show progress indicator while waiting
4. Display helpful error messages if something fails

## DigitalOcean Deployment Notes

When deploying to DigitalOcean, ensure:

1. Backend timeout is set to 120+ seconds:
   ```bash
   gunicorn -w 2 -b 0.0.0.0:8080 --timeout 120 guardr_api:app
   ```

2. Frontend environment variable points to internal API:
   ```bash
   NEXT_PUBLIC_API_URL=${guardr-api.PRIVATE_URL}
   ```

3. DigitalOcean App Platform timeout is configured for 120+ seconds

## Files Changed

- ✅ `website/src/app/page.tsx` - Added timeout and improved UX
- ✅ `website/.env.local` - Created with API URL
- ✅ `.gitignore` - Added `*.env.local` pattern
- ✅ `/home/nobby/guardrV6-clean/DEPRECATED.md` - Marked old directory as deprecated

## Migration Complete

- ✅ Working directory: `~/guardrV6/` (correct)
- ⚠️ Old directory: `~/guardrV6-clean/` (deprecated)

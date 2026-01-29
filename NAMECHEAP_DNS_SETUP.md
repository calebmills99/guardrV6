# Update DNS on Namecheap for guardr.app

## Your DigitalOcean URL
**Target:** `guardr-api-4c7ct.ondigitalocean.app`

## Step-by-Step Instructions

### 1. Log into Namecheap
1. Go to: https://www.namecheap.com/myaccount/login/
2. Log in with your credentials

### 2. Navigate to Domain List
1. Click **Domain List** in the left sidebar
2. Find **guardr.app**
3. Click **MANAGE** button next to it

### 3. Access Advanced DNS Settings
1. Click the **Advanced DNS** tab at the top
2. You'll see your current DNS records

### 4. Update/Add CNAME Record

#### Option A: Root Domain (guardr.app) - CNAME Flattening

**If Namecheap supports CNAME at root:**
1. Look for existing records with **Host** = `@` or blank
2. **Delete or modify** any A records or CNAME records for `@`
3. Click **Add New Record**
4. Set:
   - **Type:** CNAME Record
   - **Host:** `@` (for root domain)
   - **Value:** `guardr-api-4c7ct.ondigitalocean.app`
   - **TTL:** Automatic (or 1 min for testing, 5 min for production)

**If Namecheap doesn't allow CNAME at root (shows error):**
Use Option B instead.

#### Option B: Root Domain (guardr.app) - A Record (Recommended)

Since CNAME at root often doesn't work, use A records instead:

1. **Get the IP address from DigitalOcean:**
   ```bash
   # Run this command:
   dig guardr-api-4c7ct.ondigitalocean.app +short
   # Or:
   nslookup guardr-api-4c7ct.ondigitalocean.app
   ```

2. **Add A Record:**
   - **Type:** A Record
   - **Host:** `@` (for guardr.app)
   - **Value:** [IP address from above]
   - **TTL:** Automatic

3. **Add A Record for www:**
   - **Type:** A Record
   - **Host:** `www`
   - **Value:** [Same IP address]
   - **TTL:** Automatic

#### Option C: Best Practice - Use Both Domains

1. **Main domain A record:**
   - **Type:** A Record
   - **Host:** `@`
   - **Value:** [DigitalOcean App IP]
   - **TTL:** Automatic

2. **WWW subdomain CNAME:**
   - **Type:** CNAME Record
   - **Host:** `www`
   - **Value:** `guardr-api-4c7ct.ondigitalocean.app`
   - **TTL:** Automatic

### 5. Save Changes
1. Click **Save All Changes** button
2. Confirm the changes

### 6. Wait for DNS Propagation
- **Minimum:** 5-10 minutes
- **Typical:** 15-30 minutes
- **Maximum:** 24-48 hours (rare)

## How to Get the DigitalOcean IP

**Option 1: Using dig**
```bash
dig guardr-api-4c7ct.ondigitalocean.app +short
```

**Option 2: Using nslookup**
```bash
nslookup guardr-api-4c7ct.ondigitalocean.app
```

**Option 3: Online tool**
Visit: https://dnschecker.org/
Enter: `guardr-api-4c7ct.ondigitalocean.app`

## Test DNS Propagation

**While waiting, check progress:**
```bash
# Check if DNS has updated
dig guardr.app +short

# Check from multiple locations
# Visit: https://dnschecker.org/
# Enter: guardr.app
```

**What you should see:**
- Points to DigitalOcean App IP

## When DNS is Propagated

**Test your site:**
1. Visit: https://guardr.app
2. Should see your Guardr frontend!
3. Check: https://guardr.app/health
4. Test API: https://guardr.app/api/check

## Common Issues

### Issue 1: CNAME at Root Not Allowed
**Error:** "CNAME records cannot be used at the root domain"
**Solution:** Use Option B (A Records) instead

### Issue 2: SSL/HTTPS Not Working
**Problem:** DNS points to IP, but HTTPS shows error
**Solution:** Configure DigitalOcean to recognize custom domain:
1. Go to DO App Platform
2. Settings → Domains
3. Add `guardr.app` as custom domain
4. DO will auto-configure SSL (Let's Encrypt)

### Issue 3: Still Shows Old Site
**Cause:** DNS cache or browser cache
**Solutions:**
- Clear browser cache (Ctrl+Shift+R or Cmd+Shift+R)
- Try incognito/private mode
- Wait 5-30 minutes more
- Check DNS with: `dig guardr.app +short`

## Quick Reference Table

| Record Type | Host | Value | TTL |
|------------|------|-------|-----|
| A Record | `@` | [DO App IP] | Automatic |
| A Record | `www` | [DO App IP] | Automatic |
| OR CNAME | `www` | `guardr-api-4c7ct.ondigitalocean.app` | Automatic |

## Summary

**TL;DR:**
1. Log into Namecheap
2. Go to Domain List → guardr.app → Manage
3. Click Advanced DNS tab
4. Get DO IP: `dig guardr-api-4c7ct.ondigitalocean.app +short`
5. Add A Record: `@` → [IP address]
6. Add A Record: `www` → [IP address]
7. Save and wait 15-30 minutes
9. Visit guardr.app 🎉

Need the IP now? Run this:
```bash
dig guardr-api-4c7ct.ondigitalocean.app +short
```

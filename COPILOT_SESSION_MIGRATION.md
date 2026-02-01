# Copilot CLI Session Migration

## Current Session ID
```
fa0f7e4f-0f0f-4b38-9c41-8bd7f3ef5efa
```

## Windows Location
```
C:\Users\nobby.CALEBPC\.copilot\session-state\fa0f7e4f-0f0f-4b38-9c41-8bd7f3ef5efa
```

## Linux Location
```
~/.copilot/session-state/fa0f7e4f-0f0f-4b38-9c41-8bd7f3ef5efa
```

## Migration Steps

### From Windows to Linux:

1. Copy the session folder:
```bash
# On Windows (PowerShell), zip it:
Compress-Archive -Path "C:\Users\nobby.CALEBPC\.copilot\session-state\fa0f7e4f-0f0f-4b38-9c41-8bd7f3ef5efa" -DestinationPath "copilot-session.zip"

# Transfer to Linux via scp, USB, cloud, etc.
```

2. On Linux, extract to the right location:
```bash
mkdir -p ~/.copilot/session-state
unzip copilot-session.zip -d ~/.copilot/session-state/
```

3. Resume the session:
```bash
gh copilot-cli --resume fa0f7e4f-0f0f-4b38-9c41-8bd7f3ef5efa
```

## What's In The Session

- Checkpoints with prior work history
- Plan file (if created)
- Persistent files in `files/` folder
- Context about:
  - CI/CD fixes for deploy.yml
  - CORS configuration
  - Website setup and testing
  - Kallisto-OSINTer integration (pending)

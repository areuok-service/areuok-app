# Device Name and IMEI Implementation Guide

## Overview

This guide explains the implementation of device name management with uniqueness constraints and 15-day update cooldown, plus IMEI-based device binding.

## Feature Summary

### 1. IMEI Device Binding

**Purpose**: Allow devices to be bound to a physical device IMEI for account recovery.

**How it works**:
- IMEI is optional during device registration
- If provided and matches an existing device, returns that device (account recovery)
- If not provided or doesn't match, creates new device

**Use cases**:
- User reinstalls app on same device
- User switches devices but wants to keep same account
- Device identification and authentication

### 2. Device Name Uniqueness

**Purpose**: Ensure every device has a unique name across the system.

**How it works**:
- Database UNIQUE constraint on `device_name` column
- Registration validates name doesn't exist
- Name update validates new name doesn't exist
- Prevents duplicates at database level

**Use cases**:
- Search functionality relies on unique names
- Avoids confusion in supervision relationships
- Clear identification in UI

### 3. Device Name Update Cooldown

**Purpose**: Prevent frequent device name changes for stability.

**How it works**:
- `last_name_updated_at` tracks modification time
- 15-day cooldown period between updates
- First update has no cooldown
- Both client and server validate cooldown

**Use cases**:
- Prevents abuse of name changes
- Maintains stable identity
- Reduces system load

## Implementation Details

### Server-Side

#### Database Schema

```sql
-- Add IMEI for device binding
ALTER TABLE devices ADD COLUMN IF NOT EXISTS imei VARCHAR(255) UNIQUE;

-- Track last name update time
ALTER TABLE devices ADD COLUMN IF NOT EXISTS last_name_updated_at TIMESTAMPTZ;

-- Enforce name uniqueness
ALTER TABLE devices ADD CONSTRAINT devices_device_name_key UNIQUE (device_name);

-- Index IMEI for fast lookups
CREATE INDEX IF NOT EXISTS idx_devices_imei ON devices(imei);
```

#### API Endpoint: Register Device

```rust
// Check name uniqueness
let existing_name = sqlx::query!(
    r#"SELECT device_id FROM devices WHERE device_name = $1"#,
    req.device_name
).fetch_optional(&pool).await?;

if existing_name.is_some() {
    return Err(AppError::BadRequest("Device name already exists".to_string()));
}

// Check IMEI binding
if let Some(imei) = &req.imei {
    let existing_device = sqlx::query!(
        r#"SELECT device_id, device_name, imei, last_name_updated_at
           FROM devices WHERE imei = $1"#,
        imei
    ).fetch_optional(&pool).await?;

    if let Some(existing) = existing_device {
        // Return existing device (IMEI binding recovery)
        return Ok(Json(device));
    }
}

// Create new device
// ...
```

#### API Endpoint: Update Device Name

```rust
async fn update_device_name(
    State(pool): State<DbPool>,
    axum::extract::Path(id): axum::extract::Path<Uuid>,
    Json(req): Json<DeviceUpdateNameRequest>,
) -> Result<Json<Device>, AppError> {
    // Check name uniqueness
    let existing_name = sqlx::query!(
        r#"SELECT device_id FROM devices
           WHERE device_name = $1 AND device_id != $2"#,
        req.device_name,
        id
    ).fetch_optional(&pool).await?;

    if existing_name.is_some() {
        return Err(AppError::BadRequest("Device name already exists".to_string()));
    }

    // Check cooldown period
    let current_device = sqlx::query!(
        r#"SELECT last_name_updated_at FROM devices WHERE device_id = $1"#,
        id
    ).fetch_optional(&pool).await?;

    if let Some(last_updated) = current_device.last_name_updated_at {
        let now = Utc::now();
        let time_since_update = now.signed_duration_since(last_updated);

        if time_since_update.num_days() < 15 {
            return Err(AppError::BadRequest(
                format!("Device name cannot be updated. Last updated {} days ago. Minimum 15 days required.",
                    time_since_update.num_days())
            ));
        }
    }

    // Update name and timestamp
    sqlx::query!(
        r#"UPDATE devices
           SET device_name = $1, last_name_updated_at = NOW()
           WHERE device_id = $2"#,
        req.device_name,
        id
    ).execute(&pool).await?;

    Ok(Json(device))
}
```

### Client-Side

#### API Integration

```typescript
// Register device with optional IMEI
export const deviceApi = {
  register: async (deviceName: string, imei?: string, mode: DeviceMode = 'signin'): Promise<Device> => {
    return apiRequest<Device>('/devices/register', {
      method: 'POST',
      body: JSON.stringify({
        device_name: deviceName,
        imei: imei || null,
        mode,
      }),
    });
  },

  // Update device name
  updateName: async (deviceId: string, newName: string): Promise<Device> => {
    return apiRequest<Device>(`/devices/${deviceId}/name`, {
      method: 'PATCH',
      body: JSON.stringify({
        device_name: newName,
      }),
    });
  },
};
```

#### Local Storage

```typescript
export const storage = {
  // IMEI storage
  getDeviceImei: (): string | null => {
    return localStorage.getItem('areuok_device_imei');
  },

  setDeviceImei: (imei: string): void => {
    localStorage.setItem('areuok_device_imei', imei);
  },

  // Name update time tracking
  getLastNameUpdate: (): Date | null => {
    const value = localStorage.getItem('areuok_last_name_update');
    return value ? new Date(value) : null;
  },

  setLastNameUpdate: (date: Date): void => {
    localStorage.setItem('areuok_last_name_update', date.toISOString());
  },
};
```

#### Client-Side Validation

```typescript
function canUpdateName(): boolean {
  const lastUpdate = storage.getLastNameUpdate();
  if (!lastUpdate) return true;

  const daysSinceUpdate = Math.floor(
    (Date.now() - lastUpdate.getTime()) / (1000 * 60 * 60 * 24)
  );
  return daysSinceUpdate >= 15;
}

function getCooldownInfo(): { canUpdate: boolean; daysLeft: number } {
  const lastUpdate = storage.getLastNameUpdate();
  if (!lastUpdate) {
    return { canUpdate: true, daysLeft: 0 };
  }

  const daysSinceUpdate = Math.floor(
    (Date.now() - lastUpdate.getTime()) / (1000 * 60 * 60 * 24)
  );

  if (daysSinceUpdate >= 15) {
    return { canUpdate: true, daysLeft: 0 };
  }

  return {
    canUpdate: false,
    daysLeft: 15 - daysSinceUpdate
  };
}
```

#### UI Implementation

```svelte
<script>
  let showEditNameModal = $state(false);
  let newName = $state("");
  let editError = $state("");

  function canUpdateName(): boolean {
    const lastUpdate = storage.getLastNameUpdate();
    if (!lastUpdate) return true;

    const daysSinceUpdate = Math.floor((Date.now() - lastUpdate.getTime()) / (1000 * 60 * 60 * 24));
    return daysSinceUpdate >= 15;
  }

  async function handleNameUpdate() {
    if (!canUpdateName()) {
      editError = "Device name cannot be updated yet. Please wait 15 days from last update.";
      return;
    }

    try {
      const device = await deviceApi.updateName(deviceId, newName);
      storage.setDeviceName(device.device_name);
      storage.setLastNameUpdate(new Date());
      showEditNameModal = false;
    } catch (error) {
      editError = "Failed to update name: " + (error as Error).message;
    }
  }
</script>

<div class="edit-name-modal">
  <h3>Update Device Name</h3>

  <div class="cooldown-info">
    {#if canUpdateName()}
      <p class="success">✓ You can update your name now</p>
    {:else}
      {@const daysLeft = 15 - Math.floor((Date.now() - storage.getLastNameUpdate()!.getTime()) / (1000 * 60 * 60 * 24))}
      <p class="warning">⚠️ Please wait {daysLeft} more days</p>
    {/if}
  </div>

  <input bind:value={newName} placeholder="New device name" />
  <button onclick={handleNameUpdate} disabled={!canUpdateName()}>
    Update Name
  </button>
</div>
```

## Testing

### 1. IMEI Binding Test

```bash
# Register device with IMEI
curl -X POST http://localhost:3000/devices/register \
  -H "Content-Type: application/json" \
  -d '{
    "device_name": "Test Device",
    "imei": "123456789012345",
    "mode": "signin"
  }'

# Try to register with same IMEI (should return existing device)
curl -X POST http://localhost:3000/devices/register \
  -H "Content-Type: application/json" \
  -d '{
    "device_name": "Different Name",
    "imei": "123456789012345",
    "mode": "signin"
  }'

# Should return original device (not create new one)
```

### 2. Name Uniqueness Test

```bash
# Register first device
curl -X POST http://localhost:3000/devices/register \
  -H "Content-Type: application/json" \
  -d '{
    "device_name": "Unique Name",
    "mode": "signin"
  }'

# Try to register with same name (should fail)
curl -X POST http://localhost:3000/devices/register \
  -H "Content-Type: application/json" \
  -d '{
    "device_name": "Unique Name",
    "mode": "signin"
  }'

# Error: "Device name already exists"
```

### 3. Name Update Cooldown Test

```bash
DEVICE_ID="<your-device-id>"

# First update (should succeed)
curl -X PATCH http://localhost:3000/devices/$DEVICE_ID/name \
  -H "Content-Type: application/json" \
  -d '{"device_name": "Updated Name"}'

# Second update (should fail - cooldown)
curl -X PATCH http://localhost:3000/devices/$DEVICE_ID/name \
  -H "Content-Type: application/json" \
  -d '{"device_name": "Another Update"}'

# Error: "Device name cannot be updated. Last updated 0 days ago. Minimum 15 days required."
```

## Troubleshooting

### Issue: "Device name already exists"

**Cause**: Trying to use a name that's already taken by another device.

**Solution**: Choose a different device name.

### Issue: "Device name cannot be updated. Last updated X days ago"

**Cause**: Trying to update device name within 15-day cooldown period.

**Solution**: Wait until the cooldown period expires (15 days from last update).

### Issue: IMEI binding not working

**Cause**: IMEI not provided during registration or format incorrect.

**Solution**:
1. Ensure IMEI is a string, not null
2. Check IMEI matches existing device exactly
3. Verify database has IMEI column (run migrations)

### Issue: Cannot update name on first attempt

**Cause**: First name update should work without cooldown.

**Solution**:
1. Check `last_name_updated_at` is NULL in database
2. Verify cooldown logic doesn't block NULL values
3. Restart server to ensure latest code is running

## Migration Guide

### Upgrading Existing Installation

1. **Backup Database**
   ```bash
   docker-compose exec postgres pg_dump -U areuok areuok > backup.sql
   ```

2. **Run Migrations**
   ```bash
   docker-compose restart server
   ```
   Migrations run automatically on server startup.

3. **Update Client**
   ```bash
   cd areuok
   npm run build
   ```

4. **Test New Features**
   - Register a device with IMEI
   - Try to update device name
   - Verify cooldown enforcement

### Rolling Back

If issues occur:

1. Restore database from backup
2. Remove migration file: `rm crates/db/migrations/20260114_120000_add_device_imei.up.sql`
3. Restart server

## Security Considerations

### IMEI Privacy

- IMEI is stored in database (treat as sensitive)
- Never expose IMEI in API responses unless needed
- Consider encrypting IMEI in database for production

### Input Validation

- Always validate IMEI format (15 digits)
- Validate device name length (max 255 chars)
- Sanitize inputs to prevent SQL injection (sqlx handles this)

### Rate Limiting

Consider adding rate limiting to prevent abuse:
- Max name update attempts: 3 per day
- Max registration attempts: 5 per hour per IP

## Performance Optimization

### Database Indexes

- `imei` column is indexed for fast lookups
- `device_name` has unique constraint (implicitly indexed)
- Composite indexes on supervision queries

### Query Optimization

1. **IMEI Lookup**: Uses `idx_devices_imei` - O(log n)
2. **Name Search**: Uses unique constraint - O(log n)
3. **Cooldown Check**: Simple date comparison - O(1)

## Future Enhancements

### Potential Improvements

1. **Admin Override**: Allow admins to bypass cooldown
2. **Name Change History**: Track all name changes
3. **Verification**: Email verification before name change
4. **Appeal Process**: Allow users to request cooldown exemption
5. **IMEI Masking**: Partial display in UI for privacy

### Consideration

- **Case Sensitivity**: Currently case-sensitive, consider case-insensitive
- **Special Characters**: Consider allowing/restricting special chars in names
- **Cooldown Adjustment**: Allow admins to adjust cooldown period
- **IMEI Rotation**: Handle device replacement scenarios

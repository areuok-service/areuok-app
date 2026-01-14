# Changelog

All notable changes to areuok project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

#### Server (areuok-server)

- **IMEI Device Binding**
  - Added `imei` field to devices table with UNIQUE constraint
  - Devices can optionally bind to an IMEI for identification
  - IMEI binding allows device account recovery
  - Same IMEI retrieves existing device instead of creating new one

- **Device Name Uniqueness**
  - Added UNIQUE constraint on `device_name` column
  - Device names must be unique across all devices in the system
  - Registration and name updates validate name uniqueness

- **Device Name Update Cooldown**
  - Added `last_name_updated_at` field to track name modification time
  - Device names can only be updated once every 15 days
  - First name update has no cooldown restriction
  - Server validates cooldown before allowing updates

- **New API Endpoint**
  - `PATCH /devices/{id}/name` - Update device name with validation

#### Client (areuok)

- **IMEI Support**
  - Added IMEI field to Device interface
  - Device registration now includes optional IMEI parameter
  - Local storage includes IMEI persistence
  - Tauri command `get_device_imei` for retrieving device IMEI

- **Device Name Update UI**
  - Added edit button in dashboard header for device name modification
  - New modal dialog for name editing
  - Visual cooldown period indicator showing days since last update
  - Disabled editing when in cooldown period
  - Shows remaining days until next allowed update

- **Local Storage Enhancements**
  - Added `areuok_device_imei` for IMEI storage
  - Added `areuok_last_name_update` for tracking name update time
  - Updated `storage` object with new getters/setters

- **Tauri Backend Commands**
  - `get_device_imei` - Retrieve device IMEI
  - `set_device_imei` - Set device IMEI (for binding)

### Changed

#### Database Schema

- **devices table**:
  - Added `imei` VARCHAR(255) UNIQUE NULLABLE
  - Added `last_name_updated_at` TIMESTAMPTZ NULLABLE
  - Added UNIQUE constraint `devices_device_name_key` on `device_name`
  - Added UNIQUE constraint `devices_imei_key` on `imei`
  - Added index `idx_devices_imei` on `imei` column

#### API Behavior

- **POST /devices/register**:
  - Now accepts optional `imei` parameter
  - Validates device name uniqueness before registration
  - Returns existing device if IMEI matches (device recovery)
  - Sets `last_name_updated_at` to registration time for new devices

#### Data Models

- **Device Model** (Server):
  - Added `imei: Option<String>` field
  - Added `last_name_updated_at: Option<DateTime<Utc>>` field

- **Device Interface** (Client):
  - Added `imei?: string` field
  - Added `last_name_updated_at?: string` field

- **DeviceRegisterRequest** (Server):
  - Added `imei: Option<String>` field

- **DeviceInfo** (Tauri):
  - Added `imei: Option<String>` field

### Documentation

- Updated `/docs/api/device-management.md` with:
  - IMEI binding documentation
  - Device name uniqueness rules
  - Name update API with 15-day cooldown
  - Error scenarios and responses

- Updated `/docs/api/overview.md` with:
  - New PATCH endpoint for name updates
  - Business rules for name management
  - IMEI binding documentation
  - Updated error codes

- Created `/docs/database-schema.md` with:
  - Complete database schema documentation
  - Business rules and validation flows
  - IMEI binding and name update logic
  - Migration history
  - Performance considerations

- Updated `/docs/API_TESTING.md` with:
  - IMEI binding test cases
  - Device name update tests
  - Cooldown period tests
  - Duplicate name tests

- Updated `README.md` (Client) with:
  - New feature descriptions
  - Enhanced data storage documentation
  - IMEI and name update tracking info

- Updated `README.md` (Server) with:
  - New features section
  - Link to database schema documentation

- Updated `server.md` (Client) with:
  - Updated data structure design
  - New API endpoint documentation
  - Complete business rules documentation
  - Client implementation guidelines

### Migration

- **20260114_120000_add_device_imei.up.sql**:
  - Adds IMEI column to devices table
  - Adds last_name_updated_at column to devices table
  - Creates unique constraint on device_name
  - Creates unique constraint on imei
  - Creates index on imei for fast lookups

### Security & Validation

- Device names validated for uniqueness at database level
- IMEI validated for uniqueness at database level
- Name update cooldown enforced at server level
- Client-side pre-validation for better UX

### Breaking Changes

None. All changes are backward compatible.

### Migration Notes

To upgrade from previous version:

1. Run the new database migration:
   ```bash
   docker-compose exec server cargo run --bin server
   ```
   Or:
   ```sql
   -- Apply migration manually if needed
   ALTER TABLE devices ADD COLUMN IF NOT EXISTS imei VARCHAR(255) UNIQUE;
   ALTER TABLE devices ADD COLUMN IF NOT EXISTS last_name_updated_at TIMESTAMPTZ;
   ALTER TABLE devices ADD CONSTRAINT IF NOT EXISTS devices_device_name_key UNIQUE (device_name);
   CREATE INDEX IF NOT EXISTS idx_devices_imei ON devices(imei);
   ```

2. Rebuild client application to include new features

3. Client will automatically prompt for new features on next launch

### Known Issues

None

### Testing

All new features have been tested:
- IMEI binding and device recovery
- Device name uniqueness validation
- Name update cooldown enforcement
- API endpoint behavior
- Client UI interactions

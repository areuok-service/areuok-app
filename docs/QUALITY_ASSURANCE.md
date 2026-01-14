# Quality Assurance Guidelines

This document outlines the quality standards and automated checks for areuok projects.

## Table of Contents

- [Code Quality Standards](#code-quality-standards)
- [Linting and Formatting](#linting-and-formatting)
- [Type Safety](#type-safety)
- [Testing Standards](#testing-standards)
- [Code Review Process](#code-review-process)
- [Performance Guidelines](#performance-guidelines)
- [Security Guidelines](#security-guidelines)

## Code Quality Standards

### General Principles

1. **Readability** - Code should be easy to read and understand
2. **Simplicity** - Simple solutions over complex ones
3. **Maintainability** - Easy to modify and extend
4. **Consistency** - Follow existing patterns
5. **Testing** - Well-tested and documented

### Rust Server Code (areuok-server)

#### Naming Conventions

- **Functions**: `snake_case` - `register_device`, `get_device_status`
- **Types/Structs**: `PascalCase` - `Device`, `DeviceRegisterRequest`
- **Constants**: `SCREAMING_SNAKE_CASE` - `MAX_NAME_LENGTH`
- **Modules**: `snake_case` - `device_management`, `supervision`

```rust
// Good
pub fn register_device(/* ... */) -> Result<Device, AppError> {
    // ...
}

pub struct DeviceRegisterRequest {
    pub device_name: String,
    pub mode: DeviceMode,
}

pub const MAX_NAME_LENGTH: usize = 255;

// Bad
pub fn RegisterDevice(/* ... */) {} // wrong case
pub struct deviceRegister {} // wrong case
pub const max_name_length = 255; // wrong case
```

#### Error Handling

- Use `Result<T, E>` for operations that can fail
- Use `Option<T>` for optional values
- Provide meaningful error messages
- Handle errors at appropriate levels

```rust
// Good
pub async fn get_device(
    pool: &PgPool,
    device_id: Uuid,
) -> Result<Device, AppError> {
    sqlx::query_as!(/* ... */)
        .fetch_optional(pool)
        .await?
        .ok_or_else(|| AppError::NotFound("Device not found".to_string()))
}

// Bad
pub async fn get_device(
    pool: &PgPool,
    device_id: Uuid,
) -> Device { // No error handling
    // ...
}
```

#### Documentation

- Document public APIs with `///` doc comments
- Include examples for complex operations
- Document error conditions
- Keep docs up to date

```rust
/// Register a new device with optional IMEI binding.
///
/// Validates device name uniqueness and checks IMEI binding.
///
/// # Arguments
///
/// * `pool` - Database connection pool
/// * `req` - Device registration request
///
/// # Returns
///
/// * `Ok(Device)` - Successfully registered device
/// * `Err(AppError)` - Registration failed with error details
///
/// # Errors
///
/// * `BadRequest` - Device name already exists
/// * `Internal` - Database error
///
/// # Example
///
/// ```no_run
/// let device = register_device(&pool, request).await?;
/// println!("Registered: {}", device.device_name);
/// ```
pub async fn register_device(/* ... */) -> Result<Device, AppError> {
    // ...
}
```

### TypeScript/Svelte Client Code (areuok)

#### Naming Conventions

- **Variables/Functions**: `camelCase` - `deviceName`, `handleSubmit`
- **Types/Interfaces**: `PascalCase` - `Device`, `ApiResponse`
- **Constants**: `SCREAMING_SNAKE_CASE` - `API_BASE_URL`
- **Components**: `PascalCase` - `DeviceCard`, `SignInModal`

```typescript
// Good
const deviceName = "My Device";
function handleSubmit() {}

interface Device {
    device_id: string;
    device_name: string;
}

const API_BASE_URL = "http://localhost:3000";

// Bad
const device_name = "My Device"; // snake_case
function HandleSubmit() {} // PascalCase
interface device {} // snake_case
```

#### Type Safety

- Avoid `any` type
- Use specific types
- Use proper interfaces
- Type API responses

```typescript
// Good
interface Device {
    device_id: string;
    device_name: string;
    mode: DeviceMode;
}

function getDevice(id: string): Promise<Device> {
    return fetch(`/devices/${id}`).then(r => r.json());
}

// Bad
function getDevice(id: any): Promise<any> {
    // any types reduce type safety
}
```

#### Reactivity (Svelte 5)

- Use `$state` for reactive state
- Use `$derived` for computed values
- Avoid stores when local state suffices

```typescript
// Good
let count = $state(0);
let doubled = $derived(() => count * 2);

// Bad (old Svelte)
import { writable, derived } from 'svelte/store';
const count = writable(0);
const doubled = derived(count, $count => $count * 2);
```

## Linting and Formatting

### Server (Rust)

#### Configuration

Add to `.config/clippy.toml`:

```toml
# Linting
warn-on-all-wildcard-imports = true
warn-on-empty-enum-variants-with-underscore-digits = true
allow-expect-in-tests = true

# Complexity
cognitive-complexity-threshold = 30
```

#### Pre-commit Checks

```bash
cargo fmt --check
cargo clippy -- -D warnings
```

#### CI/CD Integration

```yaml
name: Rust CI

on: [push, pull_request]

jobs:
  check:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      - uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
      - name: Format check
        run: cargo fmt --all -- --check
      - name: Clippy
        run: cargo clippy -- -D warnings
      - name: Tests
        run: cargo test
```

### Client (TypeScript/Svelte)

#### ESLint Configuration

`.eslintrc.cjs`:

```javascript
module.exports = {
  root: true,
  extends: [
    'eslint:recommended',
    'plugin:@typescript-eslint/recommended',
    'prettier'
  ],
  parser: '@typescript-eslint/parser',
  plugins: ['@typescript-eslint'],
  rules: {
    'no-unused-vars': 'off',
    '@typescript-eslint/no-unused-vars': ['error', {
      argsIgnorePattern: '^_',
      varsIgnorePattern: '^_'
    }],
    '@typescript-eslint/no-explicit-any': 'error',
    'no-console': ['warn', { allow: ['warn', 'error'] }]
  }
};
```

#### Prettier Configuration

`.prettierrc`:

```json
{
  "semi": true,
  "singleQuote": true,
  "tabWidth": 2,
  "trailingComma": "es5",
  "printWidth": 100,
  "arrowParens": "avoid"
}
```

#### Pre-commit Checks

```bash
npm run lint
npm run format --check
npm run check
```

#### CI/CD Integration

```yaml
name: TypeScript CI

on: [push, pull_request]

jobs:
  check:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      - uses: actions/setup-node@v3
        with:
          node-version: 18
      - name: Install dependencies
        run: npm ci
      - name: Lint
        run: npm run lint
      - name: Type check
        run: npm run check
      - name: Format check
        run: npm run format --check
```

## Type Safety

### Rust

- Use strong types
- Avoid `unwrap()` and `expect()` in production code
- Use proper error handling
- Leverage type inference

```rust
// Good
let device: Result<Device, AppError> = fetch_device(&pool, id);

// Bad
let device: Device = fetch_device(&pool, id).unwrap(); // Can panic
```

### TypeScript

- Enable strict mode
- Use specific types
- Avoid `any`
- Use type guards

```typescript
// tsconfig.json
{
  "compilerOptions": {
    "strict": true,
    "noImplicitAny": true,
    "strictNullChecks": true
  }
}

// Good
function processValue(value: string | number): number {
  if (typeof value === 'string') {
    return parseInt(value, 10);
  }
  return value;
}

// Bad
function processValue(value: any): number {
  return value; // No type safety
}
```

## Testing Standards

### Coverage Goals

- **Unit Tests**: 80%+ coverage
- **Integration Tests**: All critical paths
- **E2E Tests**: Main user flows

### Test Organization

```
src/
  ├── lib/
  │   ├── api.ts
  │   └── api.test.ts      // Unit tests
  ├── routes/
  │   └── +page.svelte
  └── tests/
      ├── integration/         // Integration tests
      └── e2e/               // E2E tests
```

### Test Naming

- Descriptive test names
- Should-follow pattern: `should_doX_whenY`
- Group related tests

```rust
#[cfg(test)]
mod tests {
    #[test]
    fn should_reject_duplicate_device_name() {
        // ...
    }

    #[test]
    fn should_allow_first_name_update_without_cooldown() {
        // ...
    }
}
```

```typescript
describe('Device API', () => {
  it('should reject duplicate device name', async () => {
    // ...
  });

  it('should allow first name update without cooldown', async () => {
    // ...
  });
});
```

### Test Cases

Test all paths:
- Happy path (success)
- Error cases
- Edge cases
- Boundary conditions

```rust
#[test]
fn test_device_name_validation() {
    // Valid cases
    assert!(validate_name("Valid Name").is_ok());

    // Invalid cases
    assert!(validate_name("").is_err());
    assert!(validate_name("a".repeat(256)).is_err());
    assert!(validate_name("Invalid@Name!").is_err());
}
```

## Code Review Process

### Review Checklist

#### Functionality
- [ ] Implements the required feature
- [ ] Handles all edge cases
- [ ] Error handling is comprehensive
- [ ] No obvious bugs

#### Code Quality
- [ ] Follows coding standards
- [ ] Properly formatted
- [ ] No lint warnings
- [ ] Well-documented
- [ ] Type-safe

#### Testing
- [ ] Tests added/updated
- [ ] Tests cover main scenarios
- [ ] Tests pass
- [ ] Good coverage

#### Performance
- [ ] No obvious performance issues
- [ ] Efficient algorithms used
- [ ] No unnecessary allocations
- [ ] Database queries optimized

### Review Best Practices

1. **Be Constructive** - Focus on code, not the person
2. **Explain Why** - Provide reasoning for suggestions
3. **Suggest, Don't Dictate** - Offer alternatives
4. **Acknowledge Good** - Highlight what was done well
5. **Be Thorough** - Review all changed files

### Review Comments Format

```markdown
**Issue**: [Brief description]

**Suggestion**: [Proposed solution]

**Example**:
```rust
// Better approach
fn better_approach() {
    // ...
}
```

**Reasoning**: [Explain why this is better]
```

## Performance Guidelines

### Rust Server

#### Database Optimization

- Use indexes for frequently queried columns
- Avoid N+1 queries
- Use prepared statements (sqlx)
- Fetch only needed columns

```rust
// Good - specific columns
sqlx::query!(
    r#"SELECT device_id, device_name FROM devices WHERE imei = $1"#,
    imei
)

// Bad - fetch all
sqlx::query_as!(Device, r#"SELECT * FROM devices"#)
```

#### Memory Management

- Avoid unnecessary clones
- Use references when possible
- Release resources promptly

```rust
// Good
fn process_device(device: &Device) { /* ... */ }

// Bad
fn process_device(device: Device) { /* Clone not needed */ }
```

### TypeScript Client

#### Reactivity Optimization

- Use `$derived` for computed values
- Avoid re-renders
- Use keys in lists

```svelte
<!-- Good - derived computed -->
<script>
let items = $state([/* ... */]);
let visibleItems = $derived(() => items.filter(i => i.visible));
</script>

{#each visibleItems as item (item.id)}
  <div>{item.name}</div>
{/each}

<!-- Bad - computed in template -->
<script>
let items = $state([/* ... */]);
let visibleItems = items.filter(i => i.visible);
</script>
```

#### Bundle Size

- Code splitting for routes
- Lazy load heavy components
- Optimize dependencies

## Security Guidelines

### Input Validation

- Validate all user inputs
- Sanitize data
- Use prepared statements
- Limit input lengths

```rust
// Good
if device_name.len() > MAX_NAME_LENGTH || device_name.is_empty() {
    return Err(AppError::BadRequest("Invalid device name".to_string()));
}

// Bad
let device_name = req.device_name; // No validation
```

```typescript
// Good
function validateDeviceName(name: string): boolean {
  return name.length > 0 && name.length <= 255;
}

// Bad
function saveDeviceName(name: string) {
  // No validation
}
```

### Error Handling

- Don't expose internal errors to clients
- Log technical errors
- Return user-friendly messages

```rust
// Good
Err(AppError::Internal("Database error".to_string())) // Logged internally
// Client sees: {"error": "Internal server error"}

// Bad
Err(AppError::Internal(format!("Connection failed: {}", db_error))) // Exposes internals
```

### Secret Management

- Never commit secrets
- Use environment variables
- Use secret managers for production
- Rotate secrets regularly

```bash
# Good
export DATABASE_URL=postgresql://...
export API_KEY=secret_key_here

# Bad - Never do this
DATABASE_URL = "postgresql://..."
API_KEY = "secret_key_here"
```

## Continuous Improvement

### Metrics to Track

- Code coverage
- Test pass rate
- Lint warnings
- Build time
- Deployment success rate

### Regular Reviews

- Monthly code quality reviews
- Quarterly security audits
- Performance monitoring
- User feedback analysis

## Tools and Automation

### Pre-commit Hooks

#### Server (Rust)

`.git/hooks/pre-commit`:

```bash
#!/bin/bash
cargo fmt --all -- --check
if [ $? -ne 0 ]; then
    echo "Code needs formatting. Run 'cargo fmt'"
    exit 1
fi

cargo clippy -- -D warnings
if [ $? -ne 0 ]; then
    echo "Clippy found issues. Fix them before committing."
    exit 1
fi
```

#### Client (TypeScript)

`.git/hooks/pre-commit`:

```bash
#!/bin/bash
npm run lint
if [ $? -ne 0 ]; then
    echo "Linting failed. Fix issues before committing."
    exit 1
fi

npm run check
if [ $? -ne 0 ]; then
    echo "Type check failed. Fix types before committing."
    exit 1
fi
```

### Quality Gates

All PRs must pass:
1. **Formatting Check** - Code properly formatted
2. **Lint Check** - No lint warnings
3. **Type Check** - No type errors
4. **Unit Tests** - All pass
5. **Integration Tests** - All pass
6. **Build Check** - Successfully builds

## Resources

- [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)
- [Svelte Style Guide](https://svelte.dev/docs#style-guide)
- [TypeScript Handbook](https://www.typescriptlang.org/docs/)
- [SQLX Documentation](https://docs.rs/sqlx/)
- [ESLint Rules](https://eslint.org/docs/rules/)

---

**Quality is everyone's responsibility!** ✨

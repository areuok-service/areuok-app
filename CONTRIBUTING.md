# Contributing to areuok

Thank you for your interest in contributing to areuok! This document provides guidelines and instructions for contributing.

## Table of Contents

- [Code of Conduct](#code-of-conduct)
- [Getting Started](#getting-started)
- [Development Workflow](#development-workflow)
- [Coding Standards](#coding-standards)
- [Commit Message Guidelines](#commit-message-guidelines)
- [Pull Request Process](#pull-request-process)
- [Testing Guidelines](#testing-guidelines)
- [Documentation](#documentation)

## Code of Conduct

- Be respectful and inclusive
- Welcome newcomers and help them learn
- Focus on what is best for the community
- Show empathy towards other community members

## Getting Started

### Prerequisites

- Node.js 18+
- Rust 1.70+ (for Tauri backend)
- npm or pnpm
- Git

### Setup

1. **Fork and Clone**
   ```bash
   # Fork the repository on GitHub
   git clone https://github.com/YOUR_USERNAME/areuok.git
   cd areuok
   ```

2. **Install Dependencies**
   ```bash
   npm install
   # or
   pnpm install
   ```

3. **Start Development Server**
   ```bash
   npm run tauri dev
   ```

## Development Workflow

### Branching Strategy

- `main` - Production-ready code
- `develop` - Integration branch for features
- `feature/*` - New features
- `bugfix/*` - Bug fixes
- `hotfix/*` - Critical production fixes

### Workflow Steps

1. **Create Feature Branch**
   ```bash
   git checkout develop
   git pull origin develop
   git checkout -b feature/your-feature-name
   ```

2. **Make Changes**
   - Write code following [Coding Standards](#coding-standards)
   - Add/update tests
   - Update documentation

3. **Test Locally**
   ```bash
   # Run linter
   npm run lint

   # Format code
   npm run format

   # Type check
   npm run check
   ```

4. **Commit Changes**
   - Follow [Commit Message Guidelines](#commit-message-guidelines)

5. **Push and Create PR**
   ```bash
   git push origin feature/your-feature-name
   ```
   Then create a Pull Request on GitHub

## Coding Standards

### TypeScript/Svelte Style

- Follow [Svelte Style Guide](https://svelte.dev/docs#style-guide)
- Use TypeScript for type safety
- Use Svelte 5 runes ($state, $derived) for reactivity
- Avoid any types (`: any`)
- Use meaningful variable and function names

### Code Organization

- `src/lib/` - Shared utilities, API, storage
- `src/routes/` - SvelteKit pages
- `src-tauri/src/` - Tauri backend (Rust)
- `src/i18n/` - Internationalization files

### State Management

- Use Svelte 5 runes for local state
- Use localStorage for persistence
- Avoid global state unless necessary

```typescript
// Good: Use Svelte 5 runes
let count = $state(0);
let doubleCount = $derived(() => count * 2);

// Bad: Old Svelte stores
import { writable } from 'svelte/store';
const count = writable(0);
```

### API Integration

- Use centralized API methods in `src/lib/api.ts`
- Handle errors appropriately
- Show user-friendly error messages

```typescript
// Good
try {
  const device = await deviceApi.register(name, mode);
  // handle success
} catch (error) {
  alert("Registration failed: " + error.message);
}

// Bad
const response = await fetch(url);
// no error handling
```

### UI/UX Best Practices

- Mobile-first design
- Accessibility considerations (ARIA labels, keyboard navigation)
- Loading states for async operations
- Error messages for failed operations
- Consistent with existing design patterns

### Tauri Backend

- Document commands with `///` doc comments
- Handle errors gracefully
- Use Result types for error handling

```rust
/// Update device name.
///
/// # Arguments
///
/// * `name` - New device name
///
/// # Returns
///
/// Returns `DeviceConfig` or error message
#[tauri::command]
fn update_device_name(name: String) -> Result<DeviceConfig, String> {
    // implementation
}
```

### Error Handling

- Display user-friendly error messages
- Log technical errors for debugging
- Provide actionable guidance when possible

```typescript
// Good
catch (error) {
  console.error("API error:", error);
  alert("Failed to connect to server. Please check your internet connection.");
}

// Bad
catch (error) {
  alert(error);
}
```

## Commit Message Guidelines

### Format

```
<type>(<scope>): <subject>

<body>

<footer>
```

### Type

- `feat` - New feature
- `fix` - Bug fix
- `docs` - Documentation changes
- `style` - Code style changes (formatting, etc.)
- `refactor` - Code refactoring
- `perf` - Performance improvements
- `test` - Adding or updating tests
- `chore` - Maintenance tasks

### Scope

- `frontend` - Frontend changes (Svelte/TypeScript)
- `backend` - Tauri backend changes (Rust)
- `ui` - UI/UX changes
- `api` - API integration changes
- `i18n` - Internationalization
- `docs` - Documentation

### Examples

```
feat(frontend): add device name edit modal with cooldown display

- Add edit button in dashboard header
- Implement modal dialog for name editing
- Show days since last update
- Disable editing during 15-day cooldown

Fixes #123
```

```
fix(backend): resolve IMEI type mismatch in device config

- Update DeviceInfo struct to include optional imei field
- Fix type error in get_device_imei command
- Add proper error handling for missing IMEI

Closes #456
```

### Rules

- Use imperative mood ("add" not "added")
- Limit first line to 72 characters
- Reference related issues
- Explain what and why, not how

## Pull Request Process

### Before Creating PR

1. **Checklist**
   - [ ] Code follows style guidelines
   - [ ] Linter passes
   - [ ] TypeScript compiles without errors
   - [ ] UI looks correct
   - [ ] No console errors
   - [ ] Documentation updated

2. **Self-Review**
   ```bash
   # Lint
   npm run lint

   # Type check
   npm run check

   # Format
   npm run format

   # Build
   npm run build
   ```

3. **Test Changes**
   - Test in development mode
   - Test UI in browser
   - Test Tauri commands
   - Test mobile responsiveness

### Creating PR

1. **Title**: Follow commit message format
2. **Description**: Include:
   - What PR does
   - Why it's needed
   - Screenshots (if UI changes)
   - Related issues
   - Breaking changes

3. **Template**
   ```markdown
   ## Description
   Brief description of changes.

   ## Type of Change
   - [ ] Bug fix
   - [ ] New feature
   - [ ] Breaking change
   - [ ] Documentation update

   ## Testing
   - [ ] Tested in development mode
   - [ ] Tested UI in browser
   - [ ] Tested mobile responsiveness
   - [ ] No console errors

   ## Screenshots
   (Add screenshots for UI changes)

   ## Checklist
   - [ ] Code follows style guidelines
   - [ ] Linter passes
   - [ ] Self-reviewed code
   - [ ] Documentation updated
   - [ ] No new warnings
   - [ ] Tests added for new functionality
   ```

### Review Process

1. **Auto-Checks**: CI runs automatically
2. **Code Review**: At least one approval required
3. **Changes Requested**: Address feedback and update PR
4. **Approval**: PR approved and ready to merge

### After Approval

1. **Squash Commits** (if needed)
   ```bash
   git rebase -i HEAD~n
   ```

2. **Update Branch**
   ```bash
   git checkout develop
   git pull origin develop
   git merge feature/your-feature
   ```

3. **Push**
   ```bash
   git push origin develop
   ```

## Testing Guidelines

### Frontend Tests

- Test UI components
- Test user interactions
- Test error handling
- Test responsive design

### Tauri Tests

- Test backend commands
- Test file operations
- Test error cases

### Manual Testing

- Test all major user flows
- Test on different screen sizes
- Test error scenarios
- Test with different browsers (if applicable)

### Running Tests

```bash
# Lint
npm run lint

# Type check
npm run check

# Format
npm run format

# Build
npm run build
```

## Documentation

### When to Update Docs

- Adding new features
- Changing UI/UX
- Modifying API integration
- Updating configuration
- Adding translations

### Documentation Files

- `README.md` - Project overview
- `CONTRIBUTING.md` - This file
- `server.md` - Design and architecture
- Internationalization files in `src/i18n/`

### Writing Style

- Clear and concise
- Include examples
- Update existing, don't duplicate
- Use markdown formatting

## Quality Assurance

### Linting

```bash
# Run linter
npm run lint

# Fix auto-fixable issues
npm run lint --fix
```

### Type Checking

```bash
# Type check all files
npm run check
```

### Formatting

```bash
# Format all files
npm run format

# Check formatting
npm run format --check
```

### Building

```bash
# Development build
npm run tauri dev

# Production build
npm run build
```

## Getting Help

- **Issues**: Report bugs or request features
- **Discussions**: Ask questions or discuss ideas
- **Pull Requests**: Submit code changes

## Release Process

Only maintainers should handle releases:

1. **Version**: Follow semantic versioning
2. **Changelog**: Update CHANGELOG.md
3. **Tag**: Create git tag
4. **Release**: Create GitHub release
5. **Build**: Create release binaries

## License

By contributing, you agree that your contributions will be licensed under the same license as the project.

---

**Thank you for contributing to areuok!** 🎉

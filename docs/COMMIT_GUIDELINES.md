# Commit Guidelines

This document outlines commit message standards and guidelines for areuok projects.

## Table of Contents

- [Overview](#overview)
- [Commit Message Format](#commit-message-format)
- [Types](#types)
- [Scopes](#scopes)
- [Examples](#examples)
- [Best Practices](#best-practices)
- [Git Hooks](#git-hooks)

## Overview

Good commit messages:
- Help understand why a change was made
- Make maintenance and code review easier
- Improve project history
- Generate better changelogs

Bad commit messages:
- Are vague ("fixed bug")
- Don't provide context
- Are too long
- Don't follow the format

## Commit Message Format

### Structure

```
<type>(<scope>): <subject>

<body>

<footer>
```

### Format Rules

1. **Subject Line**
   - Maximum 72 characters
   - Imperative mood ("add" not "added")
   - No period at end
   - Reference issue number if applicable

2. **Body**
   - Explains what and why
   - What changed
   - Why the change is needed
   - Can be multiple lines
   - Wrapped at 72 characters

3. **Footer**
   - Optional
   - Reference related issues
   - Breaking changes
   - Co-authored commits

### Template

```
feat(api): add device name update endpoint

Add PATCH /devices/{id}/name endpoint for updating
device names with validation.

Validates:
- Device name uniqueness across all devices
- 15-day cooldown period between updates
- First update has no cooldown

Add last_name_updated_at field to devices table
to track modification time.

Fixes #123
```

## Types

Use the following types:

| Type | Description |
|------|-------------|
| `feat` | New feature |
| `fix` | Bug fix |
| `docs` | Documentation changes |
| `style` | Code style changes (formatting, semi-colons, etc.) |
| `refactor` | Code refactoring (neither fixes bug nor adds feature) |
| `perf` | Performance improvements |
| `test` | Adding or updating tests |
| `chore` | Maintenance tasks (updating deps, etc.) |
| `ci` | CI/CD changes |
| `build` | Build system or dependencies changes |

### Type Guidelines

- **feat**: For new user-facing features
- **fix**: For bug fixes
- **docs**: Only for documentation files (README, docs/)
- **style**: Only for code style, no logic changes
- **refactor**: Restructuring existing code, same behavior
- **perf**: Performance improvements without logic changes
- **test**: Test additions or updates
- **chore**: Routine tasks, updates
- **ci**: CI/CD configuration changes
- **build**: Build system or dependency changes

## Scopes

Scopes should help categorize the commit.

### Server (areuok-server)

| Scope | Description |
|-------|-------------|
| `db` | Database schema, migrations, queries |
| `api` | API endpoints, handlers, routing |
| `models` | Data structures, types |
| `server` | Server configuration, main entry point |
| `auth` | Authentication and authorization |
| `validation` | Input validation logic |

### Client (areuok)

| Scope | Description |
|-------|-------------|
| `frontend` | Frontend UI components and logic |
| `backend` | Tauri backend (Rust) commands |
| `api` | API integration, HTTP requests |
| `ui` | UI/UX changes, styling |
| `i18n` | Internationalization |
| `state` | State management |
| `storage` | Local storage, persistence |

## Examples

### Feature Addition

```
feat(api): add IMEI device binding for account recovery

Add optional IMEI field to device registration.
If IMEI matches existing device, return that device
instead of creating a new one.

This allows users to recover their accounts on
the same physical device after reinstalling the app.

Add get_device_imei Tauri command to retrieve device IMEI.

Fixes #456
```

### Bug Fix

```
fix(db): resolve NULL value constraint on IMEI column

UNIQUE constraint was rejecting multiple NULL values.
Update migration to use NULLS NOT DISTINCT for IMEI uniqueness.

This allows multiple devices without IMEI to be created
while still enforcing uniqueness for devices with IMEI.

Closes #789
```

### Documentation

```
docs(readme): update setup instructions for new features

Add instructions for:
- IMEI device binding
- Device name update cooldown
- Name uniqueness validation

Update examples and prerequisites.

Related to #456
```

### Refactoring

```
refactor(api): extract device validation logic into separate module

Move validation functions to new validation module
to improve code organization and reusability.

No functional changes, just code structure improvements.
```

### Performance

```
perf(db): add index on IMEI column for faster lookups

Create idx_devices_imei index to optimize IMEI-based
device retrieval queries during registration.

Improves registration performance by ~50%.

Fixes #123
```

### Testing

```
test(api): add integration tests for device name update

Add test cases for:
- Name uniqueness validation
- 15-day cooldown enforcement
- First update without cooldown
- Duplicate name rejection

Covers API endpoint PATCH /devices/{id}/name.
```

## Best Practices

### DO

- **Do** write commit messages in present tense
- **Do** explain what and why, not how
- **Do** reference related issue numbers
- **Do** limit the subject line to 72 characters
- **Do** start subject with capital letter
- **Do** use imperative mood ("add" not "added")
- **Do** wrap body at 72 characters

### DON'T

- **Don't** use commit messages like "fixed bug"
- **Don't** include "and" in subject (use body instead)
- **Don't** use ALL CAPS in subject
- **Don't** end subject with period
- **Don't** make multiple unrelated changes in one commit
- **Don't** use generic messages like "update code"
- **Don't** include sensitive data in commit messages

### Multiple Changes

If you have multiple unrelated changes:

**Bad**:
```
feat: add device name update and fix IMEI bug and update README
```

**Good** - Split into multiple commits:
```
feat(api): add device name update with cooldown validation

fix(db): resolve IMEI NULL value constraint issue

docs(readme): update setup for new features
```

## Breaking Changes

If your change introduces breaking changes:

1. **Mark clearly in footer**
   ```
   BREAKING CHANGE: Device name format now requires minimum 3 characters
   ```

2. **Include migration guide**
   ```
   BREAKING CHANGE: API endpoint path changed from /device to /devices

Migration guide:
- Update API base URL in client configuration
- Change all /device/* paths to /devices/*
   ```

3. **Reference related issues**
   ```
   BREAKING CHANGE: Database schema migration required

See migration guide in docs/database-migrations.md
Related issues: #123, #456
   ```

## Git Hooks

### Pre-commit Hook

Implement pre-commit hooks to enforce commit standards:

#### Server (areuok-server)

`.git/hooks/commit-msg`:

```bash
#!/bin/bash

COMMIT_MSG_FILE=$1
COMMIT_MSG=$(cat "$COMMIT_MSG_FILE")

# Check subject line length
SUBJECT_LINE=$(echo "$COMMIT_MSG" | head -n 1)
if [ ${#SUBJECT_LINE} -gt 72 ]; then
    echo "Error: Commit subject line exceeds 72 characters"
    echo "Current: $SUBJECT_LINE"
    exit 1
fi

# Check for conventional commit format
if ! echo "$COMMIT_MSG" | grep -qE "^(feat|fix|docs|style|refactor|perf|test|chore|ci|build)\(.*\):"; then
    echo "Error: Commit message must follow conventional commit format"
    echo "Format: <type>(<scope>): <subject>"
    exit 1
fi

exit 0
```

#### Client (areuok)

`.git/hooks/commit-msg`:

```bash
#!/bin/bash

COMMIT_MSG_FILE=$1
COMMIT_MSG=$(cat "$COMMIT_MSG_FILE")

# Check subject line length
SUBJECT_LINE=$(echo "$COMMIT_MSG" | head -n 1)
if [ ${#SUBJECT_LINE} -gt 72 ]; then
    echo "Error: Commit subject line exceeds 72 characters"
    echo "Current: $SUBJECT_LINE"
    exit 1
fi

# Check for conventional commit format
if ! echo "$COMMIT_MSG" | grep -qE "^(feat|fix|docs|style|refactor|perf|test|chore|ci|build)\(.*\):"; then
    echo "Error: Commit message must follow conventional commit format"
    echo "Format: <type>(<scope>): <subject>"
    exit 1
fi

exit 0
```

### Installing Hooks

```bash
# Make hooks executable
chmod +x .git/hooks/commit-msg

# Copy to actual hooks directory
cp .git/hooks/commit-msg .git/hooks/commit-msg.sample
```

Or use a tool:

```bash
# Install conventional commits CLI
npm install -g @commitlint/cli @commitlint/config-conventional

# Install husky for git hooks
npm install husky

# Setup husky
npx husky install
```

### Commitlint Configuration

`.commitlintrc.json`:

```json
{
  "extends": ["@commitlint/config-conventional"],
  "rules": {
    "type-enum": [
      2,
      "always",
      ["feat", "fix", "docs", "style", "refactor", "perf", "test", "chore", "ci", "build"]
    ],
    "subject-case": [2, "never", ["sentence-case", "start-case", "pascal-case", "upper-case"]],
    "subject-empty": [2, "never"],
    "subject-full-stop": [2, "never", "."],
    "body-max-line-length": [2, "always", 72],
    "subject-max-length": [2, "always", 72]
  }
}
```

### Husky Configuration

`package.json` (client):

```json
{
  "husky": {
    "hooks": {
      "commit-msg": "commitlint -E HUSKY_GIT_PARAMS"
    }
  }
}
```

## Tools and Automation

### Recommended Tools

- **commitlint**: Enforce commit message format
- **husky**: Git hooks made easy
- **conventional-changelog**: Generate changelogs from commits
- **gitlint**: Lint commit messages

### Installation

```bash
# Install CLI tools
npm install -g commitlint @commitlint/cli
npm install -g conventional-changelog-cli

# For Python tests
pip install gitlint
```

### Usage

```bash
# Lint a commit message
echo "feat(api): add new endpoint" | commitlint

# Lint last commit
commitlint --from HEAD~1

# Generate changelog from commits
conventional-changelog -p angular -i CHANGELOG.md
```

## Interactive Commit Tools

### Using Commitizen

```bash
# Install commitizen
npm install -g commitizen cz-conventional-changelog

# Configure git to use commitizen
git config --local commit.template .git/cz_template

# Or use globally
npx cz
```

This provides an interactive prompt for creating properly formatted commits.

### Using Gitmoji

```bash
# Install gitmoji
npm install -g gitmoji-cli

# Interactive gitmoji
gitmoji -c

# Example output
🎨 feat(api): add device name update endpoint
```

## Commit History Management

### Squashing Commits

Before merging, clean up commit history:

```bash
# Interactive rebase
git rebase -i HEAD~n

# Mark commits as:
# - pick: include as-is
# - squash: combine with previous commit
# - fixup: like squash, but keep message from previous
```

### Amending Commits

```bash
# Add forgotten file
git add forgotten-file.txt

# Amend last commit (don't do this if already pushed)
git commit --amend --no-edit
```

### Undoing Commits

```bash
# Undo last commit (keep changes)
git reset --soft HEAD~1

# Undo last 2 commits (keep changes)
git reset --soft HEAD~2

# Undo last commit (discard changes)
git reset --hard HEAD~1
```

## Commit Message Review

### Before Committing

Ask yourself:

1. **Is the subject line clear and concise?**
2. **Does the subject line start with type/scope?**
3. **Is the subject under 72 characters?**
4. **Is the body well-formatted?**
5. **Does the body explain what and why?**
6. **Is the footer included if needed?**
7. **Is this commit atomic?** (one logical change)

### Review Checklist

- [ ] Subject line is under 72 characters
- [ ] Subject uses imperative mood
- [ ] Subject starts with type/scope
- [ ] Subject doesn't end with period
- [ ] Body explains what and why
- [ ] Body lines wrapped at 72 characters
- [ ] Related issues are referenced
- [ ] Breaking changes are documented
- [ ] Commit is atomic (one logical change)

## Examples by Type

### feat
```
feat(api): add device name uniqueness validation

Enforce that device names must be unique across all devices.
Reject registration if name already exists.

Add UNIQUE constraint on device_name column in database.

Fixes #123
```

### fix
```
fix(backend): resolve IMEI type mismatch in device config

Update DeviceInfo struct to include optional imei field
to match database schema and prevent type errors.

Closes #456
```

### docs
```
docs(api): update device management API documentation

Add documentation for:
- IMEI device binding
- Device name uniqueness rules
- Name update cooldown period
- API request/response examples

Fixes #789
```

### style
```
style(api): apply cargo fmt to device registration handler

No functional changes, just code formatting.
```

### refactor
```
refactor(db): extract validation logic into separate module

Move device name and IMEI validation into dedicated
validation module for better code organization.

No functional changes.
```

### perf
```
perf(api): optimize device search query with better indexing

Create composite index on (device_name, mode) to improve
search performance by reducing query time from 100ms to 50ms.

Fixes #123
```

### test
```
test(api): add integration tests for IMEI binding

Add test cases for:
- Device recovery with existing IMEI
- New device creation with new IMEI
- IMEI uniqueness enforcement
- NULL IMEI handling

Covers #456
```

### chore
```
chore(deps): update sqlx to 0.7.0

Update sqlx dependency to latest version for
better database query compilation and type safety.

No functional changes.
```

### ci
```
ci(github): add formatting check to CI workflow

Add cargo fmt --all -- --check step to ensure
code is properly formatted before merge.

Related to #123
```

## Resources

- [Conventional Commits](https://www.conventionalcommits.org/)
- [Commitlint](https://commitlint.js.org/)
- [Husky](https://github.com/typicode/husky)
- [Git Hooks](https://git-scm.com/book/en/v2/Customizing-Git-Git-Hooks)
- [Commitizen](https://github.com/commitizen/cz-cli)

---

**Make your commits count!** ✨

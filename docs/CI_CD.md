# CI/CD Configuration and Workflow

This document describes the Continuous Integration and Continuous Deployment setup for areuok projects.

## Table of Contents

- [Overview](#overview)
- [Server CI/CD (areuok-server)](#server-cicd-areuok-server)
- [Client CI/CD (areuok)](#client-cicd-areuok)
- [Quality Gates](#quality-gates)
- [Deployment Pipeline](#deployment-pipeline)
- [Troubleshooting](#troubleshooting)

## Overview

### CI/CD Goals

- **Automated Testing**: Run tests on every commit
- **Quality Checks**: Enforce code style and linting
- **Build Verification**: Ensure code builds successfully
- **Security Scanning**: Scan for vulnerabilities
- **Automated Deployment**: Deploy on merge to main

### Platforms

- **GitHub Actions**: Primary CI/CD platform
- **Docker**: Containerized builds and deployment
- **PostgreSQL**: Database for integration tests

## Server CI/CD (areuok-server)

### Workflow Files

#### `.github/workflows/ci.yml`

```yaml
name: CI

on:
  push:
    branches: [main, develop]
  pull_request:
    branches: [main, develop]

env:
  CARGO_TERM_COLOR: always
  RUST_BACKTRACE: 1

jobs:
  check:
    name: Code Quality Checks
    runs-on: ubuntu-latest

    steps:
      - name: Checkout
        uses: actions/checkout@v3

      - name: Cache Rust
        uses: actions/cache@v3
        with:
          path: |
            ~/.cargo/registry
            ~/.cargo/git
            target
          key: ${{ runner.os }}-cargo-${{ hashFiles('**/Cargo.lock') }}
          restore-keys: |
            ${{ runner.os }}-cargo-

      - name: Install Rust Toolchain
        uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
          components: rustfmt, clippy

      - name: Format Check
        run: cargo fmt --all -- --check

      - name: Clippy Lint
        run: cargo clippy -- -D warnings -- -W clippy::all

      - name: Build Check
        run: cargo build --all --locked

      - name: Unit Tests
        run: cargo test --all --no-fail-fast

  test-integration:
    name: Integration Tests
    runs-on: ubuntu-latest
    needs: check

    services:
      postgres:
        image: postgres:15
        env:
          POSTGRES_PASSWORD: test_password
          POSTGRES_DB: areuok_test
        ports:
          - 5432:5432
        options: >-
          --health-cmd pg_isready
          --health-interval 10s
          --health-timeout 5s
          --health-retries 5

    env:
      DATABASE_URL: postgresql://postgres:test_password@localhost:5432/areuok_test

    steps:
      - name: Checkout
        uses: actions/checkout@v3

      - name: Cache Rust
        uses: actions/cache@v3
        with:
          path: |
            ~/.cargo/registry
            ~/.cargo/git
            target
          key: ${{ runner.os }}-cargo-${{ hashFiles('**/Cargo.lock') }}

      - name: Install Rust Toolchain
        uses: actions-rs/toolchain@v1
        with:
          toolchain: stable

      - name: Run Migrations
        run: |
          cargo install sqlx-cli
          sqlx database create --database-url $DATABASE_URL
          sqlx migrate run --database-url $DATABASE_URL

      - name: Integration Tests
        run: cargo test --test integration --no-fail-fast

  security-scan:
    name: Security Scanning
    runs-on: ubuntu-latest
    needs: check

    steps:
      - name: Checkout
        uses: actions/checkout@v3

      - name: Cargo Audit
        run: cargo audit

      - name: Dependency Check
        run: cargo tree --duplicates
```

#### `.github/workflows/release.yml`

```yaml
name: Release

on:
  push:
    tags:
      - 'v*'

env:
  CARGO_TERM_COLOR: always

jobs:
  build-release:
    name: Build Release
    runs-on: ubuntu-latest

    steps:
      - name: Checkout
        uses: actions/checkout@v3

      - name: Cache Rust
        uses: actions/cache@v3
        with:
          path: |
            ~/.cargo/registry
            ~/.cargo/git
            target
          key: ${{ runner.os }}-cargo-${{ hashFiles('**/Cargo.lock') }}

      - name: Install Rust Toolchain
        uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
          target: x86_64-unknown-linux-gnu

      - name: Build Release Binary
        run: cargo build --release --target x86_64-unknown-linux-gnu

      - name: Upload Binary
        uses: actions/upload-artifact@v3
        with:
          name: areuok-server-linux
          path: target/x86_64-unknown-linux-gnu/release/areuok-server

      - name: Create Release
        uses: softprops/action-gh-release@v1
        if: startsWith(github.ref, 'refs/tags/')
        with:
          files: target/x86_64-unknown-linux-gnu/release/areuok-server
        env:
          GITHUB_TOKEN: ${{ secrets.GITHUB_TOKEN }}
```

## Client CI/CD (areuok)

### Workflow Files

#### `.github/workflows/ci.yml`

```yaml
name: CI

on:
  push:
    branches: [main, develop]
  pull_request:
    branches: [main, develop]

jobs:
  check:
    name: Code Quality Checks
    runs-on: ubuntu-latest

    steps:
      - name: Checkout
        uses: actions/checkout@v3

      - name: Setup Node.js
        uses: actions/setup-node@v3
        with:
          node-version: 18

      - name: Get npm cache directory
        id: npm-cache-dir
        run: echo "dir=$(npm config get cache)" >> $GITHUB_OUTPUT

      - name: Cache node modules
        uses: actions/cache@v3
        with:
          path: ${{ steps.npm-cache-dir.outputs.dir }}
          key: ${{ runner.os }}-node-${{ hashFiles('**/package-lock.json') }}
          restore-keys: |
            ${{ runner.os }}-node-

      - name: Install Dependencies
        run: npm ci

      - name: ESLint
        run: npm run lint

      - name: Type Check
        run: npm run check

      - name: Format Check
        run: npm run format --check

      - name: Build
        run: npm run build

  test-unit:
    name: Unit Tests
    runs-on: ubuntu-latest
    needs: check

    steps:
      - name: Checkout
        uses: actions/checkout@v3

      - name: Setup Node.js
        uses: actions/setup-node@v3
        with:
          node-version: 18

      - name: Cache node modules
        uses: actions/cache@v3
        with:
          path: ${{ steps.npm-cache-dir.outputs.dir }}
          key: ${{ runner.os }}-node-${{ hashFiles('**/package-lock.json') }}

      - name: Install Dependencies
        run: npm ci

      - name: Run Unit Tests
        run: npm run test:unit

  test-e2e:
    name: E2E Tests
    runs-on: ubuntu-latest
    needs: check

    services:
      server:
        image: areuok-server:latest
        ports:
          - 3000:3000

    steps:
      - name: Checkout
        uses: actions/checkout@v3

      - name: Setup Node.js
        uses: actions/setup-node@v3
        with:
          node-version: 18

      - name: Cache node modules
        uses: actions/cache@v3
        with:
          path: ${{ steps.npm-cache-dir.outputs.dir }}
          key: ${{ runner.os }}-node-${{ hashFiles('**/package-lock.json') }}

      - name: Install Dependencies
        run: npm ci

      - name: Install Playwright
        run: npx playwright install --with-deps

      - name: Run E2E Tests
        run: npm run test:e2e

      - name: Upload Test Results
        uses: actions/upload-artifact@v3
        if: always()
        with:
          name: playwright-report
          path: playwright-report/

  security-scan:
    name: Security Scanning
    runs-on: ubuntu-latest
    needs: check

    steps:
      - name: Checkout
        uses: actions/checkout@v3

      - name: Setup Node.js
        uses: actions/setup-node@v3
        with:
          node-version: 18

      - name: Cache node modules
        uses: actions/cache@v3
        with:
          path: ${{ steps.npm-cache-dir.outputs.dir }}
          key: ${{ runner.os }}-node-${{ hashFiles('**/package-lock.json') }}

      - name: Install Dependencies
        run: npm ci

      - name: npm Audit
        run: npm audit --audit-level=moderate

      - name: Dependabot Check
        uses: actions/dependency-review-action@v3
```

#### `.github/workflows/release.yml`

```yaml
name: Release

on:
  push:
    tags:
      - 'v*'

jobs:
  build-and-upload:
    name: Build and Upload Release
    runs-on: ubuntu-latest
    strategy:
      matrix:
        os: [macos-latest, ubuntu-latest, windows-latest]

    steps:
      - name: Checkout
        uses: actions/checkout@v3

      - name: Setup Node.js
        uses: actions/setup-node@v3
        with:
          node-version: 18

      - name: Get npm cache directory
        id: npm-cache-dir
        run: echo "dir=$(npm config get cache)" >> $GITHUB_OUTPUT

      - name: Cache node modules
        uses: actions/cache@v3
        with:
          path: ${{ steps.npm-cache-dir.outputs.dir }}
          key: ${{ runner.os }}-node-${{ hashFiles('**/package-lock.json') }}

      - name: Install Dependencies
        run: npm ci

      - name: Install Rust
        uses: actions-rs/toolchain@v1
        with:
          toolchain: stable

      - name: Install Tauri CLI
        run: cargo install tauri-cli

      - name: Build Tauri App
        run: npm run tauri build

      - name: Upload Release Assets
        uses: actions/upload-artifact@v3
        with:
          name: areuok-${{ matrix.os }}
          path: src-tauri/target/release/bundle/

      - name: Create GitHub Release
        uses: softprops/action-gh-release@v1
        with:
          files: src-tauri/target/release/bundle/**/*
        env:
          GITHUB_TOKEN: ${{ secrets.GITHUB_TOKEN }}
```

## Quality Gates

### Pull Request Requirements

All PRs must pass:

1. **Code Quality**
   - [ ] Format check passes
   - [ ] Lint check passes (0 warnings)
   - [ ] Type check passes

2. **Testing**
   - [ ] Unit tests pass
   - [ ] Integration tests pass
   - [ ] E2E tests pass (if applicable)

3. **Security**
   - [ ] No high/critical vulnerabilities
   - [ ] Dependency audit passes
   - [ ] No leaked secrets

4. **Build**
   - [ ] Debug build succeeds
   - [ ] Release build succeeds

### Status Checks

#### Protected Branch Rules

**Main branch protection** requires:
- At least 1 approval
- All status checks pass
- No merge conflicts
- Up to date with base branch

**Develop branch protection** requires:
- All status checks pass
- No merge conflicts

## Deployment Pipeline

### Staging Deployment

**Trigger**: Merge to `develop`

1. **Build Docker Image**
   ```bash
   docker build -t areuok-server:develop .
   ```

2. **Push to Registry**
   ```bash
   docker tag areuok-server:develop registry.com/areuok-server:develop
   docker push registry.com/areuok-server:develop
   ```

3. **Deploy to Staging**
   ```bash
   kubectl set image deployment/areuok-server areuok-server=registry.com/areuok-server:develop
   ```

### Production Deployment

**Trigger**: Tagged release

1. **Build Release Assets**
   - Compiled binaries
   - Docker images
   - Documentation

2. **Run Tests**
   - Full test suite
   - Integration tests
   - Security scans

3. **Deploy to Production**
   ```bash
   # Server
   docker tag areuok-server:v1.2.3 registry.com/areuok-server:latest
   docker push registry.com/areuok-server:latest

   # Client
   Upload binaries to releases
   Publish to app stores
   ```

4. **Post-Deployment**
   - Health checks
   - Smoke tests
   - Monitor metrics

## Monitoring

### CI/CD Metrics

- **Build Time**: Track and optimize
- **Test Duration**: Identify slow tests
- **Success Rate**: Track flaky tests
- **Deployment Time**: Monitor deployment speed

### Alerts

- **Build Failures**: Immediate notification
- **Test Failures**: After 3 consecutive failures
- **Security Issues**: Immediate notification
- **Deployment Issues**: Immediate notification

## Troubleshooting

### Common CI Issues

#### Cache Issues

**Problem**: Cache becomes stale

**Solution**:
```yaml
- name: Clear Cache
  run: rm -rf ~/.cargo/registry
```

#### Timeouts

**Problem**: Tests timeout

**Solution**:
- Increase timeout in workflow
- Optimize slow tests
- Split into smaller test suites

#### Flaky Tests

**Problem**: Tests sometimes fail

**Solution**:
- Add retries
- Fix race conditions
- Improve test isolation
```yaml
- name: Tests with Retry
  uses: nick-fields/retry-action@v2
  with:
    timeout_minutes: 10
    max_attempts: 3
    command: npm test
```

### Local CI Simulation

Run CI checks locally:

```bash
# Server
cargo fmt --all -- --check
cargo clippy -- -D warnings
cargo test --all

# Client
npm run lint
npm run check
npm run format --check
npm run test:unit
```

## Best Practices

### Workflow Optimization

1. **Caching**: Cache dependencies and build artifacts
2. **Parallel Jobs**: Run independent checks in parallel
3. **Fail Fast**: Stop on first failure
4. **Resource Limits**: Set appropriate resource limits

### Security

1. **Secret Management**: Use GitHub Secrets
2. **Dependabot**: Enable automated dependency updates
3. **CodeQL**: Enable security scanning
4. **Audit**: Regular security audits

### Maintainability

1. **Document Workflows**: Explain complex steps
2. **Modular Workflows**: Reusable actions
3. **Clear Naming**: Descriptive job and step names
4. **Version Control**: Pin major versions

## Resources

- [GitHub Actions Documentation](https://docs.github.com/en/actions)
- [Docker Best Practices](https://docs.docker.com/develop/dev-best-practices/)
- [CI/CD Patterns](https://www.cicdpatterns.com/)
- [Testing Best Practices](https://testingjavascript.com/)

---

**Happy Shipping!** 🚀

# Documentation Index

Welcome to the areuok project documentation. This index helps you navigate through all available documentation.

## Project Documentation

### Server (areuok-server)

| Document | Description |
|----------|-------------|
| [README.md](./areuok-server/README.md) | Project overview, setup, and quick start guide |
| [CONTRIBUTING.md](./areuok-server/CONTRIBUTING.md) | Contribution guidelines and workflow |
| [docs/api/device-management.md](./areuok-server/docs/api/device-management.md) | Device management API documentation |
| [docs/api/overview.md](./areuok-server/docs/api/overview.md) | API overview and common patterns |
| [docs/database-schema.md](./areuok-server/docs/database-schema.md) | Database schema and design |
| [docs/API_TESTING.md](./areuok-server/docs/API_TESTING.md) | API testing examples |
| [test/README.md](./areuok-server/test/README.md) | Testing framework and guidelines |

### Client (areuok)

| Document | Description |
|----------|-------------|
| [README.md](./areuok/README.md) | Project overview, features, and setup guide |
| [CONTRIBUTING.md](./areuok/CONTRIBUTING.md) | Contribution guidelines and workflow |
| [server.md](./areuok/server.md) | Design specifications and architecture |

## Shared Documentation

| Document | Description |
|----------|-------------|
| [CHANGELOG.md](./CHANGELOG.md) | Version history and changes |
| [QUALITY_ASSURANCE.md](./QUALITY_ASSURANCE.md) | Quality standards and best practices |
| [CI_CD.md](./CI_CD.md) | CI/CD configuration and workflow |
| [COMMIT_GUIDELINES.md](./COMMIT_GUIDELINES.md) | Commit message standards and guidelines |
| [IMPLEMENTATION_GUIDE.md](./IMPLEMENTATION_GUIDE.md) | IMEI and device name implementation guide |

## Documentation by Topic

### Getting Started

- **Server**: [Server README](./areuok-server/README.md) → Quick Start
- **Client**: [Client README](./areuok/README.md) → Development Setup

### API Documentation

- **Device Management**: [Device Management API](./areuok-server/docs/api/device-management.md)
- **API Overview**: [API Overview](./areuok-server/docs/api/overview.md)
- **API Testing**: [API Testing Guide](./areuok-server/docs/API_TESTING.md)

### Database

- **Schema Documentation**: [Database Schema](./areuok-server/docs/database-schema.md)
- **Migration Guide**: [Implementation Guide](./IMPLEMENTATION_GUIDE.md) → Migration Instructions

### Features

- **IMEI Binding**: [Implementation Guide](./IMPLEMENTATION_GUIDE.md) → IMEI Device Binding
- **Device Name Management**: [Implementation Guide](./IMPLEMENTATION_GUIDE.md) → Device Name Management
- **Name Uniqueness**: [API Overview](./areuok-server/docs/api/overview.md) → Device Name Constraints

### Development

- **Contribution Guidelines**: [Server Contributing](./areuok-server/CONTRIBUTING.md), [Client Contributing](./areuok/CONTRIBUTING.md)
- **Quality Standards**: [Quality Assurance](./QUALITY_ASSURANCE.md)
- **Commit Guidelines**: [Commit Guidelines](./COMMIT_GUIDELINES.md)
- **CI/CD Workflow**: [CI/CD Configuration](./CI_CD.md)

### Testing

- **Server Testing**: [API Testing Guide](./areuok-server/docs/API_TESTING.md)
- **Test Framework**: [Test README](./areuok-server/test/README.md)
- **Testing Standards**: [Quality Assurance](./QUALITY_ASSURANCE.md) → Testing Guidelines

## Quick Links

### For New Contributors

1. Read [Contribution Guidelines](./areuok-server/CONTRIBUTING.md)
2. Review [Quality Standards](./QUALITY_ASSURANCE.md)
3. Learn [Commit Guidelines](./COMMIT_GUIDELINES.md)
4. Check [CI/CD Setup](./CI_CD.md)

### For Developers

1. **API Integration**: [API Overview](./areuok-server/docs/api/overview.md)
2. **Database Design**: [Database Schema](./areuok-server/docs/database-schema.md)
3. **Implementation Guide**: [Implementation Guide](./IMPLEMENTATION_GUIDE.md)

### For Maintainers

1. **Release Process**: [Change Log](./CHANGELOG.md)
2. **CI/CD Management**: [CI/CD Configuration](./CI_CD.md)
3. **Quality Gates**: [Quality Assurance](./QUALITY_ASSURANCE.md)

## Documentation Maintenance

### When to Update Docs

- **New Feature**: Document in appropriate location
- **API Change**: Update API documentation
- **Breaking Change**: Document migration path
- **Bug Fix**: Update if affects documented behavior
- **Deprecation**: Mark features as deprecated

### Documentation Standards

- **Clear and Concise**: Easy to understand
- **Examples**: Provide working examples
- **Up-to-Date**: Keep synchronized with code
- **Cross-Reference**: Link to related documentation
- **Structured**: Use proper markdown formatting

### Documentation Review

Before merging documentation changes:

- [ ] Links are valid
- [ ] Code examples work
- [ ] Tables and lists are properly formatted
- [ ] Cross-references are correct
- [ ] Grammar and spelling checked

## Versioning

Documentation versioning follows semantic versioning:

- **Major**: Breaking changes
- **Minor**: New features, backward compatible
- **Patch**: Bug fixes

## Help and Support

### Getting Help

- **Issues**: Report documentation issues in GitHub Issues
- **PRs**: Contribute improvements via Pull Requests
- **Discussions**: Ask questions in GitHub Discussions

### Reporting Documentation Issues

When reporting documentation issues:

1. **Which document**: Link to document
2. **What's wrong**: Describe the issue
3. **Expected behavior**: What you expect
4. **Current behavior**: What you see
5. **Steps to reproduce**: How to encounter the issue

## Document Structure

```
areuok/
├── README.md                          # Main project index (this file)
├── CHANGELOG.md                       # Version history
├── QUALITY_ASSURANCE.md              # Quality standards
├── CI_CD.md                           # CI/CD setup
├── COMMIT_GUIDELINES.md                # Commit standards
├── IMPLEMENTATION_GUIDE.md             # Feature implementation guide
├── areuok-server/
│   ├── README.md                       # Server project README
│   ├── CONTRIBUTING.md                # Server contribution guide
│   ├── docs/
│   │   ├── api/
│   │   │   ├── device-management.md   # Device management API
│   │   │   └── overview.md         # API overview
│   │   ├── database-schema.md         # Database documentation
│   │   └── API_TESTING.md           # API testing guide
│   └── test/
│       └── README.md                  # Testing framework guide
└── areuok/
    ├── README.md                       # Client project README
    ├── CONTRIBUTING.md                # Client contribution guide
    └── server.md                     # Design specifications
```

## API Quick Reference

### Server Endpoints

| Method | Endpoint | Description | Documentation |
|--------|----------|-------------|---------------|
| POST | `/devices/register` | Register device with optional IMEI | [Device Management](./areuok-server/docs/api/device-management.md) |
| GET | `/devices/{id}` | Get device information | [Device Management](./areuok-server/docs/api/device-management.md) |
| PATCH | `/devices/{id}/name` | Update device name | [Device Management](./areuok-server/docs/api/device-management.md) |
| GET | `/search/devices` | Search devices by name | [Device Management](./areuok-server/docs/api/device-management.md) |
| POST | `/devices/{id}/signin` | Record device sign-in | [Device Management](./areuok-server/docs/api/device-management.md) |
| GET | `/devices/{id}/status` | Get device status | [Device Management](./areuok-server/docs/api/device-management.md) |
| POST | `/supervision/request` | Create supervision request | [API Overview](./areuok-server/docs/api/overview.md) |
| GET | `/supervision/pending/{id}` | Get pending requests | [API Overview](./areuok-server/docs/api/overview.md) |
| POST | `/supervision/accept` | Accept supervision request | [API Overview](./areuok-server/docs/api/overview.md) |
| POST | `/supervision/reject` | Reject supervision request | [API Overview](./areuok-server/docs/api/overview.md) |
| GET | `/supervision/list/{id}` | List supervision relations | [API Overview](./areuok-server/docs/api/overview.md) |
| DELETE | `/supervision/{relation_id}` | Remove supervision relation | [API Overview](./areuok-server/docs/api/overview.md) |

## Database Schema Quick Reference

### Tables

| Table | Description | Documentation |
|-------|-------------|---------------|
| `devices` | Device information with IMEI and name tracking | [Database Schema](./areuok-server/docs/database-schema.md) |
| `supervision_requests` | Supervision relationship requests | [Database Schema](./areuok-server/docs/database-schema.md) |
| `supervision_relations` | Active supervision relationships | [Database Schema](./areuok-server/docs/database-schema.md) |
| `signin_records` | Device sign-in history and streak data | [Database Schema](./areuok-server/docs/database-schema.md) |

## Key Concepts

### IMEI Binding

- **Purpose**: Device identification and account recovery
- **Implementation**: [Implementation Guide](./IMPLEMENTATION_GUIDE.md) → IMEI Device Binding

### Device Name Management

- **Uniqueness**: Global unique names enforced
- **Cooldown**: 15-day update limit
- **Implementation**: [Implementation Guide](./IMPLEMENTATION_GUIDE.md) → Device Name Management

### Supervision System

- **Requests**: Pending supervision requests
- **Relations**: Active supervision relationships
- **Workflow**: [API Overview](./areuok-server/docs/api/overview.md) → Supervision Management

## Contributing to Documentation

### How to Contribute

1. **Identify Issue**: Find missing or incorrect documentation
2. **Propose Fix**: Create issue or PR with proposed changes
3. **Follow Standards**: Match existing documentation style
4. **Review Changes**: Ensure accuracy and completeness

### Documentation PR Template

```markdown
## Description

Brief description of documentation changes.

## Type of Change

- [ ] New documentation
- [ ] Update existing documentation
- [ ] Fix incorrect documentation
- [ ] Reorganize documentation

## Files Changed

List of modified documentation files.

## Testing

- [ ] Links verified
- [ ] Examples tested
- [ ] Cross-references checked

## Related Issues

Closes #[issue-number]
```

## External Resources

### Rust

- [Rust Book](https://doc.rust-lang.org/book/)
- [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)
- [SQLX Documentation](https://docs.rs/sqlx/)

### Svelte

- [Svelte Documentation](https://svelte.dev/docs)
- [Svelte 5 Documentation](https://svelte.dev/docs/run)

### TypeScript

- [TypeScript Handbook](https://www.typescriptlang.org/docs/)
- [TypeScript Deep Dive](https://basarat.gitbook.io/typescript/)

### Tauri

- [Tauri Documentation](https://tauri.app/v1/guides/)

### PostgreSQL

- [PostgreSQL Documentation](https://www.postgresql.org/docs/)

## Changelog

Latest version documentation changes are tracked in [CHANGELOG.md](./CHANGELOG.md).

---

**Happy documenting!** 📚

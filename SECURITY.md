# Security Policy

## Reporting a Vulnerability

If you discover a security vulnerability within Evoury, please send an email to security@evoury.app. All security vulnerabilities will be promptly addressed.

**Please do not report security vulnerabilities through public GitHub issues.**

## Supported Versions

| Version | Supported          |
| ------- | ------------------ |
| 0.1.x   | :white_check_mark: |

## Security Update Process

When a security vulnerability is reported:

1. **Acknowledgment**: We will acknowledge receipt of your vulnerability report within 48 hours.

2. **Assessment**: Our security team will assess the vulnerability and determine its impact.

3. **Fix Development**: We will develop a fix for the vulnerability.

4. **Testing**: The fix will be thoroughly tested to ensure it resolves the issue without introducing new problems.

5. **Release**: A security update will be released as soon as possible.

6. **Disclosure**: We will publicly disclose the vulnerability after the fix has been released.

## Security Measures

### Application Security

- **Input Validation**: All user inputs are validated and sanitized
- **SQL Injection Prevention**: Parameterized queries are used throughout
- **File System Security**: Proper file permission checks and sandboxing
- **Memory Safety**: Rust's memory safety guarantees prevent common vulnerabilities

### Data Security

- **Local Storage**: All data is stored locally on the user's machine
- **No Cloud Dependencies**: Evoury works entirely offline
- **Encryption**: Sensitive data is encrypted at rest
- **Secure Deletion**: File deletion follows secure deletion practices

### Build Security

- **Dependency Auditing**: Regular audits of dependencies for vulnerabilities
- **Reproducible Builds**: Ensuring build integrity
- **Code Signing**: Released binaries are code-signed

## Best Practices for Users

1. **Keep Updated**: Always use the latest version of Evoury
2. **Download from Official Sources**: Only download from official releases
3. **Report Issues**: If you find a security issue, report it responsibly
4. **Review Permissions**: Be mindful of file system permissions

## Bug Bounty Program

We are considering implementing a bug bounty program in the future. Stay tuned for updates.

## Contact

For security-related inquiries, please contact:
- Email: security@evoury.app
- PGP Key: [Link to PGP key] (coming soon)

---

*This policy is subject to change. Last updated: September 2026*

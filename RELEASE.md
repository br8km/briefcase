# Briefcase Release Configuration

## Release Process

This document describes the automated release process for Briefcase.

### Automated Releases

Briefcase uses GitHub Actions to automatically create releases when code is pushed to the `main` branch.

#### Release Artifacts

The CI pipeline builds binaries for multiple platforms:
- `briefcase-linux-x64.tar.gz` - Linux x86_64
- `briefcase-macos-x64.tar.gz` - macOS Intel
- `briefcase-macos-arm64.tar.gz` - macOS Apple Silicon
- `briefcase-windows-x64.zip` - Windows x64

#### Release Notes

Release notes are automatically generated from:
- Pull request titles and descriptions
- Commit messages using conventional commit format
- Changelog entries

### Manual Release Process

If needed, releases can be created manually:

1. **Update Version**: Update version in `Cargo.toml`
2. **Update Changelog**: Add new version entry to `CHANGELOG.md`
3. **Create Tag**: `git tag v1.x.x`
4. **Push Tag**: `git push origin v1.x.x`
5. **GitHub Release**: Create release on GitHub with artifacts

### Pre-release Checklist

- [ ] All tests pass
- [ ] Code formatted and clippy clean
- [ ] Documentation updated
- [ ] Changelog updated
- [ ] Version bumped appropriately
- [ ] Breaking changes documented
- [ ] CI pipeline passes

### Versioning

Briefcase follows [Semantic Versioning](https://semver.org/):
- **MAJOR**: Breaking changes
- **MINOR**: New features (backward compatible)
- **PATCH**: Bug fixes (backward compatible)

### Distribution Channels

- **GitHub Releases**: Primary distribution channel
- **Package Managers**: Future consideration (brew, apt, etc.)
- **Docker Images**: Future consideration
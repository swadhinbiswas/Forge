# Branch Protection Rules

## Overview

This document defines the branch protection rules for the ForgeDesk repository. These rules enforce code quality, security, and release stability.

## Branch: `main`

### Rules

| Rule | Setting | Description |
|------|---------|-------------|
| **Require pull request** | ✅ Enabled | All changes must go through a PR |
| **Required approvals** | 1 (2 for core) | Minimum review count |
| **Dismiss stale reviews** | ✅ Enabled | New pushes dismiss old approvals |
| **Require code owner review** | ✅ Enabled | CODEOWNERS must approve |
| **Require status checks** | ✅ Enabled | CI must pass |
| **Require up-to-date branch** | ✅ Enabled | Branch must be current with main |
| **Include administrators** | ✅ Enabled | Rules apply to admins too |
| **Restrict force pushes** | ✅ Enabled | No force push allowed |
| **Restrict deletions** | ✅ Enabled | Cannot delete main branch |

### Required Status Checks

The following checks must pass before merging to `main`:

```
✅ Rust (fmt, clippy, test)
✅ Python (ubuntu-latest, 3.14)
✅ Python (macos-latest, 3.14)
✅ Python (windows-latest, 3.14)
✅ Node.js
✅ Version Alignment
✅ Security Scan
```

### How to Configure (GitHub UI)

1. Go to **Settings** → **Branches**
2. Click **Add rule** for branch name pattern: `main`
3. Configure:
   - ✅ Require a pull request before merging
   - ✅ Require approvals (set to 1)
   - ✅ Dismiss stale pull request approvals when new commits are pushed
   - ✅ Require review from Code Owners
   - ✅ Require status checks to pass before merging
   - ✅ Require branches to be up to date before merging
   - Search and add: `Rust`, `Python`, `Node.js`, `Version Alignment`, `Security Scan`
   - ✅ Do not allow bypassing the above settings
   - ✅ Restrict who can push to matching branches (leave empty = nobody)
   - ✅ Allow force pushes (DISABLED)
   - ✅ Allow deletions (DISABLED)

### How to Configure (GitHub CLI)

```bash
# Install GitHub CLI if not already installed
# https://cli.github.com/

# Set branch protection for main
gh api repos/{owner}/{repo}/branches/main/protection \
  --method PUT \
  --field required_status_checks='{"strict":true,"contexts":["Rust","Python (ubuntu-latest, 3.14)","Python (macos-latest, 3.14)","Python (windows-latest, 3.14)","Node.js","Version Alignment","Security Scan"]}' \
  --field enforce_admins=true \
  --field required_pull_request_reviews='{"required_approving_review_count":1,"dismiss_stale_reviews":true,"require_code_owner_reviews":true}' \
  --field restrictions=null \
  --field allow_force_pushes=false \
  --field allow_deletions=false
```

### How to Configure (Terraform)

```hcl
resource "github_branch_protection" "main" {
  repository_id = github_repository.forge.node_id
  pattern       = "main"

  enforce_admins = true

  required_status_checks {
    strict   = true
    contexts = [
      "Rust",
      "Python (ubuntu-latest, 3.14)",
      "Python (macos-latest, 3.14)",
      "Python (windows-latest, 3.14)",
      "Node.js",
      "Version Alignment",
      "Security Scan",
    ]
  }

  required_pull_request_reviews {
    required_approving_review_count = 1
    dismiss_stale_reviews           = true
    require_code_owner_reviews      = true
  }

  allows_force_pushes = false
  allows_deletions    = false
}
```

---

## Branch: `release/**`

### Rules

| Rule | Setting | Description |
|------|---------|-------------|
| **Require pull request** | ✅ Enabled | All changes must go through a PR |
| **Required approvals** | 1 | Minimum review count |
| **Dismiss stale reviews** | ✅ Enabled | New pushes dismiss old approvals |
| **Require status checks** | ✅ Enabled | CI must pass |
| **Include administrators** | ✅ Enabled | Rules apply to admins too |
| **Restrict force pushes** | ✅ Enabled | No force push allowed |

### Required Status Checks

Same as `main` branch.

---

## Branch Naming Conventions

| Prefix | Purpose | Example |
|--------|---------|---------|
| `feat/` | New features | `feat/add-dark-mode` |
| `fix/` | Bug fixes | `fix/ipc-timeout` |
| `docs/` | Documentation | `docs/update-api-reference` |
| `chore/` | Maintenance | `chore/update-dependencies` |
| `test/` | Test additions | `test/add-e2e-tests` |
| `refactor/` | Code refactoring | `refactor/simplify-bridge` |
| `perf/` | Performance | `perf/optimize-ipc` |
| `ci/` | CI/CD changes | `ci/add-security-scan` |
| `release/` | Release branches | `release/v3.1.0` |
| `hotfix/` | Critical fixes | `hotfix/security-patch` |

---

## Commit Message Standards

We enforce **Conventional Commits**:

```
<type>(<scope>): <description>

[optional body]

[optional footer]
```

### Types

| Type | Description | Example |
|------|-------------|---------|
| `feat` | New feature | `feat(bridge): add async command support` |
| `fix` | Bug fix | `fix(scope): resolve path traversal issue` |
| `docs` | Documentation | `docs(api): update WindowAPI reference` |
| `chore` | Maintenance | `chore(deps): bump pyo3 to 0.28` |
| `test` | Tests | `test(bridge): add injection prevention tests` |
| `refactor` | Refactoring | `refactor(events): simplify emitter` |
| `perf` | Performance | `perf(ipc): optimize JSON serialization` |
| `ci` | CI/CD | `ci: add CodeQL security scanning` |
| `build` | Build system | `build(maturin): update wheel config` |
| `revert` | Revert commit | `revert: undo breaking change` |

### Scopes

| Scope | Description |
|-------|-------------|
| `bridge` | IPC bridge |
| `events` | Event system |
| `state` | State management |
| `window` | Window management |
| `scope` | Security scope |
| `api` | API modules |
| `cli` | CLI tools |
| `rust` | Rust core |
| `deps` | Dependencies |

### Examples

```bash
# Feature
git commit -m "feat(bridge): add binary data transfer support"

# Bug fix
git commit -m "fix(scope): prevent symlink escape in path validation"

# Breaking change
git commit -m "feat(bridge)!: change IPC protocol format

BREAKING CHANGE: The IPC protocol now uses msgspec instead of json."

# With issue reference
git commit -m "fix(events): prevent duplicate listener registration

Fixes #123"
```

---

## Merge Strategy

### Squash and Merge (Default)

All PRs are squash-merged to maintain a clean commit history:

```
main
├── feat(bridge): add async command support (#123)
├── fix(scope): resolve path traversal issue (#124)
└── chore(deps): bump pyo3 to 0.28 (#125)
```

### When to Use Other Strategies

| Strategy | When to Use |
|----------|-------------|
| **Squash** | Default for all PRs |
| **Merge** | Release branches merging back to main |
| **Rebase** | Long-running feature branches (rare) |

---

## Release Branches

### Naming

```
release/v3.1.0
release/v3.2.0
```

### Lifecycle

1. **Created** from `main` ~1 week before release
2. **Frozen** - no new features, only bug fixes
3. **Merged** back to `main` and tagged
4. **Deleted** after release

### Rules

- Only bug fixes allowed
- All fixes must be cherry-picked to `main`
- Version bumped in release branch
- Tag created from release branch

---

## Hotfix Branches

### Naming

```
hotfix/security-patch-2024-01
hotfix/critical-crash-fix
```

### Process

1. Created from `main`
2. Fix applied
3. PR reviewed and merged to `main`
4. Cherry-picked to active release branches
5. New patch release created

---

## Enforcement

### Automated Checks

- **Pre-commit hooks**: Lint, format, type check
- **CI pipeline**: All status checks must pass
- **Branch protection**: GitHub enforces rules

### Manual Checks

- **Code review**: At least 1 approval
- **Code owner review**: Required for sensitive files
- **Security review**: Required for security-sensitive changes

---

## Exceptions

### Emergency Hotfixes

For critical security vulnerabilities:

1. Security team can create hotfix branch
2. Minimal review required (1 security team member)
3. Fast-tracked through CI
4. Immediately deployed
5. Post-incident review within 48 hours

### Documentation-Only Changes

For changes to `.md` files only:

1. No CI required (optional)
2. 1 approval required
3. Can be merged without status checks

---

## Monitoring

### Metrics to Track

- PR merge time (target: <24 hours)
- CI pass rate (target: >95%)
- Review turnaround (target: <4 hours)
- Revert rate (target: <1%)

### Alerts

- CI failure rate >10% in 24 hours
- PRs pending review >48 hours
- Failed deployments

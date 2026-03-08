# Forge Production Plan

## 1. Executive Summary

Forge must evolve from a promising hybrid prototype into a production-grade application platform for building secure, native-feeling desktop apps with Python and web technologies. The target operating model is not merely "fast webview + Python IPC"; it is a complete framework with a stable runtime, enforceable security model, reliable developer experience, platform packaging, operational tooling, and long-term maintainability.

This plan defines the engineering work required to make Forge production-ready, with explicit architecture, workstreams, milestones, quality gates, ownership expectations, risks, and exit criteria.

---

## 2. Product Vision

Forge should become the best-in-class Python-native desktop framework for teams that want:

- Python as the primary backend application language.
- A Rust runtime for native OS integration and performance-critical paths.
- A stable and secure command bridge between frontend and backend.
- First-class support for HTML/CSS/JS and modern frameworks.
- Predictable packaging, distribution, updating, and diagnostics.

The end state is a framework that teams can use for internal tools, commercial desktop software, regulated environments, and long-lived products.

---

## 3. Engineering Principles

### 3.1 Product Principles
- **Correct before fast**: API trust and runtime correctness matter more than benchmark claims.
- **Secure by default**: dangerous capabilities must be opt-in and enforceable.
- **Stable contracts**: public APIs, CLI behavior, config schema, and IPC formats must be versioned.
- **Native first**: critical OS integrations belong in the Rust runtime, not in incidental Python or browser shims.
- **Operationally real**: building apps must include packaging, logging, crash reports, updates, and diagnostics.

### 3.2 Systems Principles
- **Single source of truth** for config, permissions, API manifests, and generated bindings.
- **Capability-based architecture** instead of broad global access.
- **Fail closed** on permission checks, unsupported platforms, and malformed inputs.
- **Measure everything**: performance, memory, IPC latency, startup, crash rate, install success, update success.
- **Cross-platform parity with explicit exceptions**: every capability must document support gaps by OS.

---

## 4. Definition of Production Readiness

Forge is production-ready only when all of the following are true:

1. Public APIs, examples, docs, templates, and CLI outputs are internally consistent.
2. Permissions are enforced at runtime and test-covered.
3. All core platform features are implemented in stable, typed, versioned APIs.
4. Linux, macOS, and Windows builds pass CI and generate installable artifacts.
5. Update, logging, error reporting, and diagnostics are available by default.
6. The framework supports at least one commercial-grade reference app with automated tests.
7. Performance targets are met on representative hardware, not on a single development machine.
8. Release, rollback, and security patch workflows are documented and exercised.

---

## 5. Target Architecture

## 5.1 Runtime Layers

### Layer A: Rust Native Runtime
Responsibilities:
- Window lifecycle and multi-window management.
- WebView embedding and protocol mediation.
- Native OS APIs: dialogs, clipboard, notifications, tray, menus, shortcuts, deep links, shell restrictions.
- Secure asset serving and protocol isolation.
- Binary/typed IPC transport.
- Crash boundaries, runtime telemetry hooks, logging sinks.

### Layer B: Python Host Runtime
Responsibilities:
- Command registration and dispatch.
- Background task execution and concurrency management.
- App state container and lifecycle hooks.
- Strongly typed config loading and validation.
- Python SDK for app authors.
- Optional web/server hosting mode with the same capability model.

### Layer C: Frontend Runtime
Responsibilities:
- Stable `window.__forge__` API.
- Typed invocation and event APIs.
- Safe transport abstraction for desktop and web modes.
- Dev-mode diagnostics, hot reload integration, and source-map-aware error reporting.

### Layer D: Tooling and Distribution
Responsibilities:
- Project scaffolding.
- Development orchestration.
- Build graph coordination.
- Packaging, signing, notarization, update artifact generation.
- CI/CD templates and release automation.

---

## 6. Current Gaps to Close

The present codebase demonstrates core promise but not platform completeness. The major production blockers are:

1. **Public API drift**
	- Examples, templates, and docs do not match runtime behavior.
	- The framework surface is not yet self-consistent.

2. **Permission model not enforced**
	- Permissions exist in config but are not systematically enforced at registration and invocation boundaries.

3. **Incomplete native feature set**
	- Windowing is limited.
	- Dialog, clipboard, and other APIs are partly browser-backed or inconsistent.

4. **No real production packaging pipeline**
	- Build exists, but installers, signing, notarization, and update flows are missing.

5. **Developer experience gaps**
	- No robust hot reload or full dev server integration.
	- No end-to-end debugging experience.

6. **Operational gaps**
	- No standard crash reporting, update channel, structured logging, or support bundle tooling.

7. **Weak ecosystem posture**
	- No plugin system contract.
	- No stable extension model.

---

## 7. Strategic Workstreams

## 7.1 Workstream A — Public API Stabilization

### Objective
Make the framework honest, predictable, and internally consistent.

### Deliverables
- Canonical public API spec.
- Runtime support for instance-level `app.command` and module-level decorators.
- Explicit `app.fs`, `app.system`, `app.dialog`, `app.clipboard`, `app.tray`, `app.window`, and `app.runtime` surfaces.
- Config schema and docs alignment.
- Templates and examples converted into executable integration fixtures.

### Required Tasks
- Define public API naming conventions and deprecation rules.
- Add compatibility shims only where migration value is real.
- Replace ambiguous command naming with a versioned command registry manifest.
- Generate API docs from code metadata rather than manually maintained markdown alone.

### Exit Criteria
- All shipped templates run unchanged.
- All examples are green in CI on supported platforms.
- README, CLI help, examples, and runtime outputs are aligned.

---

## 7.2 Workstream B — Security and Capability System

### Objective
Implement a real Tauri-class capability model.

### Deliverables
- Capability manifest format in `forge.toml` or split manifest files.
- Runtime enforcement at API registration, command invocation, and native call boundaries.
- Per-window and per-origin capability scoping.
- Deny-by-default mode.
- Security policy documentation and threat model.

### Required Tasks
- Define capability classes:
  - file system
  - dialog
  - clipboard
  - shell/process
  - notifications
  - deep links
  - updater
  - network
  - custom protocol
  - window management
- Add path-scoped permissions for filesystem access.
- Add command-level permission annotations.
- Add origin validation for web mode.
- Add unsafe API review tags for potentially dangerous plugins.
- Add fuzzing for IPC parsing and command argument validation.

### Exit Criteria
- Every sensitive API is permission-gated.
- Security test suite covers allow, deny, malformed input, and escalation attempts.
- Default generated apps expose only minimal capabilities.

---

## 7.3 Workstream C — Native Runtime Expansion

### Objective
Upgrade the Rust runtime from single-window proof-of-concept to full application runtime.

### Deliverables
- Multi-window manager.
- Window state control: show/hide, maximize/minimize, fullscreen, center, focus, close, always-on-top, decorations, transparency, resizable, position.
- Native menu system.
- Native tray integration.
- Native dialogs and clipboard.
- Notifications.
- Deep links and custom protocol registration.
- Download handling and navigation policy controls.

### Required Tasks
- Create runtime event bus in Rust.
- Replace ad hoc event enums with structured runtime commands.
- Add unique window identifiers and parent/child relationships.
- Design a thread-safe handle model for window operations from Python.
- Implement consistent lifecycle callbacks: `created`, `ready`, `navigated`, `close_requested`, `closed`, `crashed`.
- Add OS abstraction layer with feature flags where parity is impossible.

### Technical Factors vs Tauri — Desktop Feature Depth

| Factor | Tauri-Class Expectation | Forge Current State | Technical Requirement to Close Gap |
| :--- | :--- | :--- | :--- |
| Multi-window runtime | Stable runtime-owned window registry with labels, parent/child relationships, and lifecycle-safe cross-window messaging | Single primary window with partial runtime controls | Introduce window registry, window IDs, ownership/lifecycle model, and cross-window event routing in Rust and Python |
| Window lifecycle contract | Deterministic events for create, ready, focus, blur, navigate, close request, close, destroy, and crash | Basic window events now exist, but contract is not yet complete or formally documented | Freeze lifecycle schema, add parity tests, and define behavior for denied close, destroyed windows, and reload/recreate flows |
| Menu model | Native application and window menus with dynamic updates, accelerators, enable/disable state, check/radio items, and event callbacks | Missing | Add runtime-owned menu tree, menu item IDs, accelerator mapping, and platform adapters for macOS app menu, Windows menu bar, and Linux variants |
| Tray/system status integration | Runtime-owned tray with image lifecycle, menus, click semantics, and platform parity guarantees | Python-side tray exists, but not runtime-owned | Move tray into Rust runtime ownership, define event model, and support icon updates, menu updates, and background app behavior |
| Notifications | Native OS notifications with capability gating, payload schema, click callbacks, and per-OS fallback rules | Missing | Build notification API in Rust, define click/action callback path to Python/JS, and document support matrix and permission behavior |
| Deep links and protocols | Registration, invocation routing, single-instance handoff, and cold-start/warm-start protocol delivery | Missing | Add app protocol config, runtime dispatch on startup and while running, OS-specific registration pipeline, and security validation of incoming URLs |
| Shell/process APIs | Restricted child process and shell APIs with capability controls and argument validation | Missing | Define safe process/shell contract, default deny posture, allowlisted execution, stdout/stderr streaming, and cancellation controls |
| File associations | OS-native file-open integration with routing into running app instance | Missing | Add file association manifest model, startup handoff hooks, and testable open-file lifecycle semantics |
| Download/navigation policy | Download interception, navigation allow/deny policy, external URL opening rules, and origin enforcement | Partial | Add navigation policy callbacks, download events, external opener policies, and capability-aware shell handoff |
| Platform polish | Per-OS conventions for macOS activation, app menu behavior, Linux tray variance, Windows taskbar behavior, and focus semantics | Limited | Build explicit per-platform behavior matrix and acceptance tests instead of one generic runtime path |

### Engineering Rule
Desktop APIs must be runtime-owned first and browser-backed only as explicit fallback behavior. Anything that depends on frontend shims cannot be considered Tauri-class parity.

### Required Platform Contracts

#### Runtime Ownership Contract
- Rust owns all long-lived desktop primitives:
  - windows
  - menus
  - tray handles
  - notifications
  - protocol dispatch
  - navigation/download policies
- Python exposes orchestration APIs and business logic hooks, but not raw unsafe runtime handles.
- JS receives stable high-level APIs only; it must not be required to emulate native desktop behavior.

#### Public API Contract
- `app.window` becomes a window-manager surface, not just a single-window helper.
- Add explicit APIs for:
  - `create_window(...)`
  - `get_window(label)`
  - `list_windows()`
  - `broadcast(event, payload)`
- Add runtime-owned surfaces for:
  - `app.menu`
  - `app.notification`
  - `app.protocol`
  - `app.shell` or `app.process`
- Every desktop API must declare:
  - capability requirement
  - OS support matrix
  - failure model
  - sync vs async behavior

#### State and Event Contract
- All runtime-managed resources need stable IDs.
- Window, tray, menu, protocol, and notification callbacks must use a single event envelope format.
- Event ordering guarantees must be documented for:
  - create → ready
  - close_requested → closed
  - navigate_requested → navigated / navigation_blocked
  - protocol_received → routed
- App startup must define cold-start vs warm-start semantics for protocol and file-open delivery.

#### Desktop Testing Contract
- Add reference examples for:
  - multi-window app
  - tray + menu app
  - notification app
  - protocol/deep-link app
- Add GUI smoke tests for Linux, Windows, and macOS.
- Add parity snapshots for window lifecycle and menu event behavior.
- No desktop feature should be considered complete until one example and one CI smoke path validate it.

#### Platform Acceptance Matrix
- Linux:
  - GTK/WebKit runtime behavior
  - tray fallback behavior
  - window manager variance handling
- macOS:
  - app activation model
  - application menu ownership
  - dock/tray/protocol behavior
- Windows:
  - taskbar/focus behavior
  - notification path
  - protocol/file association behavior

### Exit Criteria
- Multi-window apps are supported and documented.
- Core native features behave consistently across Linux, macOS, and Windows.
- Python app authors do not need to reach into internal runtime handles.

---

## 7.4 Workstream D — IPC Redesign

### Objective
Build a typed, performant, observable bridge.

### Deliverables
- IPC protocol v1 spec.
- Typed request/response envelopes.
- Structured error model.
- Streaming and cancellation support.
- Optional binary transport for large payloads.
- Metrics and tracing hooks.

### Required Tasks
- Define command metadata: name, version, args schema, return schema, capability requirements.
- Support request correlation IDs and cancellation tokens.
- Separate control plane from payload plane.
- Introduce backpressure strategy for high-frequency events.
- Benchmark JSON vs MessagePack vs shared-memory transports.
- Add compatibility layer for legacy JSON invoke during migration.

### Exit Criteria
- IPC protocol is versioned and documented.
- Large payloads no longer rely on naive JSON serialization.
- Errors are stable, sanitized, and machine-readable.

---

## 7.5 Workstream E — Developer Experience Platform

### Objective
Make local development fast, deterministic, and debuggable.

### Deliverables
- Real hot reload pipeline.
- Frontend dev server integration.
- Python backend reload strategy.
- Unified logs for Rust, Python, and frontend.
- Devtools integration and debug commands.

### Required Tasks
- Support Vite and plain frontend workflows as first-class modes.
- Implement frontend file watching and runtime reload signaling.
- Decide backend reload model: interpreter restart vs targeted module reload.
- Add source maps and error overlays.
- Add CLI doctor command for environment validation.
- Add deterministic development ports and conflict resolution.

### Technical Factors vs Tauri — DX and Tooling

| Factor | Tauri-Class Expectation | Forge Current State | Technical Requirement to Close Gap |
| :--- | :--- | :--- | :--- |
| Frontend dev-server orchestration | CLI detects and manages Vite/webpack/etc. dev servers, proxies correctly, and surfaces failures clearly | Partial and template-dependent | Add explicit dev-server contract in config, health checks, startup timeout policy, log forwarding, and graceful shutdown handling |
| Hot reload semantics | Reliable frontend reload/HMR with clear fallback to full page reload when needed | Config exists but robust behavior is absent | Implement file watching, change classification, websocket/runtime reload channel, and deterministic reload policy |
| Backend reload model | Predictable restart or module reload with state/lifecycle rules and error recovery | Missing | Choose one supported model first, codify lifecycle teardown/startup, and preserve runtime diagnostics across reload boundaries |
| Unified development logs | One surface for Rust, Python, frontend, IPC, and dev-server output with filtering and timestamps | Partial supportability primitives only | Add CLI log multiplexer, structured log channels, colored domains, and persisted dev session traces |
| Error overlays and source maps | Browser/frontend build errors surface immediately with mapped stack traces into source | Missing | Integrate source-map aware frontend errors, Python trace relay, and Rust runtime dev diagnostics into one developer-facing surface |
| CLI determinism | `forge dev` and `forge build` behave identically across templates and OSes with stable defaults | Not yet production-grade | Harden config discovery, env validation, dependency checks, command contracts, and machine-readable CLI outputs |
| Environment doctor | First-class validation for Python, Rust, WebView prerequisites, build tools, signing tools, and package managers | Missing | Add `forge doctor` with actionable checks, exit codes, remediation guidance, and CI-friendly output |
| Template contract | Official templates are versioned, tested, and validated against the current runtime and CLI | Partial | Turn examples/templates into CI fixtures, add template schema versioning, and detect stale generated projects |
| Build orchestration | CLI manages frontend build, backend build, asset bundling, and package metadata validation in one graph | Partial | Build a deterministic build graph with staged artifacts, cache strategy, and reproducible failure reporting |
| Debug surfaces | Developers can inspect IPC traffic, capabilities, runtime state, window state, and updater state during local dev | Partial | Expand runtime inspector output, CLI debug commands, trace toggles, and live event inspection |
| Cross-platform smoke tests | Dev loop is exercised in CI on Linux, Windows, and macOS, including GUI/runtime smoke paths | Limited | Add GUI smoke harnesses, template launch tests, packaging smoke tests, and matrix gating for release branches |

### Engineering Rule
`forge dev` must become the canonical loop for every official template. If developers still need to hand-wire frontend dev servers or manually infer reload behavior, Forge remains materially behind Tauri.

### Required Tooling Contracts

#### Canonical Developer Loop Contract
- `forge dev` must own the full local loop:
  - config discovery
  - frontend dev server launch or static mode selection
  - runtime startup
  - reload signaling
  - log aggregation
  - failure reporting
- Templates must declare their dev workflow in machine-readable config rather than implicit shell scripts.

#### Reload Contract
- Frontend changes must be classified into:
  - HMR-safe
  - full-page reload required
  - runtime restart required
- Backend changes must use one supported policy first:
  - full interpreter restart
- Reload behavior must preserve:
  - deterministic port reuse
  - log continuity
  - diagnostics snapshots on failure
- Reload failures must never silently stall the loop.

#### CLI Contract
- All primary commands must support:
  - structured human output
  - machine-readable output mode
  - stable exit codes
  - actionable error messages
- `forge create`, `forge dev`, `forge build`, and `forge doctor` must be versioned contracts, not best-effort scripts.

#### Dev Observability Contract
- Dev mode must expose:
  - Rust logs
  - Python logs
  - frontend logs
  - IPC traffic inspection
  - capability denials
  - runtime/window state snapshots
  - updater state in dev mode when configured
- Error overlays must correlate source locations across frontend and Python boundaries when possible.

#### Template and Fixture Contract
- Every official template must be treated as a tested product surface.
- Templates must have:
  - schema version
  - supported Forge version range
  - deterministic dependency bootstrap
  - CI validation on create + dev + build flows
- Examples are not documentation ornaments; they are compatibility fixtures.

#### Tooling Acceptance Metrics
- Fresh scaffold to first successful `forge dev`: under 2 minutes on a baseline machine.
- Config or environment failure must be diagnosed by `forge doctor` without manual log inspection.
- Frontend edit to visible refresh in dev mode must be reliable and documented.
- A failed dev-server start must surface root cause within one CLI session without requiring external tooling.

### Exit Criteria
- `forge dev` is the recommended developer loop for all official templates.
- Editing frontend or backend code produces a documented and reliable refresh behavior.
- Developers can inspect IPC traffic and runtime logs in dev mode.

---

## 7.6 Workstream F — Packaging, Signing, and Distribution

### Objective
Ship installable, trustworthy applications.

### Deliverables
- Installer generation for Windows, macOS, and Linux.
- Code signing pipeline.
- Notarization support for macOS.
- Release metadata and update feed generation.
- Asset bundling rules.

### Required Tasks
- Define app bundle layout per OS.
- Generate icons in required platform formats.
- Add package metadata validation.
- Build Windows MSI/EXE pipeline.
- Build macOS `.app` and DMG pipeline.
- Build Linux AppImage/DEB/RPM pipeline.
- Add reproducible build documentation.
- Create release manifest schema.

### Exit Criteria
- A sample app can be built and installed on all supported OSes.
- Signing/notarization is documented and CI-compatible.
- Release artifacts are consistent and update-capable.

---

## 7.7 Workstream G — Update, Telemetry, and Diagnostics

### Objective
Provide production operations capabilities.

### Deliverables
- Built-in updater service.
- Structured logging framework.
- Crash capture and support bundles.
- Optional telemetry hooks with privacy controls.

### Required Tasks
- Define stable log schema across Rust/Python/frontend.
- Capture fatal runtime failures and app-level exceptions.
- Implement support bundle export: logs, system info, config snapshot, redacted diagnostics.
- Add update channel model: stable, beta, nightly.
- Add rollback-safe updater flow.
- Document privacy posture and opt-in telemetry behavior.

### Exit Criteria
- Production apps can update safely.
- Support teams can collect actionable diagnostics.
- Logging is structured and composable with SIEM/monitoring stacks.

---

## 7.8 Workstream H — Plugin and Ecosystem Architecture

### Objective
Allow extension without degrading platform safety.

### Deliverables
- Plugin API contract.
- Plugin lifecycle model.
- Capability declarations for plugins.
- Official core plugins.

### Required Tasks
- Define plugin types:
  - Rust-native plugins
  - Python plugins
  - optional frontend companion plugins
- Add plugin manifest and compatibility constraints.
- Define plugin sandbox and trust boundaries.
- Publish at least five reference plugins:
  - filesystem
  - store
  - notification
  - updater
  - shell/process

### Exit Criteria
- Plugins can be added without patching core.
- Plugins participate in capability validation and version checks.

---

## 8. Milestone Plan

## Milestone 0 — Stabilization Foundation

### Goal
Make the current repository self-consistent.

### Scope
- Fix API drift.
- Make examples/templates executable.
- Align docs and runtime.
- Enforce permissions in current APIs.
- Add missing `ForgeApp` public surfaces.

### Success Criteria
- All examples pass in CI.
- No stale documentation for current APIs.
- Public API mismatch bugs reduced to zero.

---

## Milestone 1 — Runtime and Capability Core

### Goal
Establish secure, versioned foundations.

### Scope
- Capability enforcement.
- IPC protocol v1.
- Native window manager v1.
- Native dialog and clipboard.

### Success Criteria
- Core runtime contracts frozen for beta.
- Capability tests pass across all OSes.

---

## Milestone 2 — Developer Platform Beta

### Goal
Make Forge pleasant and predictable for daily development.

### Scope
- Real `forge dev`.
- Debug tooling.
- Improved CLI ergonomics.
- Reference app suite.

### Success Criteria
- Official templates support live development without ad hoc manual setup.
- Developers can inspect logs and IPC in one place.

---

## Milestone 3 — Distribution and Operations

### Goal
Make shipping feasible.

### Scope
- Installers.
- Signing.
- Notarization.
- Updater.
- Support bundles.

### Success Criteria
- A sample app is installable and updatable on all target OSes.

---

## Milestone 4 — Ecosystem and 1.0 Release

### Goal
Lock a stable platform and ship Forge 1.0 production release.

### Scope
- Plugin system.
- Long-term support policy.
- Migration guides.
- Performance certification.

### Success Criteria
- API stability policy is published.
- Core plugins available.
- 1.0 release checklist complete.

---

## 9. Quality Engineering Plan

### 9.1 Test Pyramid
- Unit tests for Python, Rust, and JS runtime layers.
- Contract tests for IPC schemas and capability enforcement.
- Integration tests for templates and examples.
- End-to-end GUI smoke tests per OS.
- Performance regression tests.
- Security fuzzing and malicious input suites.

### 9.2 Mandatory CI Matrix
- Linux x86_64
- Windows x86_64
- macOS arm64
- macOS x86_64 if supported

### 9.3 Mandatory Validation Gates
- Formatting and linting.
- Static type checks.
- Rust clippy and docs build.
- Python type checks.
- Example app compile/run smoke tests.
- Packaging smoke tests.
- Security regression suite.

### 9.4 Release Blocking Rules
- Any failing example blocks release.
- Any permission bypass is a release blocker.
- Any installer regression on a supported OS blocks release.

## 9.5 Release Hardening Execution Plan (March 2026)

### Objective
Close the remaining gaps between "buildable" and "releasable" artifacts.

### Phase 1 — Eliminate misleading artifacts
1. Remove all placeholder installer fallbacks.
2. Fail packaging immediately when required external tools are missing.
3. Surface missing-tool failures in machine-readable build validation.

### Phase 2 — Real platform matrix
1. Add a Linux/macOS/Windows release matrix workflow that:
  - builds the native extension
  - runs core tests
  - packages a reference app with real platform toolchains
  - runs installer smoke tests
2. Treat matrix failures as release blockers.

### Phase 3 — Installer smoke validation
1. Linux:
  - launch produced AppImage under headless CI
  - install and launch Flatpak bundle
2. macOS:
  - mount produced DMG
  - locate bundled `.app`
  - launch the contained executable
3. Windows:
  - install MSI silently into a controlled directory
  - install NSIS package silently into a controlled directory
  - launch the installed executable

### Phase 4 — Signing and notarization validation
1. Linux:
  - generate an ephemeral CI GPG key
  - sign and verify release artifacts end to end
2. Windows:
  - generate or load a code-signing certificate in CI
  - sign and verify MSI/NSIS artifacts with `signtool`
3. macOS:
  - import a signing identity into a temporary keychain
  - sign app and DMG artifacts
  - notarize through `notarytool`
  - fail release validation if notarization secrets are absent for protected release branches

### Exit Criteria
- No requested installer format emits a fake artifact.
- The reference app is packaged and smoke-tested on Linux, macOS, and Windows.
- Linux signing, Windows signing, and macOS signing/notarization each have an executable CI path.
- Release workflows produce auditable artifacts and machine-readable results.

---

## 10. Performance Targets

These are targets, not marketing claims. They must be measured under repeatable conditions.

| Metric | Alpha Target | 1.0 Target | Notes |
| :--- | :--- | :--- | :--- |
| Cold startup | <700ms | <350ms | Representative mid-range hardware |
| Warm startup | <400ms | <200ms | App shell ready |
| Idle RAM | <90MB | <55MB | Simple app |
| IPC roundtrip | <5ms | <1.5ms | Small payload |
| Large payload transfer | <150ms | <40ms | 5MB structured payload |
| Build dev cycle | <4s | <2s | Frontend change to visible refresh |
| Crash-free sessions | 99.0% | 99.7% | On reference apps |

### Benchmark Rules
- Publish benchmark harnesses.
- Store results in CI artifacts.
- Compare against previous releases.
- Reject cherry-picked machine-local results.

---

## 11. Security Program

### 11.1 Threat Model Areas
- IPC injection and deserialization attacks.
- Filesystem traversal and symlink escalation.
- Command abuse through exposed frontend surfaces.
- Custom protocol and local asset abuse.
- Remote content loading.
- Plugin privilege escalation.
- Unsafe updater or signing pipeline.

### 11.2 Required Security Controls
- Capability enforcement.
- Sanitized and structured errors.
- CSP recommendations for frontend assets.
- Secure defaults for local-only content.
- Origin validation for web mode.
- Release signing verification.
- Security advisory and patch workflow.

### 11.3 Required Security Assets
- Threat model document.
- Secure coding standard.
- Dependency review policy.
- Vulnerability disclosure process.
- Patch severity rubric.

---

## 12. Documentation Program

Documentation must be treated as part of the product, not an afterthought.

### Required Documentation Sets
- Quick start.
- Architecture guide.
- Runtime API reference.
- Capability and permissions guide.
- Packaging and release guide.
- Security guide.
- Debugging guide.
- Migration guide.
- Platform support matrix.

### Documentation Rules
- Examples are tested in CI.
- API reference should be generated where possible.
- Deprecated behavior must be tagged with version timelines.

---

## 13. Governance and Ownership

### Required Ownership Areas
- Runtime core
- Python SDK
- CLI/tooling
- Security
- Release engineering
- Documentation
- Developer experience

### Engineering Practices
- RFC process for major API changes.
- Versioned milestones and release notes.
- Backward-compatibility review on public API changes.
- Security review for any new capability or plugin hook.

---

## 14. Risks and Mitigations

| Risk | Impact | Mitigation |
| :--- | :--- | :--- |
| API drift continues | Loss of trust | Lock public API spec and CI-test examples |
| Cross-platform divergence | High maintenance cost | OS parity matrix and feature flags |
| Python/Rust boundary complexity | Crashes and deadlocks | Strong ownership model, tracing, concurrency tests |
| Packaging complexity | Delayed adoption | Dedicated release engineering workstream |
| Security model remains partial | Unsafe apps | Capability system as release gate |
| Docs remain stale | Developer confusion | Docs generated from tested sources |
| Plugin model weakens security | Platform erosion | Signed manifests, capabilities, review process |

---

## 15. Immediate Engineering Priorities

### Priority 0 — Repository Integrity
1. Fix `ForgeApp` public API consistency.
2. Expose actual built-in API objects on the app instance.
3. Enforce permissions during API setup and invocation.
4. Update templates/examples to match the real runtime.
5. Rewrite stale documentation to reflect implemented behavior only.

### Priority 1 — Runtime Contract
1. Define IPC protocol v1.
2. Add typed command metadata.
3. Build native window manager abstraction.
4. Replace JS-backed clipboard/dialog shortcuts with native implementations.

### Priority 2 — Developer Loop
1. Implement hot reload architecture.
2. Add unified logging and inspection.
3. Build CI-tested template workflows.

### Priority 3 — Shipping Platform
1. Implement packaging pipeline per OS.
2. Add signing/notarization support.
3. Add updater and diagnostics.

---

## 16. Release Recommendation

Forge should not position itself as a full Tauri-equivalent platform until Milestones 0 through 3 are complete. Before that point, it should be described as:

> "An experimental but fast Rust + Python desktop framework under active development."

After Milestone 3, Forge can credibly market itself as:

> "A production-ready Python desktop framework with a Rust-native runtime, secure capability system, and cross-platform packaging."

---

## 17. Final Directive

The project should optimize for **trustworthiness, platform completeness, and operational maturity** over short-term feature count. If there is a conflict between adding new APIs and hardening the runtime, the runtime wins.

The path to a production-grade framework is clear:

1. Make the current system internally consistent.
2. Enforce security and capability boundaries.
3. Expand the native runtime to cover real application needs.
4. Build the developer and release platform around it.
5. Lock contracts, measure quality, and ship with discipline.

---

## 18. Execution Backlog by Sprint

### Sprint Model
- Sprint length: 2 weeks
- Cadence: engineering demo at end of sprint, release candidate cut every 2 sprints
- Branching: short-lived feature branches, trunk-based integration for runtime contracts
- Definition of done for each item:
  - code merged
  - tests added or updated
  - docs updated
  - example or fixture validated where relevant
  - CI green on required matrix subset

### Sprint 0 — Baseline, Inventory, and Contract Freeze

#### Objective
Establish control over the current codebase and freeze the first set of public contracts.

#### Backlog
1. Create a public API inventory for Python, Rust, JS, CLI, and config.
2. Create a current-vs-documented behavior inventory.
3. Mark all unstable APIs and undocumented behavior.
4. Define canonical naming for commands and app surfaces.
5. Add release labels: `experimental`, `beta-candidate`, `internal-only`.
6. Create architecture decision records for:
  - command decorator model
  - app surface model
  - permissions enforcement boundary
  - IPC protocol versioning

#### Deliverables
- API inventory document
- config schema inventory
- architecture decision record set
- issue tracker seeded with normalized work items

#### Exit Criteria
- No ambiguity remains about what the current framework exposes.
- Every documented feature is tagged as implemented, partial, or planned.

---

### Sprint 1 — Repository Integrity and API Truthfulness

#### Objective
Make the repository self-consistent and restore user trust.

#### Backlog
1. Implement instance-level `app.command` with the same semantics as documented templates.
2. Expose built-in API objects on `ForgeApp`:
  - `app.fs`
  - `app.system`
  - `app.dialog`
  - `app.clipboard`
  - `app.tray`
3. Align method naming between runtime and docs, or add temporary compatibility shims.
4. Update templates so all generated apps run without edits.
5. Fix all official examples to match the real runtime.
6. Add integration tests that instantiate each template and example.

#### Deliverables
- consistent `ForgeApp` surface
- passing templates
- passing example smoke tests
- updated README and SECURITY docs

#### Exit Criteria
- A new user can run every official template and example without patching framework internals.

---

### Sprint 2 — Permission Enforcement and Security Baseline

#### Objective
Turn permissions from config metadata into runtime policy.

#### Backlog
1. Enforce permission checks during API registration in `ForgeApp`.
2. Enforce permission checks during command invocation in the bridge.
3. Introduce capability descriptors for built-in APIs.
4. Add command-level annotations for required capabilities.
5. Add deny-by-default mode for generated apps.
6. Expand tests for denied access, escalation attempts, and bad capability manifests.
7. Remove stale security guidance that references non-existent behaviors.

#### Deliverables
- capability enforcement layer v0
- security regression suite expansion
- updated `forge.toml` permission schema docs

#### Exit Criteria
- Sensitive APIs cannot be accessed when not explicitly enabled.

---

### Sprint 3 — IPC Protocol v1 Design and Migration Layer

#### Objective
Define and begin implementing a typed, versioned IPC contract.

#### Backlog
1. Write IPC protocol v1 spec.
2. Define request envelope, response envelope, and error envelope.
3. Add protocol version field to JS and Python runtime paths.
4. Add structured error codes.
5. Add command metadata registry with:
  - name
  - version
  - args schema
  - return schema
  - capability requirements
6. Add compatibility mode for legacy JSON invoke.
7. Add trace hooks for IPC roundtrip timings.

#### Deliverables
- IPC v1 specification
- migration-compatible invoke path
- protocol conformance tests

#### Exit Criteria
- IPC behavior is versioned and testable, not implicit.

---

### Sprint 4 — Native Runtime Foundation Expansion

#### Objective
Lift the Rust core from single-window shell to application runtime foundation.

#### Backlog
1. Introduce structured runtime command/event bus in Rust.
2. Add window identifiers and registry.
3. Implement basic window operations:
  - show/hide
  - close
  - maximize/minimize
  - fullscreen
  - focus
  - center
  - navigate
  - reload
4. Expose a stable Python-side window API.
5. Add window lifecycle callbacks.
6. Implement fullscreen config pass-through.
7. Define runtime-owned menu architecture and identifier model.
8. Define tray ownership migration from Python-side integration to Rust runtime.
9. Define deep-link, protocol, and file-open routing contract.
10. Define navigation/download policy callback contract.

#### Deliverables
- window manager v1
- Python window control surface
- runtime lifecycle event model
- desktop runtime contract document
- platform parity matrix v1

#### Exit Criteria
- Multi-window support exists in controlled form and can be used by a reference example.
- Menu, tray, protocol, and notification work items have frozen public contracts before implementation fan-out.

---

### Sprint 5 — Native Platform APIs

#### Objective
Replace browser-backed placeholders with runtime-owned platform integrations.

#### Backlog
1. Implement native clipboard in Rust runtime.
2. Implement native dialogs in Rust runtime.
3. Integrate tray support into runtime ownership model.
4. Add notification API design and first implementation.
5. Define support matrix per OS.
6. Add parity tests and fallback behaviors where OS support differs.
7. Implement menu API v1 with accelerators and callbacks.
8. Implement protocol/deep-link routing v1.
9. Define safe shell/process API contract and capability model.
10. Add reference examples for tray, notifications, and protocol handling.

#### Deliverables
- native clipboard API
- native dialog API
- tray ownership strategy
- notification API v1
- menu API v1
- deep-link/protocol API v1
- desktop examples as fixtures

#### Exit Criteria
- Core OS APIs no longer depend on browser shims for default desktop behavior.
- At least one reference app proves tray/menu/notification/protocol flows through the native runtime.

---

### Sprint 6 — Developer Experience and Hot Reload

#### Objective
Create a real development loop.

#### Backlog
1. Implement frontend file watcher and reload signaling.
2. Add frontend dev server integration for Vite-based templates.
3. Define backend reload strategy and implement first version.
4. Add unified logs pane support through CLI and runtime channels.
5. Add debug/devtools toggles.
6. Add `forge doctor` for environment diagnostics.
7. Add machine-readable CLI output and stable exit codes.
8. Add template schema/version validation.
9. Add error overlays and source-map aware diagnostics.
10. Add CI smoke tests for create/dev/build on official templates.

#### Deliverables
- `forge dev` v1
- hot reload architecture
- developer diagnostics toolchain
- CLI contract v1
- tested template workflow matrix

#### Exit Criteria
- Frontend and backend changes have a predictable development workflow documented and tested.
- `forge dev` is the default documented loop for official templates without manual shell wiring.

---

### Sprint 7 — Packaging and Artifact Pipeline

#### Objective
Make app outputs shippable rather than merely buildable.

#### Backlog
1. Define bundle layout for Windows, macOS, and Linux.
2. Add metadata validation for packaging.
3. Generate icon sets and bundle resources.
4. Implement Linux package targets first:
  - AppImage
  - DEB
  - RPM
5. Design Windows and macOS packaging workflow.
6. Add packaging smoke tests in CI.

#### Deliverables
- packaging specification
- Linux artifact pipeline v1
- metadata validation rules

#### Exit Criteria
- A reference app produces installable Linux artifacts in CI.

---

### Sprint 8 — Signing, Notarization, and Updater Architecture

#### Objective
Build trustable delivery mechanisms.

#### Backlog
1. Define release manifest schema.
2. Define update channel model.
3. Implement updater metadata generator.
4. Add signing workflow documentation and CI hooks.
5. Implement macOS notarization support plan.
6. Implement Windows signing integration plan.
7. Add artifact integrity verification.

#### Deliverables
- updater architecture
- release manifest schema
- signing/notarization workflow docs

#### Exit Criteria
- Release artifacts can be verified and prepared for update distribution.

---

### Sprint 9 — Observability, Crash Handling, and Supportability

#### Objective
Make production support feasible.

#### Backlog
1. Define structured log schema across Rust, Python, and frontend.
2. Implement crash capture boundaries.
3. Add support bundle generator.
4. Add runtime diagnostics command.
5. Add optional telemetry interfaces with opt-in policy.
6. Add privacy and data retention documentation.

#### Deliverables
- structured logging v1
- support bundle tooling
- crash handling baseline

#### Exit Criteria
- A production incident can be investigated with framework-generated diagnostics.

---

### Sprint 10 — Plugin Architecture and 1.0 Readiness Review

#### Objective
Prepare Forge for ecosystem growth and release hardening.

#### Backlog
1. Define plugin manifest and compatibility contract.
2. Define plugin lifecycle hooks.
3. Add capability validation for plugins.
4. Build one Rust plugin reference implementation.
5. Build one Python plugin reference implementation.
6. Run 1.0 readiness review across all workstreams.
7. Produce release checklist and rollback procedure.

#### Deliverables
- plugin API draft
- 1.0 readiness report
- release checklist

#### Exit Criteria
- Core platform contracts are stable enough to support external extension.

---

## 19. Gap Matrix — Current Codebase vs Production Plan

### Status Legend
- **Implemented**: feature exists and is materially usable
- **Partial**: feature exists but is incomplete, inconsistent, or not production-grade
- **Missing**: feature is absent or only documented

| Plan Area | Current Status | Evidence in Codebase | Gap | Priority |
| :--- | :--- | :--- | :--- | :--- |
| Public API stabilization | Partial | `ForgeApp` exists, but templates/examples expect `@app.command` and app-owned APIs that are not exposed consistently | Public surface is not trustworthy | P0 |
| App command decorator | Partial | Module-level decorator exists, but instance-level behavior used by templates/examples is not reliably implemented | Docs and runtime mismatch | P0 |
| Built-in API object exposure | Partial | APIs are registered in setup flow, but examples expect direct `app.fs`, `app.dialog`, `app.clipboard`, `app.tray` access | App surface contract incomplete | P0 |
| Config schema quality | Implemented | Strong typed config classes and validation exist | Needs schema versioning and stronger production docs | P1 |
| Permission model | Partial | Permission config exists, but enforcement is not systemic | Security posture is not fail-closed | P0 |
| Filesystem security | Implemented | Path resolution, traversal prevention, size limits, symlink checks exist | Needs capability integration and scoped policy model | P1 |
| Bridge validation | Implemented | Command validation, request-size limits, sanitized errors exist | Needs typed IPC, versioning, metrics, cancellation | P1 |
| IPC protocol versioning | Implemented | Protocol version, supported versions, structured replies, and registry metadata now exist in the bridge | Needs published spec and schema-level compatibility guarantees | P0 |
| Binary/large payload strategy | Missing | Roadmap mentions binary IPC, runtime uses JSON | Performance and scalability risk | P2 |
| JS runtime API | Partial | `window.__forge__` exists with invoke/event/websocket pathways | Needs typed contracts, diagnostics, compatibility handling | P1 |
| Web mode runtime | Partial | JS and CLI imply web mode, but end-to-end server/runtime path is not complete | Advertised mode is not productized | P0 |
| Native window runtime | Partial | Rust runtime now supports navigation, devtools, structured window events, and broader control surface | Still lacks multi-window architecture, menus, protocols, and platform polish | P0 |
| Multi-window management | Missing | No mature runtime-level window registry or public surface | Not usable for complex desktop software | P1 |
| Window lifecycle events | Partial | Basic ready and close-related behavior exists | Needs stable and complete lifecycle model | P1 |
| Window controls | Partial | Title, size, position, visibility, focus, minimize/maximize, fullscreen, navigation, and devtools controls exist | Missing persistence, multi-window semantics, and OS-specific polish guarantees | P1 |
| Fullscreen support | Partial | Config includes fullscreen, runtime pass-through is incomplete | Config/runtime drift | P1 |
| Native dialogs | Partial | Dialog API is descriptor-based and frontend-dependent | Must be runtime-native by default | P1 |
| Native clipboard | Partial | Clipboard API is frontend/browser-dependent | Must be runtime-native by default | P1 |
| Native tray | Partial | Tray exists through Python-side `pystray` integration | Needs runtime ownership and parity model | P2 |
| Native menus | Missing | No production menu system | Major desktop app gap | P2 |
| Notifications | Missing | No first-class runtime-owned notification API | Missing core desktop capability | P2 |
| Deep links / custom app protocols | Missing | No end-user app protocol registration model | Missing app platform feature | P3 |
| Developer hot reload | Missing | Config fields exist, but no robust implementation | DX gap | P0 |
| Frontend dev server integration | Partial | Templates imply frontend workflows, CLI/dev pipeline is incomplete | Not first-class | P1 |
| Unified runtime logs | Missing | Logging exists in pieces, not as one developer/operator surface | Weak supportability | P1 |
| CLI scaffolding | Implemented | Project create/dev/build/info commands exist | Needs stronger correctness and environment validation | P1 |
| CLI doctor and diagnostics | Missing | No environment doctor or supportability command | Onboarding/support gap | P2 |
| Build pipeline | Partial | Basic build path exists using maturin or nuitka | Not a full packaging or shipping pipeline | P0 |
| Linux packaging | Missing | Build artifacts exist, installers/packages do not | Production distribution gap | P1 |
| Windows packaging | Missing | No installer or signing pipeline | Production distribution gap | P1 |
| macOS packaging / notarization | Missing | No `.app`/DMG/notarization pipeline | Production distribution gap | P1 |
| Updater | Partial | Signed manifest verification, download, apply, rollback-safe backup flow, and support bundle exposure now exist | Missing installer handoff, restart coordination, and packaged release integration | P1 |
| Structured logging | Partial | Runtime log buffer and support bundle logging exist | Missing cross-layer schema unification and operator-grade log sinks | P1 |
| Crash handling | Partial | Crash capture, last-crash reporting, diagnostics, and support bundles exist | Missing native crash boundaries, symbolication strategy, and production incident workflow | P1 |
| Telemetry policy | Missing | No governed opt-in telemetry model | Operations maturity gap | P3 |
| Documentation integrity | Partial | Docs exist, but some are stale and inconsistent with runtime | Trust gap | P0 |
| Tested examples | Partial | Examples exist, but they are not yet a guaranteed green contract suite | Must become CI-backed fixtures | P0 |
| Cross-platform CI maturity | Partial | Tests exist, but full matrix, packaging, and GUI smoke coverage are not established | Release confidence gap | P1 |
| Plugin system | Missing | Planned only | Ecosystem gap | P3 |
| Governance / RFC discipline | Missing | Plan now exists, but formal change governance is not visible in repo | Long-term stability risk | P3 |

---

## 20. Recommended Immediate Sequencing

If execution capacity is constrained, work should be sequenced as follows:

### Sequence A — Trust and Safety First
1. Sprint 0
2. Sprint 1
3. Sprint 2
4. Sprint 3

### Sequence B — Runtime and DX Core
5. Sprint 4
6. Sprint 5
7. Sprint 6

### Sequence C — Shipping and Operations
8. Sprint 7
9. Sprint 8
10. Sprint 9

### Sequence D — Ecosystem and 1.0 Lockdown
11. Sprint 10

### Non-Negotiable Rule
No new marketing claims should be added before Sequence A is complete.

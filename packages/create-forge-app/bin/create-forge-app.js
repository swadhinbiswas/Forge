#!/usr/bin/env node
import { spawnSync } from "node:child_process";

function pythonLaunchers() {
  if (process.platform === "win32") {
    return [
      { command: "py", args: ["-3"] },
      { command: "python", args: [] },
    ];
  }
  return [
    { command: process.env.FORGE_PYTHON || "python3", args: [] },
    { command: "python", args: [] },
  ];
}

function findLauncher() {
  for (const launcher of pythonLaunchers()) {
    const probe = spawnSync(launcher.command, [...launcher.args, "-m", "forge_cli.main", "--help"], {
      stdio: "ignore",
    });
    if (probe.status === 0) {
      return launcher;
    }
  }
  for (const launcher of pythonLaunchers()) {
    const install = spawnSync(
      launcher.command,
      [...launcher.args, "-m", "pip", "install", "forge-framework"],
      { stdio: "inherit" },
    );
    if (install.status === 0) {
      return launcher;
    }
  }
  return null;
}

const launcher = findLauncher();
if (!launcher) {
  console.error("Unable to bootstrap Forge. Install Python 3.14+ and run: python -m pip install forge-framework");
  process.exit(1);
}

const argv = process.argv.slice(2);
process.exit(
  spawnSync(launcher.command, [...launcher.args, "-m", "forge_cli.main", "create", ...argv], {
    stdio: "inherit",
  }).status ?? 1,
);

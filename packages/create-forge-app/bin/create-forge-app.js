#!/usr/bin/env node
import { spawnSync } from "node:child_process";
import os from "node:os";
import path from "node:path";

function pythonLaunchers() {
  const launchers = [];
  const seen = new Set();

  function addLauncher(command, args = []) {
    const key = `${command}::${args.join(" ")}`;
    if (!seen.has(key)) {
      seen.add(key);
      launchers.push({ command, args });
    }
  }

  const explicit = process.env.FORGE_PYTHON;
  if (explicit) {
    addLauncher(explicit, []);
  }

  const virtualEnv = process.env.VIRTUAL_ENV;
  if (virtualEnv) {
    if (process.platform === "win32") {
      addLauncher(path.join(virtualEnv, "Scripts", "python.exe"), []);
    } else {
      addLauncher(path.join(virtualEnv, "bin", "python"), []);
    }
  }

  const localVenv = process.platform === "win32"
    ? path.join(process.cwd(), ".venv", "Scripts", "python.exe")
    : path.join(process.cwd(), ".venv", "bin", "python");
  addLauncher(localVenv, []);

  if (process.platform === "win32") {
    addLauncher("py", ["-3"]);
    addLauncher("python", []);
    return launchers;
  }

  addLauncher("python3", []);
  addLauncher("python", []);
  return launchers;
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

  if (process.env.FORGE_SKIP_AUTO_INSTALL === "1") {
    return null;
  }

  const runtimeRoot = path.join(os.homedir(), ".cache", "forgedesk", "python-runtime");
  const runtimePython = process.platform === "win32"
    ? path.join(runtimeRoot, "Scripts", "python.exe")
    : path.join(runtimeRoot, "bin", "python");

  for (const launcher of pythonLaunchers()) {
    const createVenv = spawnSync(launcher.command, [...launcher.args, "-m", "venv", runtimeRoot], {
      stdio: "inherit",
    });
    if (createVenv.status !== 0) {
      continue;
    }

    const uvInstall = spawnSync(runtimePython, ["-m", "pip", "install", "uv"], { stdio: "ignore" });
    if (uvInstall.status === 0) {
      const uvInstallForge = spawnSync(
        runtimePython,
        ["-m", "uv", "pip", "install", "--python", runtimePython, "forge-framework"],
        { stdio: "inherit" },
      );
      if (uvInstallForge.status === 0) {
        return { command: runtimePython, args: [] };
      }
    }

    const install = spawnSync(
      runtimePython,
      ["-m", "pip", "install", "forge-framework"],
      { stdio: "inherit" },
    );
    if (install.status === 0) {
      return { command: runtimePython, args: [] };
    }
  }
  return null;
}

const launcher = findLauncher();
if (!launcher) {
  console.error("\x1b[1;31m✖\x1b[0m \x1b[1mUnable to bootstrap Forge.\x1b[0m\n\x1b[33mInstall Python 3.14+ with pip (or ensurepip) and run:\x1b[0m \x1b[36mpython -m pip install forge-framework\x1b[0m");
  process.exit(1);
}

const argv = process.argv.slice(2);
process.exit(
  spawnSync(launcher.command, [...launcher.args, "-m", "forge_cli.main", "create", ...argv], {
    stdio: "inherit",
  }).status ?? 1,
);

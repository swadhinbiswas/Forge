"""Load built-in extension APIs based on :class:`forge.config.BuiltinPluginsConfig`."""

from __future__ import annotations

import logging
from typing import TYPE_CHECKING, Any

if TYPE_CHECKING:
    from forge.app import ForgeApp

logger = logging.getLogger(__name__)


def register_builtin_plugins(app: ForgeApp) -> None:
    if not app.has_capability("forge_extensions"):
        return

    cfg = app.config.builtin_plugins

    _reg: list[tuple[str, bool, Any]] = [
        ("database", cfg.database, _load_database),
        ("auth", cfg.auth, _load_auth),
        ("cloud_sync", cfg.cloud_sync, _load_cloud),
        ("media", cfg.media, _load_media),
        ("network", cfg.network, _load_network),
        ("hardware", cfg.hardware, _load_hardware),
        ("ai_ml", cfg.ai_ml, _load_ai),
        ("telemetry", cfg.telemetry, _load_telemetry),
        ("theme", cfg.theme, _load_theme),
        ("scheduler", cfg.scheduler, _load_scheduler),
        ("crypto", cfg.crypto, _load_crypto),
        ("compression", cfg.compression, _load_compression),
        ("fs_tools", cfg.fs_tools, _load_fs_tools),
        ("memory_cache", cfg.memory_cache, _load_memory_cache),
        ("i18n", cfg.i18n, _load_i18n),
        ("archive", cfg.archive, _load_archive),
        ("file_watch", cfg.file_watch, _load_file_watch),
        ("serialization", cfg.serialization, _load_serialization),
    ]

    for name, enabled, loader in _reg:
        if not enabled:
            continue
        try:
            loader(app)
        except Exception as exc:
            logger.error("builtin plugin %s failed to load: %s", name, exc)


def _load_database(app: ForgeApp) -> None:
    from forge.builtins import database as mod

    mod.register(app)


def _load_auth(app: ForgeApp) -> None:
    from forge.builtins import auth as mod

    mod.register(app)


def _load_cloud(app: ForgeApp) -> None:
    from forge.builtins import cloud_sync as mod

    mod.register(app)


def _load_media(app: ForgeApp) -> None:
    from forge.builtins import media as mod

    mod.register(app)


def _load_network(app: ForgeApp) -> None:
    from forge.builtins import network as mod

    mod.register(app)


def _load_hardware(app: ForgeApp) -> None:
    from forge.builtins import hardware as mod

    mod.register(app)


def _load_ai(app: ForgeApp) -> None:
    from forge.builtins import ai_ml as mod

    mod.register(app)


def _load_telemetry(app: ForgeApp) -> None:
    from forge.builtins import telemetry as mod

    mod.register(app)


def _load_theme(app: ForgeApp) -> None:
    from forge.builtins import theme as mod

    mod.register(app)


def _load_scheduler(app: ForgeApp) -> None:
    from forge.builtins import scheduler as mod

    mod.register(app)


def _load_crypto(app: ForgeApp) -> None:
    from forge.builtins import crypto as mod

    mod.register(app)


def _load_compression(app: ForgeApp) -> None:
    from forge.builtins import compression as mod

    mod.register(app)


def _load_fs_tools(app: ForgeApp) -> None:
    from forge.builtins import fs_tools as mod

    mod.register(app)


def _load_memory_cache(app: ForgeApp) -> None:
    from forge.builtins import memory_cache as mod

    mod.register(app)


def _load_i18n(app: ForgeApp) -> None:
    from forge.builtins import i18n as mod

    mod.register(app)


def _load_archive(app: ForgeApp) -> None:
    from forge.builtins import archive as mod

    mod.register(app)


def _load_file_watch(app: ForgeApp) -> None:
    from forge.builtins import file_watch as mod

    mod.register(app)


def _load_serialization(app: ForgeApp) -> None:
    from forge.builtins import serialization as mod

    mod.register(app)

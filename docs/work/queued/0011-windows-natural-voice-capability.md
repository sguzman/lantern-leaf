# 0011 — Windows Natural/HD voice capability

## Outcome

Accurately expose the highest-quality supported Windows speech voices available to LanternLeaf, including the user's installed Windows Natural/Narrator voices if Microsoft exposes a supported application API for them, without regressing the current WinRT Windows voice backend.

## Why now

The user has installed additional Windows voices including Microsoft Aria (Natural), Guy (Natural), and Jenny (Natural), but LanternLeaf's current catalog is based on `Windows.Media.SpeechSynthesis::SpeechSynthesizer::AllVoices()` and those newly installed voices do not appear on this machine.

This should not be treated as a stale-catalog/refresh bug without evidence. The goal must first establish what supported Windows API surfaces actually expose these voice models to third-party desktop applications.

## Authorized passes

### A — capability research/probe first

Before changing the UI/backend:

- document the currently used WinRT `SpeechSynthesizer` / `AllVoices` contract;
- identify supported Microsoft APIs, packages, or runtime surfaces for Natural/Narrator/HD voices on current Windows 11;
- build a narrow local capability probe if needed;
- distinguish voices visible in Windows Settings/Narrator from voices actually available to application synthesis APIs.

Do not assume Narrator-installed voices must appear through `AllVoices()`.

### B — supported integration if available

If Microsoft provides a supported usable API for these voices:

- model it as an explicit Windows backend/capability rather than pretending it is the existing catalog;
- enumerate stable display names and stable machine identifiers where available;
- synthesize through the supported API off the UI thread;
- preserve canonical first-sample playback boundary behavior;
- integrate with the existing app-default preference and per-book override layering without persisting fragile display-only identifiers as if they were stable IDs.

### C — explicit limitation if unavailable

If no supported local third-party synthesis API exposes the installed Natural/Narrator models:

- keep the current working WinRT voice backend intact;
- make the capability distinction visible/documented enough that `Refresh Windows voice catalog` does not imply it will discover Narrator-only voices;
- report the supported options and the evidence for the limitation.

Do not ship undocumented encryption-key extraction, private model decryption, registry spelunking, reverse-engineered Narrator internals, or other brittle hacks as the default implementation.

## Preserve

- working Windows David/Zira/Mark-style voice enumeration and synthesis;
- app-level portable default voice preference;
- per-book explicit voice override persistence;
- backend-neutral ReaderSession/TTS semantics;
- first-sample sentence boundaries and pretty/text-only synchronization;
- transactional backend switching and failure recovery.

## Acceptance gates

1. Repository documentation/tests clearly distinguish current WinRT voices from Natural/Narrator/HD capability.
2. The implementation is grounded in a supported Windows API contract or explicitly records that the supported surface is unavailable.
3. If integrated, installed supported Natural/HD voices enumerate and synthesize successfully on representative Windows hardware without blocking egui.
4. Existing Windows voice catalog and synthesis continue to work.
5. Voice preference/per-book override semantics remain stable across the new capability.
6. Unsupported Natural voice availability never breaks ordinary Windows TTS.
7. Windows CI and representative local capability validation pass as far as hosted Windows permits.

## Non-goals

- changing the user's chosen default voice before they request it;
- Piper provisioning/model browser;
- Caliberate covers/materialization;
- PDF implementation.

## Repository handoff

This goal remains queued. Do not promote it ahead of the active Goal 0009 correction or any higher-priority director-selected library reliability work.

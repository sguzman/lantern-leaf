# 0018 — Windows QA bootstrap idempotence

## Outcome

Make repeated repo-native Windows QA invocations reliable in the same PowerShell process instead of allowing Visual Studio environment setup to accumulate until `VsDevCmd.bat` fails with `The input line is too long`.

## Starting evidence

Goal 0016 physical QA hit this failure twice in a reused PowerShell process:

`The input line is too long.`
`VsDevCmd failed with exit code 255`

Opening a fresh PowerShell process cleared the failure and product QA proceeded normally.

Current `scripts/windows-dev-env.ps1` invokes `VsDevCmd.bat ... && set`, then writes every returned environment variable back into the current process. Repeated invocation therefore needs explicit idempotence discipline rather than assuming a pristine shell.

## Architectural direction

Preserve repo-native `qa.ps1` as the normal Windows entrypoint. Do not require the human to remember to open a fresh terminal between runs.

Prefer one of these bounded approaches:

- detect an already-valid MSVC environment and skip re-running `VsDevCmd` when the required compiler/toolchain variables are already present and probes pass; or
- sanitize/replace Visual Studio path/environment variables from a known baseline before importing a newly generated environment.

Do not blindly append duplicate PATH/INCLUDE/LIB/LIBPATH segments across runs.

## Acceptance gates

1. Run `qa.ps1 -PrepareOnly` repeatedly in the same PowerShell process; every invocation succeeds.
2. PATH/INCLUDE/LIB/LIBPATH do not grow without bound across repeated runs.
3. MSVC C and C++ probes still execute successfully.
4. Fresh-shell bootstrap still works when no VS environment is present.
5. Scoop shims, Cargo, CMake, Ninja, Pandoc, and MSBuild remain discoverable.
6. Hosted Windows CI and repo-native QA preparation remain green.

## Priority

Queued engineering cleanup. It is not a product-runtime blocker and should not preempt the next substantive PDF gate unless repeated QA bootstrap failures become frequent enough to obstruct development.

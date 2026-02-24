import { spawnSync } from "node:child_process";
import { mkdirSync, writeFileSync } from "node:fs";
import { fileURLToPath } from "node:url";
import path from "node:path";

const __dirname = fileURLToPath(new URL(".", import.meta.url));
const uiRoot = path.resolve(__dirname, "..");
const repoRoot = path.resolve(uiRoot, "..");

function parsePositiveInt(value, fallback) {
  const parsed = Number.parseInt(String(value ?? ""), 10);
  if (!Number.isFinite(parsed) || parsed < 1) {
    return fallback;
  }
  return parsed;
}

function parseArgs(argv) {
  let iterations = parsePositiveInt(process.env.TAURI_SOAK_ITERATIONS, 3);
  let reportPath = process.env.TAURI_SOAK_REPORT?.trim() || path.resolve(repoRoot, "tmp", "tauri-soak-report.json");
  let stopOnFailure = true;

  for (let idx = 0; idx < argv.length; idx += 1) {
    const token = argv[idx];
    if (token === "--iterations") {
      iterations = parsePositiveInt(argv[idx + 1], iterations);
      idx += 1;
      continue;
    }
    if (token.startsWith("--iterations=")) {
      iterations = parsePositiveInt(token.split("=")[1], iterations);
      continue;
    }
    if (token === "--report") {
      reportPath = path.resolve(argv[idx + 1]);
      idx += 1;
      continue;
    }
    if (token.startsWith("--report=")) {
      reportPath = path.resolve(token.split("=")[1]);
      continue;
    }
    if (token === "--keep-going") {
      stopOnFailure = false;
    }
  }

  return {
    iterations,
    reportPath,
    stopOnFailure
  };
}

function parseSmokeDurationMs(stdout, stderr) {
  const combined = `${stdout}\n${stderr}`;
  const match = combined.match(
    /tauri runner opens source and exercises core reader controls \((\d+(?:\.\d+)?)ms\)/
  );
  if (!match) {
    return null;
  }
  return Number.parseFloat(match[1]);
}

function summarizeDurations(durations) {
  if (durations.length === 0) {
    return {
      avg_ms: null,
      min_ms: null,
      max_ms: null,
      p95_ms: null
    };
  }

  const sorted = [...durations].sort((a, b) => a - b);
  const avg = sorted.reduce((sum, value) => sum + value, 0) / sorted.length;
  const percentileIndex = Math.min(sorted.length - 1, Math.ceil(sorted.length * 0.95) - 1);

  return {
    avg_ms: Number(avg.toFixed(3)),
    min_ms: Number(sorted[0].toFixed(3)),
    max_ms: Number(sorted[sorted.length - 1].toFixed(3)),
    p95_ms: Number(sorted[percentileIndex].toFixed(3))
  };
}

function runOneIteration(iterationNumber) {
  const startedAt = Date.now();
  const result = spawnSync("node", ["--test", "e2e-tauri/smoke.test.mjs"], {
    cwd: uiRoot,
    env: process.env,
    encoding: "utf8",
    maxBuffer: 1024 * 1024 * 50
  });
  const finishedAt = Date.now();

  const durationMs = parseSmokeDurationMs(result.stdout ?? "", result.stderr ?? "");

  process.stdout.write(`\n=== Tauri soak iteration ${iterationNumber} ===\n`);
  if (result.stdout) {
    process.stdout.write(result.stdout);
  }
  if (result.stderr) {
    process.stderr.write(result.stderr);
  }
  process.stdout.write(`=== End iteration ${iterationNumber} (exit ${result.status ?? 1}) ===\n\n`);

  return {
    iteration: iterationNumber,
    started_at_unix_ms: startedAt,
    finished_at_unix_ms: finishedAt,
    wall_duration_ms: finishedAt - startedAt,
    smoke_duration_ms: durationMs,
    status: result.status === 0 ? "passed" : "failed",
    exit_code: result.status ?? 1
  };
}

function main() {
  const config = parseArgs(process.argv.slice(2));
  const startedAt = Date.now();
  const runs = [];

  process.stdout.write(
    `Starting Tauri soak run: iterations=${config.iterations} stop_on_failure=${config.stopOnFailure} report=${config.reportPath}\n`
  );

  for (let iteration = 1; iteration <= config.iterations; iteration += 1) {
    const run = runOneIteration(iteration);
    runs.push(run);
    if (run.status === "failed" && config.stopOnFailure) {
      break;
    }
  }

  const finishedAt = Date.now();
  const passCount = runs.filter((run) => run.status === "passed").length;
  const failCount = runs.length - passCount;
  const smokeDurations = runs
    .map((run) => run.smoke_duration_ms)
    .filter((value) => typeof value === "number");

  const report = {
    started_at_unix_ms: startedAt,
    finished_at_unix_ms: finishedAt,
    iterations_requested: config.iterations,
    iterations_completed: runs.length,
    pass_count: passCount,
    fail_count: failCount,
    stop_on_failure: config.stopOnFailure,
    metrics: summarizeDurations(smokeDurations),
    runs
  };

  mkdirSync(path.dirname(config.reportPath), { recursive: true });
  writeFileSync(config.reportPath, `${JSON.stringify(report, null, 2)}\n`, "utf8");

  process.stdout.write(
    `Soak summary: pass=${passCount} fail=${failCount} avg_smoke_ms=${report.metrics.avg_ms ?? "n/a"} report=${config.reportPath}\n`
  );

  if (failCount > 0) {
    process.exitCode = 1;
  }
}

main();

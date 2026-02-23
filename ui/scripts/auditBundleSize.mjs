import fs from "node:fs";
import path from "node:path";
import process from "node:process";
import zlib from "node:zlib";

const DIST_ASSETS_DIR = path.resolve(process.cwd(), "dist", "assets");

const BUDGETS = {
  cssRawBytes: 30 * 1024,
  cssGzipBytes: 10 * 1024,
  jsRawBytes: 650 * 1024,
  jsGzipBytes: 220 * 1024
};

function formatKiB(bytes) {
  return `${(bytes / 1024).toFixed(2)} KiB`;
}

function gzipSize(bytes) {
  return zlib.gzipSync(bytes).length;
}

function assertWithinBudget(label, actual, limit) {
  if (actual > limit) {
    throw new Error(
      `${label} exceeded budget: ${formatKiB(actual)} > ${formatKiB(limit)}`
    );
  }
}

function main() {
  if (!fs.existsSync(DIST_ASSETS_DIR)) {
    throw new Error(
      `dist assets directory not found: ${DIST_ASSETS_DIR}. Run the UI build first.`
    );
  }

  const assetFiles = fs.readdirSync(DIST_ASSETS_DIR);
  const cssFile = newestAssetByExtension(assetFiles, ".css");
  const jsFile = newestAssetByExtension(assetFiles, ".js");

  if (!cssFile || !jsFile) {
    throw new Error("Expected both CSS and JS bundle outputs in dist/assets");
  }

  const cssPath = path.join(DIST_ASSETS_DIR, cssFile);
  const jsPath = path.join(DIST_ASSETS_DIR, jsFile);
  const cssBytes = fs.readFileSync(cssPath);
  const jsBytes = fs.readFileSync(jsPath);

  const cssRaw = cssBytes.length;
  const cssGzip = gzipSize(cssBytes);
  const jsRaw = jsBytes.length;
  const jsGzip = gzipSize(jsBytes);

  console.log("Bundle size audit");
  console.log(`- CSS file: ${cssFile}`);
  console.log(`  raw:  ${formatKiB(cssRaw)}`);
  console.log(`  gzip: ${formatKiB(cssGzip)}`);
  console.log(`- JS file: ${jsFile}`);
  console.log(`  raw:  ${formatKiB(jsRaw)}`);
  console.log(`  gzip: ${formatKiB(jsGzip)}`);

  assertWithinBudget("CSS raw", cssRaw, BUDGETS.cssRawBytes);
  assertWithinBudget("CSS gzip", cssGzip, BUDGETS.cssGzipBytes);
  assertWithinBudget("JS raw", jsRaw, BUDGETS.jsRawBytes);
  assertWithinBudget("JS gzip", jsGzip, BUDGETS.jsGzipBytes);
}

function newestAssetByExtension(assetFiles, extension) {
  const candidates = assetFiles.filter((file) => file.endsWith(extension));
  if (candidates.length === 0) {
    return undefined;
  }
  return candidates
    .map((file) => ({
      file,
      mtimeMs: fs.statSync(path.join(DIST_ASSETS_DIR, file)).mtimeMs
    }))
    .sort((left, right) => right.mtimeMs - left.mtimeMs)[0]?.file;
}

main();

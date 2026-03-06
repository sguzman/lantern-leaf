export interface HtmlSentenceAnchorMapDiagnostics {
  confidentMatches: number;
  fallbackMatches: number;
  cappedLeaps: number;
}

export interface HtmlSentenceAnchorMapResult {
  map: number[];
  diagnostics: HtmlSentenceAnchorMapDiagnostics;
}

export function normalizeSyncText(value: string): string {
  return value
    .toLowerCase()
    .replace(/[^\p{L}\p{N}\s]+/gu, " ")
    .replace(/\s+/g, " ")
    .trim();
}

export function buildHtmlSentenceAnchorMap(
  anchorTextsRaw: string[],
  sentences: string[],
  hintAnchors: Array<number | null | undefined>
): HtmlSentenceAnchorMapResult {
  if (anchorTextsRaw.length === 0 || sentences.length === 0) {
    return {
      map: [],
      diagnostics: {
        confidentMatches: 0,
        fallbackMatches: 0,
        cappedLeaps: 0
      }
    };
  }

  const anchorTexts = anchorTextsRaw.map((value) => normalizeSyncText(value));
  const mapped: number[] = [];
  let scanStart = 0;
  let confidentMatches = 0;
  let fallbackMatches = 0;
  let cappedLeaps = 0;

  for (let sentenceIdx = 0; sentenceIdx < sentences.length; sentenceIdx += 1) {
    const normalizedSentence = normalizeSyncText(sentences[sentenceIdx] ?? "");
    const sentencePrefix = normalizedSentence.slice(0, 120);
    const shortPrefix = normalizedSentence.slice(0, 56);
    const prev = mapped.length > 0 ? mapped[mapped.length - 1] : scanStart;
    const hintRaw = hintAnchors[sentenceIdx];
    const hint =
      hintRaw !== null && hintRaw !== undefined && Number.isFinite(hintRaw)
        ? Math.max(0, Math.min(anchorTexts.length - 1, hintRaw))
        : prev;
    const base = Math.max(prev, hint);
    const localStart = Math.max(0, Math.min(prev, hint) - 18);
    const localEnd = Math.min(anchorTexts.length - 1, base + 56);

    let found = -1;
    let foundScore = 0;

    if (sentencePrefix.length > 0) {
      for (let idx = localStart; idx <= localEnd; idx += 1) {
        const anchorText = anchorTexts[idx];
        if (!anchorText) {
          continue;
        }
        if (anchorText.includes(sentencePrefix)) {
          found = idx;
          foundScore = 1;
          break;
        }

        let score = 0;
        if (shortPrefix.length >= 24 && anchorText.includes(shortPrefix)) {
          score = Math.max(score, 0.92);
        }
        const anchorPrefix = anchorText.slice(0, Math.min(72, anchorText.length));
        if (anchorPrefix.length >= 24 && sentencePrefix.includes(anchorPrefix)) {
          score = Math.max(score, 0.78);
        }
        if (score > foundScore) {
          found = idx;
          foundScore = score;
        }
      }
    }

    if (found < 0 || foundScore < 0.88) {
      fallbackMatches += 1;
      const driftTarget = Math.max(prev, hint);
      found = Math.min(prev + 1, driftTarget);
    } else if (found - prev > 2 && foundScore < 0.99) {
      cappedLeaps += 1;
      found = prev;
    } else {
      confidentMatches += 1;
    }

    const clamped = Math.max(0, Math.min(anchorTexts.length - 1, found));
    mapped.push(clamped);
    scanStart = Math.max(scanStart, clamped);
  }

  return {
    map: mapped,
    diagnostics: {
      confidentMatches,
      fallbackMatches,
      cappedLeaps
    }
  };
}

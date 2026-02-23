export interface ReaderTopBarVisibility {
  showSentenceButtons: boolean;
  showTextModeButton: boolean;
  showSettingsButton: boolean;
  showStatsButton: boolean;
  showTtsButton: boolean;
}

export function computeReaderTopBarVisibility(widthPx: number): ReaderTopBarVisibility {
  const width = Math.max(0, widthPx);
  return {
    showSentenceButtons: width >= 860,
    showTextModeButton: width >= 980,
    showSettingsButton: width >= 1090,
    showStatsButton: width >= 1200,
    showTtsButton: width >= 1310
  };
}

export interface ReaderTtsControlVisibility {
  showPlayButton: boolean;
  showPauseButton: boolean;
  showPlayPageButton: boolean;
  showPlayHighlightButton: boolean;
  showPrevSentenceButton: boolean;
  showNextSentenceButton: boolean;
  showRepeatButton: boolean;
}

export function computeReaderTtsControlVisibility(widthPx: number): ReaderTtsControlVisibility {
  const width = Math.max(0, widthPx);
  return {
    showPlayButton: width >= 620,
    showPauseButton: width >= 700,
    showPlayPageButton: width >= 180,
    showPlayHighlightButton: width >= 270,
    showPrevSentenceButton: width >= 360,
    showNextSentenceButton: width >= 450,
    showRepeatButton: width >= 540
  };
}

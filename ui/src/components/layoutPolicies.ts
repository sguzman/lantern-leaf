export interface ReaderTopBarVisibility {
  showSentenceButtons: boolean;
  showJumpButton: boolean;
  showTextModeButton: boolean;
  showThemeButton: boolean;
  showSettingsButton: boolean;
  showStatsButton: boolean;
  showTtsButton: boolean;
}

export function computeReaderTopBarVisibility(widthPx: number): ReaderTopBarVisibility {
  const width = Math.max(0, widthPx);
  return {
    showSentenceButtons: width >= 860,
    showJumpButton: width >= 980,
    showTextModeButton: width >= 1090,
    showThemeButton: width >= 1200,
    showSettingsButton: width >= 1310,
    showStatsButton: width >= 1420,
    showTtsButton: width >= 1530
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

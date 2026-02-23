import type { CalibreBook } from "../types";

export type CalibreSort =
  | "title_asc"
  | "title_desc"
  | "author_asc"
  | "author_desc"
  | "year_desc"
  | "year_asc"
  | "id_asc"
  | "id_desc";

export interface VirtualWindow<T> {
  items: T[];
  topSpacerPx: number;
  bottomSpacerPx: number;
  totalCount: number;
  startIndex: number;
  endIndex: number;
}

export function filterAndSortCalibreBooks(
  calibreBooks: CalibreBook[],
  query: string,
  sort: CalibreSort
): CalibreBook[] {
  const normalized = query.trim().toLowerCase();
  const filtered = calibreBooks.filter((book) => {
    if (!normalized) {
      return true;
    }
    return (
      book.title.toLowerCase().includes(normalized) ||
      book.authors.toLowerCase().includes(normalized) ||
      book.extension.toLowerCase().includes(normalized)
    );
  });

  const sorted = [...filtered];
  sorted.sort((left, right) => {
    switch (sort) {
      case "title_desc":
        return right.title.localeCompare(left.title);
      case "author_asc":
        return left.authors.localeCompare(right.authors);
      case "author_desc":
        return right.authors.localeCompare(left.authors);
      case "year_desc":
        return (right.year ?? 0) - (left.year ?? 0);
      case "year_asc":
        return (left.year ?? 0) - (right.year ?? 0);
      case "id_asc":
        return left.id - right.id;
      case "id_desc":
        return right.id - left.id;
      case "title_asc":
      default:
        return left.title.localeCompare(right.title);
    }
  });
  return sorted;
}

export function computeVirtualWindow<T>(
  items: T[],
  scrollTop: number,
  rowHeight: number,
  viewportHeight: number,
  overscan: number
): VirtualWindow<T> {
  const totalCount = items.length;
  const safeRowHeight = Math.max(1, Math.floor(rowHeight));
  const safeViewportHeight = Math.max(1, Math.floor(viewportHeight));
  const safeOverscan = Math.max(0, Math.floor(overscan));

  if (totalCount === 0) {
    return {
      items: [],
      topSpacerPx: 0,
      bottomSpacerPx: 0,
      totalCount: 0,
      startIndex: 0,
      endIndex: 0
    };
  }

  const start = Math.max(0, Math.floor(scrollTop / safeRowHeight) - safeOverscan);
  const maxVisible = Math.ceil(safeViewportHeight / safeRowHeight) + safeOverscan * 2;
  const end = Math.min(totalCount, start + maxVisible);

  return {
    items: items.slice(start, end),
    topSpacerPx: start * safeRowHeight,
    bottomSpacerPx: Math.max(0, (totalCount - end) * safeRowHeight),
    totalCount,
    startIndex: start,
    endIndex: end
  };
}

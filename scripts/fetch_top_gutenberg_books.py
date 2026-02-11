#!/usr/bin/env python3
"""Temporary script to fetch top Gutenberg books as plain text.

- Scrapes https://www.gutenberg.org/browse/scores/top
- Extracts the top N ebook IDs and titles from the "Top 100 EBooks yesterday" section
- Downloads each book's plain text
- Saves into data/books using sanitized title-based filenames
"""

from __future__ import annotations

import argparse
import html
import re
import sys
import time
from pathlib import Path
from typing import Iterable, List, Optional, Tuple
from urllib.error import HTTPError, URLError
from urllib.request import Request, urlopen

TOP_URL = "https://www.gutenberg.org/browse/scores/top"
USER_AGENT = "delta-encoding-book-fetcher/0.1 (+https://www.gutenberg.org/)"


def fetch_url(url: str, timeout: int = 30) -> bytes:
    req = Request(url, headers={"User-Agent": USER_AGENT})
    with urlopen(req, timeout=timeout) as resp:
        return resp.read()


def extract_top_books(top_page_html: str, limit: int) -> List[Tuple[int, str]]:
    # Each list entry looks like:
    # <li><a href="/ebooks/1342">Pride and Prejudice by Jane Austen (12345)</a></li>
    entry_pattern = re.compile(r'<a\s+href="/ebooks/(\d+)">(.*?)</a>', flags=re.DOTALL | re.IGNORECASE)
    sections = re.findall(
        r"<h2[^>]*>\s*Top\s+100\s+EBooks[^<]*</h2>\s*<ol[^>]*>(.*?)</ol>",
        top_page_html,
        flags=re.DOTALL | re.IGNORECASE,
    )

    blocks = sections
    if not blocks:
        # Fallback: consume all ordered lists if headings changed.
        blocks = re.findall(r"<ol[^>]*>(.*?)</ol>", top_page_html, flags=re.DOTALL | re.IGNORECASE)

    books: List[Tuple[int, str]] = []
    seen_ids = set()

    for block in blocks:
        for ebook_id_str, raw_title in entry_pattern.findall(block):
            ebook_id = int(ebook_id_str)
            if ebook_id in seen_ids:
                continue
            seen_ids.add(ebook_id)

            text = html.unescape(re.sub(r"<.*?>", "", raw_title)).strip()
            # Remove trailing "(downloads)" if present.
            text = re.sub(r"\s*\(\d+\)\s*$", "", text)
            books.append((ebook_id, text))
            if len(books) >= limit:
                return books

    if not books:
        raise RuntimeError("No book entries found on Gutenberg top page.")
    return books


def sanitize_filename(name: str, max_len: int = 140) -> str:
    name = name.strip().lower()
    name = re.sub(r"[^a-z0-9]+", "_", name)
    name = name.strip("_")
    if not name:
        name = "untitled"
    if len(name) > max_len:
        name = name[:max_len].rstrip("_")
    return name


def text_candidate_urls(ebook_id: int) -> Iterable[str]:
    # Try common Gutenberg plain-text URL patterns.
    yield f"https://www.gutenberg.org/cache/epub/{ebook_id}/pg{ebook_id}.txt"
    yield f"https://www.gutenberg.org/cache/epub/{ebook_id}/pg{ebook_id}.txt.utf8"
    yield f"https://www.gutenberg.org/cache/epub/{ebook_id}/pg{ebook_id}.txt.utf-8"
    yield f"https://www.gutenberg.org/files/{ebook_id}/{ebook_id}-0.txt"
    yield f"https://www.gutenberg.org/files/{ebook_id}/{ebook_id}.txt"


def download_book_text(ebook_id: int) -> Optional[str]:
    for url in text_candidate_urls(ebook_id):
        try:
            data = fetch_url(url)
            # Decode with utf-8 first, fallback to latin-1 for older texts.
            try:
                return data.decode("utf-8")
            except UnicodeDecodeError:
                return data.decode("latin-1")
        except HTTPError as exc:
            if exc.code in (403, 404):
                continue
            raise
        except URLError:
            continue
    return None


def main() -> int:
    parser = argparse.ArgumentParser(description="Fetch top Project Gutenberg books as .txt files")
    parser.add_argument("--limit", type=int, default=150, help="Number of books to fetch (default: 150)")
    parser.add_argument(
        "--out-dir",
        type=Path,
        default=Path("data/books"),
        help="Output directory for downloaded books (default: data/books)",
    )
    parser.add_argument(
        "--delay",
        type=float,
        default=0.2,
        help="Delay in seconds between book download attempts (default: 0.2)",
    )
    args = parser.parse_args()

    out_dir: Path = args.out_dir
    out_dir.mkdir(parents=True, exist_ok=True)

    print(f"Fetching top page: {TOP_URL}")
    top_html = fetch_url(TOP_URL).decode("utf-8", errors="replace")
    books = extract_top_books(top_html, args.limit)

    print(f"Found {len(books)} book entries. Downloading to {out_dir} ...")
    downloaded = 0
    skipped = 0

    used_names = set()
    for idx, (ebook_id, title) in enumerate(books, start=1):
        safe = sanitize_filename(title)
        file_name = f"{safe}.txt"

        # Ensure uniqueness if sanitized names collide.
        if file_name in used_names:
            file_name = f"{safe}_{ebook_id}.txt"
        used_names.add(file_name)

        target = out_dir / file_name

        text = download_book_text(ebook_id)
        if text is None:
            skipped += 1
            print(f"[{idx}/{len(books)}] SKIP id={ebook_id} title={title!r} (no .txt found)")
            continue

        target.write_text(text, encoding="utf-8")
        downloaded += 1
        print(f"[{idx}/{len(books)}] OK   id={ebook_id} -> {target.name}")

        if args.delay > 0:
            time.sleep(args.delay)

    print(f"Done. Downloaded={downloaded}, skipped={skipped}, requested={len(books)}")
    return 0


if __name__ == "__main__":
    try:
        raise SystemExit(main())
    except KeyboardInterrupt:
        print("Interrupted.", file=sys.stderr)
        raise SystemExit(130)

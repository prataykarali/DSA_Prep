import sys


def count_words(text):
    """Count word frequencies in the given text."""
    word_counts = {}
    for word in text.lower().split():
        word_counts[word] = word_counts.get(word, 0) + 1
    return word_counts


def sort_by_frequency(word_counts):
    """Sort word counts by frequency (highest first), then alphabetically."""
    sorted_words = sorted(word_counts.items(), key=lambda item: (-item[1], item[0]))
    return sorted_words


def print_bar_chart(sorted_words, top_n=10, max_bar_width=20):
    """Print a horizontal bar chart for the top N words."""
    top_words = sorted_words[:top_n]
    if not top_words:
        print("No words to display.")
        return

    max_count = top_words[0][1]

    print(f"\n{'Word':<15} {'Bar':<{max_bar_width + 2}} {'Count':>5}")
    print("-" * (15 + max_bar_width + 9))
    for word, count in top_words:
        bar_length = int((count / max_count) * max_bar_width)
        bar = "#" * bar_length
        print(f"{word:<15} {bar:<{max_bar_width}}  ({count})")


def main():
    if len(sys.argv) < 2:
        print("Usage: python3 word_freq.py <file_path>")
        sys.exit(1)

    file_path = sys.argv[1]

    try:
        with open(file_path, "r") as f:
            text = f.read()
    except FileNotFoundError:
        print(f"Error: File '{file_path}' not found.")
        sys.exit(1)

    print("=== Word Frequency Analyzer (Python) ===\n")

    word_counts = count_words(text)
    sorted_words = sort_by_frequency(word_counts)

    print(f"Total unique words: {len(word_counts)}")
    print(f"Showing top 10 most frequent words:\n")

    print_bar_chart(sorted_words, top_n=10)


if __name__ == "__main__":
    main()
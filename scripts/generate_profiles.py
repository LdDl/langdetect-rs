# Gracefully stolen from here: https://github.com/fedelopez77/langdetect/blob/master/langdetect.py

import json
import re
import string
from collections import Counter
import os

# Language codes and output paths
# ISO 639-1/ISO 639-3 codes are used in this example
LANGUAGES = {
    # "uz": "./datasets/downloads/uz/",
    # "tg": "./datasets/downloads/tg/",
    # "az": "./datasets/downloads/az/",
    "sah": "./datasets/downloads/sah/"
}

# Output directory for generated profiles
output_dir = "./datasets/generated"

# Suffix for generated profile files
files_suffix = "_generated"

# Taken from gensim
def to_unicode(text, encoding='utf8', errors='strict'):
    """Convert a string (bytestring in `encoding` or unicode), to unicode."""
    if isinstance(text, str):
        return text
    return str(text, encoding, errors=errors)

# Taken from gensim
RE_PUNCT = re.compile('([%s])+' % re.escape(string.punctuation), re.UNICODE)
def strip_punctuation(s):
    return RE_PUNCT.sub(" ", s)

# Taken from gensim
RE_NUMERIC = re.compile(r"[0-9]+", re.UNICODE)
def strip_numeric(s):
    return RE_NUMERIC.sub("", s)

def clean_text(text):
    cleaning_functions = [to_unicode, lambda x: x.lower(), strip_punctuation, strip_numeric]
    for f in cleaning_functions:
        text = f(text)
    return text

def add_padding(text):
    # Add padding for n-gram generation
    # MAX_NGRAM - 1, assuming MAX_NGRAM=3
    padding = " " * (3 - 1)
    return " " + text + padding

def create_ngrams(text):
    text = add_padding(text)
    # 1 to 3
    for length in range(1, 4):
        for i in range(len(text) - length + 1):
            yield text[i:i + length]

def create_langdetect_profile(text: str, lang_code: str) -> dict:
    text = clean_text(text)
    # Generate n-grams
    ngram_freq = {}
    for ngram in create_ngrams(text):
        if ngram in ngram_freq:
            ngram_freq[ngram] += 1
        else:
            ngram_freq[ngram] = 1
    # Separate by length for counting
    uni_freq = {k: v for k, v in ngram_freq.items() if len(k.strip()) == 1}
    bi_freq = {k: v for k, v in ngram_freq.items() if len(k.strip()) == 2}
    tri_freq = {k: v for k, v in ngram_freq.items() if len(k.strip()) == 3}
    # Get top 300 most common for each, but combine into one freq dict
    uni_top = Counter(uni_freq).most_common(300)
    bi_top = Counter(bi_freq).most_common(300)
    tri_top = Counter(tri_freq).most_common(300)
    # Combine all into one freq dict
    combined_freq = {}
    for gram, count in uni_top + bi_top + tri_top:
        combined_freq[gram] = count
    # n_words: total counts for each n-gram type
    n_words = [
        sum(uni_freq.values()),
        sum(bi_freq.values()),
        sum(tri_freq.values())
    ]
    profile = {
        "freq": combined_freq,
        "n_words": n_words,
        "name": lang_code.lower()
    }
    return profile

if __name__ == "__main__":
    # Create output directory
    os.makedirs(output_dir, exist_ok=True)
    # Generate profiles for each language
    for lang_code, path in LANGUAGES.items():
        text = ""
        for filename in os.listdir(path):
            with open(os.path.join(path, filename), "r", encoding="utf-8") as f:
                text += f.read() + "\n"
        profile = create_langdetect_profile(text, lang_code)
        output_file = f"{output_dir}/{lang_code}{files_suffix}.json"
        with open(output_file, "w", encoding="utf-8") as f:
            json.dump(profile, f, ensure_ascii=False, indent=2)
        print(f"Generated profile for {lang_code} and saved to {output_file}")
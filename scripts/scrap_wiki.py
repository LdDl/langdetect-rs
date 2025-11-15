## Gracefully stolen from here: https://github.com/fedelopez77/langdetect/blob/master/datasets/wikiscrapper.py

import os
import wikipedia

# Language codes and names
# ISO 639-1/ISO 639-3 codes are used in this example
LANGUAGES = {
    # "uz": "Uzbek",
    # "tg": "Tajik",
    # "az": "Azerbaijani",
    "sah": "Sakha" # 3-letter code because Yakut is not represented in ISO 639-1
}

# Maximum size of articles to scrape (in kB)
MAX_SIZE_OF_ARTICLES = 50000
# Maximum number of articles to scrape per language
MAX_ARTICLES = 200
# Output directory for downloaded articles
output_dir = "./datasets/downloads/"
os.makedirs(output_dir, exist_ok=True)

def get_size_of_all_articles(language):
    path = language + "/"
    if not os.path.exists(path):
        return 0
    return sum(os.path.getsize(path + f) for f in os.listdir(path) if os.path.isfile(path + f)) / 1024  # Convert bytes to KB

for language in LANGUAGES:
    i = 1
    wikipedia.set_lang(language)
    while i <= MAX_ARTICLES and get_size_of_all_articles(language) < MAX_SIZE_OF_ARTICLES:
        try:
            page = wikipedia.page(wikipedia.random())
        except (wikipedia.DisambiguationError, wikipedia.exceptions.PageError):
            continue

        filename = os.path.join(output_dir, language, f"{i}-{language}.txt")
        os.makedirs(os.path.dirname(filename), exist_ok=True)
        try:
            with open(filename, "w", encoding="utf-8") as f:
                f.write(page.content)  # page.content is already a string
        except UnicodeEncodeError:
            # Skip this page if encoding issues
            continue
        
        i += 1
        print(f"Downloaded {i-1} articles for {language}, total size: {get_size_of_all_articles(language):.2f} KB")
    
    print(f"Finished scraping {language}: {i-1} articles, {get_size_of_all_articles(language):.2f} KB")



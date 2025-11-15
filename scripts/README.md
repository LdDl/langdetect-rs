# Useful scripts for langdetect-rs

Everything should be run from the scripts root:
```sh
cd scripts
```

Script to scrape Wikipedia articles and generating profile has been taken from this repository:
https://github.com/fedelopez77/langdetect

## Setting up Python virtual environment

```sh
python -m venv .venv-langdetect
source .venv-langdetect/bin/activate
# Deactivate when needed:
# deactivate
```

## Install dependencies

```sh
pip install -r requirements.txt
```

# Prepare dataset from Wikipedia articles

It is strongly recommended to not to scrape too many articles at once to avoid being blocked by Wikipedia. It is also better to
find dataset somewhere else with better prepared data.

Tune the [scrap_wiki.py](./scrap_wiki.py) to fit your needs:
```python
LANGUAGES = {
    "ISO CODE": "Language name"
}
# e.g.
# LANGUAGES = {
#     "sah": "Sakha"
# }

# Maximum size of articles to scrape (in kB)
MAX_SIZE_OF_ARTICLES = 50000
# Maximum number of articles to scrape per language
MAX_ARTICLES = 200
# Output directory for downloaded articles
output_dir = "./datasets/downloads/"
```

Run the script:
```sh
python scrap_wiki.py
```

This will create a folder structure for given `output_dir` like this:
```
datasets/
    downloads/
        sah/
            0-sah.txt
            1-sah.txt
            ...
        other_language/
            0-other_language.txt
            1-other_language.txt
            ...
```
# Useful scripts for langdetect-rs

Everything should be run from the scripts root:
```sh
cd scripts
```

Script to scrape Wikipedia articles and generating profile has been taken from this repository:
https://github.com/fedelopez77/langdetect

## Table of Contents

- [Setting up Python virtual environment](#setting-up-python-virtual-environment)
- [Install dependencies](#install-dependencies)
- [Prepare dataset from Wikipedia articles](#prepare-dataset-from-wikipedia-articles)
- [Generate language profile](#generate-language-profile)
- [Usage in langdetect-rs](#usage-in-langdetect-rs)


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

## Generate language profile

Tune the [generate_profiles.py.py](./generate_profiles.py.py) to fit your needs:

```python
LANGUAGES = {
    "ISO CODE": "path to dataset folder containing txt files"
}
# e.g.
# LANGUAGES = {
#     "sah": "./datasets/downloads/sah/"
# }

# Output directory for generated profiles
output_dir = "./datasets/generated"

# Suffix for generated profile files
files_suffix = "_generated"
# For this given example files would be named `ISOCODE_generated.json`
```

Important notes:
- Make sure requested language dataset folders exist and contain text files.
- Make sure dataset directory looks like this:
    ```
    path/
        to/
            dataset/
                language_code/
                    0-language_code.txt
                    1-language_code.txt
                    ...
                other_language_code/
                    0-other_language_code.txt
                    1-other_language_code.txt
                    ...
    ```
- Output directory will be created if it does not exist.
- Output directory will look like this after running the script:
    ```
    datasets/
        generated/
            sah.json
            other_language_code.json
    ```

Run the script:
```sh
python generate_profiles.py
```

## Usage in langdetect-rs

You can now use the generated profiles in langdetect-rs by loading them via `DetectorBuilder`.

You may refer to the main [README.md](../README.md) for more details on how to use custom profiles in langdetect-rs.

You may refer to [this particular example](../examples/extend_default/main.rs) to see how to extend default profiles with generated ones.
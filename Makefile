
DATA_PEOPLE_PATH := $(shell pwd)/data/people.json
DATA_PEOPLE_COUNT := 1000

INDEX_PEOPLE_PATH := $(shell pwd)/data/people

.PHONY: default generate-data build

default:
	@echo "Available targets:"
	@echo "  - generate-data"
	@echo "  - run"

generate-data:
	rm -f $(DATA_PEOPLE_PATH)

	node data/generate.js --count $(DATA_PEOPLE_COUNT) --output $(DATA_PEOPLE_PATH)

run: generate-data
	rm -rf $(INDEX_PEOPLE_PATH)
	mkdir $(INDEX_PEOPLE_PATH)

	DATA_PEOPLE_PATH="$(DATA_PEOPLE_PATH)" INDEX_PEOPLE_PATH="$(INDEX_PEOPLE_PATH)" cargo run --release --bin indexer

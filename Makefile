check:
	@for directory in */; do \
		echo $$directory; \
		cd $$directory; cargo clippy; cargo test; \
	done

.PHONY: check
ONESHELL:

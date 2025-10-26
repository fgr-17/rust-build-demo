COVERAGE_DIR := target/lcov
RUSTFLAGS := -Cinstrument-coverage
LLVM_PROFILE_FILE := $(COVERAGE_DIR)/coverage-%p-%m.profraw
COVERAGE_OUTPUT_FILE := coverage-metrics

export RUSTFLAGS
export LLVM_PROFILE_FILE

cov: clean-cov
	cargo test
	cargo llvm-cov --html
	cargo llvm-cov report --summary-only --output-path $(COVERAGE_OUTPUT_FILE)


.PHONY: clean-cov

clean-cov:
	# Remove old profile data and the target directory
	rm -f *.profraw
	rm -rf $(COVERAGE_DIR)
	rm -f $(COVERAGE_OUTPUT_FILE)

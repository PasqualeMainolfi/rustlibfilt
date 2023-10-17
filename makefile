COMPILE = pip uninstall rustlibfilt && maturin build --release && maturin develop --release
INSTALL_MATURIN = pip install maturin
UPDATE = python install_pyrf.py

.IPHONY: run
all: run update_lib

run: check
	$(COMPILE)

check:
	@if [ -x $$(command -v maturin) ]; then \
		echo "maturin is installed"; \
	else \
	    echo "maturin is not installed, installing..."; \
	    $(INSTALL_MATURIN); \
	fi

update_lib:
	$(UPDATE)


COMPILE = pip uninstall rustlibfilt && maturin build --release && maturin develop --release
INSTALL_MATURIN = pip install maturin

.IPHONY: run
all: run

run: check
	$(COMPILE)

check:
	@if [ -x $$(command -v maturin) ]; then \
		echo "maturin is installed"; \
	else \
	    echo "maturin is not installed, installing..."; \
	    $(INSTALL_MATURIN); \
	fi


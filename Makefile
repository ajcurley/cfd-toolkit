build:
	@maturin develop

clean:
	@cargo clean

fmt:
	@black .

check_fmt:
	@black --check .

lint:
	@ruff check .

test: build
	@pytest

tools:
	@rm -rf env/
	@python3 -m venv env
	@source env/bin/activate && pip install -r tests/requirements.txt

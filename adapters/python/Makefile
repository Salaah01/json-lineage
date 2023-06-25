test:
	python3 -m unittest

coverage:
	coverage run -m unittest
	coverage report
	coverage html

lint:
	ruff check .

format:
	ruff check . --fix
	isort .

mypy:
	mypy -p json_lineage
	mypy -p tests

build:
	python3 setup.py sdist bdist_wheel

upload:
	twine upload dist/*
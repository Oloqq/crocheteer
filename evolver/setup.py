from setuptools import setup, find_packages

setup(
    name="evolver",
    version="0.0.1",
    packages=find_packages(),
    entry_points={
        'console_scripts': [
            'evo = src.main:main',
        ],
    },
)
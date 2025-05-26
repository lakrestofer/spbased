#!/usr/bin/env python

from setuptools import setup, find_packages

setup(
    name="flashcard_image",
    version="0.0.5",
    # Modules to import from other scripts:
    packages=find_packages(),
    # Executables
    scripts=["flashcard_image.py"],
)

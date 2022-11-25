from setuptools import setup, Extension
from Cython.Build import cythonize

ext = Extension(
    "ByteStream",
    sources = ["./src/types/ByteStream.pyx"],
    language = "c++",
)

setup(
    name = "ByteStream",
    ext_modules = cythonize(
        ext,
        language_level = '3',
        compiler_directives = {
            'language_level': '3'
        },
        annotate = True,
    )
)

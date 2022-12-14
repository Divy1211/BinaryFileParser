from setuptools import setup, find_packages

with open("requirements.txt", "r", encoding = 'utf-8') as file:
    requires = file.read().splitlines()

setup(
    name='binary-file-parser',
    version='0.0.0',
    license='MIT',
    author="Divy1211",
    author_email='divy1211.dc@gmail.com',
    packages=find_packages('src'),
    package_dir={'': 'src'},
    url='https://github.com/Divy1211/FileParser',
    keywords='File Parser',
    install_requires=requires,
)

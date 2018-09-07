from setuptools import setup
from setuptools_rust import Binding, RustExtension

setup(
    name="pyef",
    version="1.0",
    rust_extensions=[RustExtension("pyef.native", binding=Binding.PyO3)],
    packages=["pyef"],
    zip_safe=False,
)

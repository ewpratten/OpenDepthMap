from setuptools import setup
from setuptools_rust import RustExtension


setup(
    name="odm",
    version="0.1.0",
    classifiers=[
        "Development Status :: 3 - Alpha",
        "Intended Audience :: Developers",
        "Programming Language :: Python",
        "Programming Language :: Rust",
    ],
    packages=["odm"],
    rust_extensions=[RustExtension("pylibodm.pylibodm", "pylibodm/Cargo.toml", debug=False)],
    include_package_data=True,
    zip_safe=False,
)
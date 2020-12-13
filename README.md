# oddity-engine

The anomaly detection and tools engine used by Oddity. The engine is used as a Python binding with PyO3, and therefore is currently not intended to be used as a standalone Rust library. 

The engine provides some of the following functionalities:

- STL decompositon
- Time series collection and tools
- Gaussian process fitting
- Gaussian kernels
- Gaussian distribition fitting
- FFT for periodicity inference

More functionality and general optimizations will be added in the future.

## Important Links

The following are some important links for more information:

> PyPi: [https://pypi.org/project/oddity/](https://pypi.org/project/oddity/)

> Oddity: [https://github.com/Lleyton-Ariton/oddity](https://github.com/Lleyton-Ariton/oddity)

> Oddity Demo: [https://github.com/Lleyton-Ariton/oddity-demo](https://github.com/Lleyton-Ariton/oddity-demo)
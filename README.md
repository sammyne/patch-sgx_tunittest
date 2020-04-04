# patch-sgx_tunittest

To run the project, ensure the submodule is ready by executing 

```bash
git submodule update --init
```

The `should_panic!` macro in `sgx_tunittest` uses `std::panic`, which requires us the imports
`std::panic` every time we use `should_panic!`.

To verify, just comment out the patch section of Cargo.toml.

This project demonstrates the this inconvenience, and helps to support [this patch](https://github.com/sammyne/incubator-teaclave-sgx-sdk/commit/34a50a6307b2853d907b3a457cf9c961074f98e0#diff-b02276d3fdf8ec232bd27c1b90221758R109).

The patch mainly add the `std::` prefix to the `panic` function in use so as to avoid explict
importing the `std::panic` every time.

Hope it helps~
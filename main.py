from wasmtime import Store

# The magic, refer to https://github.com/bytecodealliance/wasmtime-py?tab=readme-ov-file#usage
import wasmtime.loader  # noqa: F401

import minimal


def main():
    store = Store()
    testt_component_instance = minimal.Root(store)

    print(testt_component_instance.answer(store))


if __name__ == "__main__":
    main()

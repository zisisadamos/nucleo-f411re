## Support package for the [Nucleo-f411re](https://www.st.com/en/evaluation-tools/nucleo-f411re.html) board.

### How to use the examples:

1. Clone this repository

2. Flash blinky or echo example

 ```
 openocd -f nucleo.cfg -c"program target/thumbv7m-none-eabi/debug/examples/gpio_hal_blinky verify reset exit"
 ```

 ```
 openocd -f nucleo.cfg -c"program target/thumbv7m-none-eabi/debug/examples/serial_hal_blocking_echo verify reset exit"
 ```

This repository is based on https://github.com/therealprof/nucleo-f103rb
Memory Management

---------------------------------------------
|                  VASpace                  |
---------------------------------------------
VASegment | VASegment | VASegment | VASegment
 (heap)      (text)      (stack)     (MMIO)
---------------------------------------------
|  Physical Memory (PageAlloc)    | Dev Mem |
---------------------------------------------



GPIO + PINMUXING etc

UART Driver
    -> requests pin ctrl activate "uart_0" pins
    -> configures uart via mailbox driver
    -> ??How should access to MMIO be governed??

mailbox driver
    -> Provides interface for sending and receiving from mailbox
    -> Manages memory + serializes access to mailbox
    -> ??How should access to MMIO be governed??

pinctrl driver
    -> Manages ownership over pins for a device
    -> Calls HAL to activate and deactivate configurations
    -> Provides query functionality for pin setup

pin driver (Provides HAL)
    -> Manages state transition of pins
    -> Provides information to the pinctrl driver
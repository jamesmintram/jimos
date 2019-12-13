
#!/bin/bash

aarch64-none-elf-gdb --command="$(dirname "$0")/jlink_attach.gdb"


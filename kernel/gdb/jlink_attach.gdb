target remote localhost:3333
file build/blinky.elf

b *0x00000000000800f0 
b *0x000000000008022c 

j *0x0000000000080024 

# b enter_virtual_addressing


#layout next 
#layout next 
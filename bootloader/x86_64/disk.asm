sectalign off

%include "bootsector.asm"

startup_start:
%include "startup-x86_64.asm"

align 512, db 0
startup_end:

kernel_file:
    %defstr KERNEL_STR %[KERNEL]
    incbin KERNEL_STR
.end:
align 512, db 0

target remote :3333
set print pretty on
load
monitor itm port 0 on
break main
continue

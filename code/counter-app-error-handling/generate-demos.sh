#/bin/bash

for i in *.tape; do
    ~/go/bin/vhs $i
done
rm -f basic-app-*.gif

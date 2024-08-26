#/bin/bash

for i in *.tape; do
    vhs $i
done
rm -f basic-app-*.gif

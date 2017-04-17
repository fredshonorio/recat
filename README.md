# recat

__recat__ is a command to spawn multiple processes and concatenate their output.
Processes that stop are restarted and the number of active processes can be limited.

`recat -c 'yes 1 | head -n 100' -c 'yes 2 | head -n 3' -n 3`

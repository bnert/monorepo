# microservicer

A cli app which acts as a lightweight control plane for services.

## In depth

This app spawns as a daemon process which then acts as a local reverse proxy.
Services are registered and kept as in memory at runtime as key/value pairs.

The cli map for base funcitonality is:

msrvr list
  Lists the processes currently being monitored.

msrvradd
  Adds a process

msrvr run
  Runs a process, and adds to kv store if not already there.

msrvr stop
  Sends a kill signal to a process

msrvr delete
  If running, kills; then removes from watched

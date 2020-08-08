#!bin/bash

# redisgraph
redis-server --loadmodule /usr/lib/redis/modules/redisgraph.so &

#setup graph
sleep 2
/usr/data/imasparql_redisgraph
wait
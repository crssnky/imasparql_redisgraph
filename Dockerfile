FROM redislabs/redisgraph

COPY ./target/release/imasparql_redisgraph /usr/data/imasparql_redisgraph
COPY ./launch.sh /usr/data/launch.sh

ENTRYPOINT ["sh", "/usr/data/launch.sh"]
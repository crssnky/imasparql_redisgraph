extern crate imasparql_redisgraph as imasparql;
extern crate percent_encoding;
extern crate redis;
extern crate redisgraph;
extern crate ureq;
extern crate url;

use imasparql::structs::*;
use percent_encoding::percent_decode;
use redis::Client;
use redisgraph::{Graph, RedisGraphResult};
use url::form_urlencoded;

fn main() -> RedisGraphResult<()> {
  let separator = ",,,,,";
  let json = get_unit(separator);
  if json.len() > 0 {
    set_to_redisgraph(json, separator)
  } else {
    Ok(())
  }
}

fn get_unit(separator: &'static str) -> Vec<Bindings> {
  let quety = format!(
    "\
    PREFIX schema: <http://schema.org/>
    PREFIX imas: <https://sparql.crssnky.xyz/imasrdf/URIs/imas-schema.ttl#>
    PREFIX rdfs:  <http://www.w3.org/2000/01/rdf-schema#>

    SELECT (group_concat(?unit;separator='{}')as ?units) ?idol
    where {{
      ?s  schema:name ?idol;
          schema:memberOf/schema:name ?unit
      filter(str(lang(?idol))='en')
    }}group by ?idol",
    separator
  );
  let encoded_query = form_urlencoded::Serializer::new(String::new())
    .append_pair("output", "json")
    .append_pair("force-accept", "text/plain")
    .append_pair("query", &quety)
    .finish();
  let base_url = format!(
    "https://sparql.crssnky.xyz/spql/imas/query?{}",
    encoded_query
  );
  let res = ureq::get(&base_url).call();

  if res.ok() {
    let json_str = res.into_string().unwrap();
    let res_json: Response = serde_json::from_str(&json_str).unwrap();
    res_json.results.bindings
  } else {
    // no response
    Vec::new()
  }
}

fn set_to_redisgraph(json: Vec<Bindings>, separator: &'static str) -> RedisGraphResult<()> {
  let client = Client::open("redis://127.0.0.1")?;
  let mut connection = client.get_connection()?;
  let graph_name = "imasparql".to_string();

  let mut graph = Graph::open(&mut connection, &graph_name)?;

  let mut result: RedisGraphResult<()> = Ok(());
  for element in json {
    let units = element.units.value;
    let idol = element.idol.value;
    for unit in units.split(separator) {
      let create_query = format!(
        "CREATE (:Idol {{name:'{}'}})-[:memberOf]->(:Unit {{name: '{}'}})",
        idol,
        percent_decode(unit.as_bytes()) // ユニット名はエンコードされてるかも
          .decode_utf8()
          .unwrap()
          .to_string()
          .replace("'", "\\'")
      );
      result = graph.mutate(&create_query);
      if !result.is_ok() {
        return result;
      }
    }
    println!("{}", idol);
  }
  result
}

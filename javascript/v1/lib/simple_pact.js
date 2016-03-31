var ffi = require('ffi');
var path = require('path');
const http = require('http');
const net = require('net');
const url = require('url');

var lib = ffi.Library(path.join(__dirname, '../../../rust/v1/libpact_v1_mock_server/target/debug/libpact_v1_mock_server.so'), {
  create_mock_server: ['int32', ['string']]
});

var pact = "{\n" +
  "\"provider\": {\n" +
  "  \"name\": \"Alice Service\"\n" +
  "},\n" +
  "\"consumer\": {\n" +
  "  \"name\": \"Consumer\"\n" +
  "},\n" +
  "\"interactions\": [\n" +
  "  {\n" +
  "    \"description\": \"a retrieve Mallory request\",\n" +
  "    \"request\": {\n" +
  "      \"method\": \"GET\",\n" +
  "      \"path\": \"/mallory\",\n" +
  "      \"query\": \"name=ron&status=good\"\n" +
  "    },\n" +
  "    \"response\": {\n" +
  "      \"status\": 200,\n" +
  "      \"headers\": {\n" +
  "        \"Content-Type\": \"text/html\"\n" +
  "      },\n" +
  "      \"body\": \"\\\"That is some good Mallory.\\\"\"\n" +
  "    }\n" +
  "  }\n" +
  "],\n" +
  "\"metadata\": {\n" +
  "  \"pact-specification\": {\n" +
  "    \"version\": \"1.0.0\"\n" +
  "  },\n" +
  "  \"pact-jvm\": {\n" +
  "    \"version\": \"1.0.0\"\n" +
  "  }\n" +
  "}\n" +
"}\n";

var port = lib.create_mock_server(pact);
console.log(port);

var options = {
  hostname: 'localhost',
  port: port,
  path: '/mallory?name=ron&status=good',
  method: 'GET',
  headers: {
    'Content-Type': 'application/json'
  }
};

var req = http.request(options, (res) => {
  console.log(`STATUS: ${res.statusCode}`);
  console.log(`HEADERS: ${JSON.stringify(res.headers)}`);
  res.setEncoding('utf8');
  res.on('data', (chunk) => {
    console.log(`BODY: ${chunk}`);
  });
  res.on('end', () => {
    console.log('No more data in response.')
  })
});

req.on('error', (e) => {
  console.log(`problem with request: ${e.message}`);
});

req.end();

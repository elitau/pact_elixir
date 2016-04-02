var ffi = require('ffi');
var path = require('path');
const http = require('http');
const net = require('net');
const url = require('url');

var dll = '../../../rust/v1/libpact_v1_mock_server/target/debug/libpact_v1_mock_server';
var lib = ffi.Library(path.join(__dirname, dll), {
  create_mock_server: ['int32', ['string']],
  mock_server_matched: ['bool', ['int32']],
  mock_server_mismatches: ['string', ['int32']],
  cleanup_mock_server: ['bool', ['int32']]
});

var pact = '{\n' +
'      "provider": {\n' +
'        "name": "test_provider"\n' +
'      },\n' +
'      "consumer": {\n' +
'        "name": "test_consumer"\n' +
'      },\n' +
'      "interactions": [\n' +
'        {\n' +
'          "providerState": "test state",\n' +
'          "description": "test interaction",\n' +
'          "request": {\n' +
'            "method": "POST",\n' +
'            "path": "/",\n' +
'            "body": {\n' +
'              "complete": {\n' +
'                "certificateUri": "http://...",\n' +
'                "issues": {\n' +
'                  "idNotFound": {}\n' +
'                },\n' +
'                "nevdis": {\n' +
'                  "body": null,\n' +
'                  "colour": null,\n' +
'                  "engine": null\n' +
'                },\n' +
'                "body": 123456\n' +
'              },\n' +
'              "body": [\n' +
'                1,\n' +
'                2,\n' +
'                3\n' +
'              ]\n' +
'            }\n' +
'          },\n' +
'          "response": {\n' +
'            "status": 200\n' +
'          }\n' +
'        }\n' +
'      ],\n' +
'      "metadata": {\n' +
'        "pact-specification": {\n' +
'          "version": "2.0.0"\n' +
'        },\n' +
'        "pact-jvm": {\n' +
'          "version": ""\n' +
'        }\n' +
'      }\n' +
'    }';

var port = lib.create_mock_server(pact);
console.log("Mock server port=" + port);

if (lib.mock_server_matched(port)) {
  console.log("No requests yet, as expected");
} else {
  console.log("Hmm, something smells a bit off.");
}

var options = {
  hostname: 'localhost',
  port: port,
  path: '/',
  method: 'POST',
  headers: {
    'Content-Type': 'application/json'
  }
};

var request1_done = false;
var request2_done = false;

var req1 = http.request(options, (res) => {
  console.log(`STATUS: ${res.statusCode}`);
  console.log(`HEADERS: ${JSON.stringify(res.headers)}`);
  res.setEncoding('utf8');
  res.on('data', (chunk) => {
    console.log(`BODY: ${chunk}`);
  });
  res.on('end', () => {
    console.log('No more data in response.');
    request1_done = true;
  })
});

req1.on('error', (e) => {
  console.log(`problem with request: ${e.message}`);
  request1_done = true;
});
req1.write(JSON.stringify({
  "complete": {
    "certificateUri": "http://...",
    "issues": {},
    "nevdis": {
      "body": "red",
      "colour": null,
      "engine": null
    },
    "body": "123456"
  },
  "body": [1, 3]
}));
req1.end();

var options2 = {
  hostname: 'localhost',
  port: port,
  path: '/mallory',
  method: 'PUT',
  headers: {
    'Content-Type': 'application/json'
  }
};

var req2 = http.request(options2, (res) => {
  console.log(`STATUS: ${res.statusCode}`);
  console.log(`HEADERS: ${JSON.stringify(res.headers)}`);
  res.setEncoding('utf8');
  res.on('data', (chunk) => {
    console.log(`BODY: ${chunk}`);
  });
  res.on('end', () => {
    console.log('No more data in response.');
    request2_done = true;
  })
});

req2.on('error', (e) => {
  console.log(`problem with request: ${e.message}`);
  request2_done = true;
});
req2.write(JSON.stringify({}));
req2.end();

var waitForResult;
waitForResult = function () {
    console.log('.');
    if (!request1_done || !request2_done) {
        setTimeout(waitForResult, 1000);
    } else {
      console.log("-----------------------------------------------");
      if (lib.mock_server_matched(port)) {
        console.log("Mock server matched all requests, That Is Not Good (tm)");
      } else {
        console.log("We got some mismatches, as expected.");
        var mismatch_json = lib.mock_server_mismatches(port);
        console.log(mismatch_json);
        console.log();
        console.log(JSON.stringify(JSON.parse(mismatch_json), null, 4));
      }
      lib.cleanup_mock_server(port);
    }
};
setTimeout(waitForResult, 1000);

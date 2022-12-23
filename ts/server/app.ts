import * as http from "http";

const port = 5000;

const server = http.createServer(
  (request, response) => {
    response.end("Hello! world");
  }
);

server.listen(port);

console.log(`http://localhost:${port}`);

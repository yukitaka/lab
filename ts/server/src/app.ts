import * as http from "http";
import * as url from "url";
import { port } from "./config";
import { route } from "./router";
import * as handlers from "./handlers"

const handler = {
  "/": handlers.index
}

const server = http.createServer(
  (request, response) => {
    const pathname = url.parse(request.url!).pathname!;
    route(handler, pathname);
    response.end("Hello! world");
  }
);


server.listen(port);

console.log(`http://localhost:${port}`);

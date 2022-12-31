export function route(handler: {[key: string]: Function}, pathname: string) {
  console.log("Route for " + pathname);

  if (handler[pathname]) {
    handler[pathname]();
  } else {
    console.log("Not found a request handler for " + pathname); 
  }
}

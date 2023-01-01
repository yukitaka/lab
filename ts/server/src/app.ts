import express from "express";
import { port } from "./config";
import * as handlers from "./handlers"

const app = express()
const host = "localhost"

app.get("/", (req, res) => handlers.index(req, res))

app.listen(port, host, () => {
  console.log(`Running on http://${host}:${port}`)
})
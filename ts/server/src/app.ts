import express from "express";
import bodyParser from "body-parser"
import { port } from "./config";
import * as handlers from "./handlers"

const app = express()
app.use(bodyParser.json())

const host = "localhost"

app.get("/", (req, res) => handlers.index(req, res))
app.post("/send", (req, res) => handlers.send(req, res))
app.listen(port, host, () => {
  console.log(`Running on http://${host}:${port}`)
})
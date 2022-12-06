package com.example.plugins

import com.example.routes.customerRouting
import io.ktor.server.routing.*
import io.ktor.server.application.*

fun Application.configureRouting() {

    routing {
        customerRouting()
    }
}

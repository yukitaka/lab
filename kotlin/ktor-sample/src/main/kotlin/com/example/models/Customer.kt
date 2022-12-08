package com.example.models

import kotlinx.serialization.Serializable
import org.jetbrains.exposed.sql.*

@Serializable
data class Customer(val id: String, val firstName: String, val lastName: String, val email: String)

object Customers : Table() {
    val id = integer("id").autoIncrement()
    val firstName = varchar("First", 128)
    val lastName = varchar("Last", 128)

    override val primaryKey = PrimaryKey(id)
}

val customerStorage = mutableListOf<Customer>()
package com.example.dao

import com.example.dao.DatabaseFactory.dbQuery
import com.example.models.Customer
import com.example.models.Customers
import kotlinx.coroutines.runBlocking
import org.jetbrains.exposed.sql.ResultRow
import org.jetbrains.exposed.sql.selectAll

class DAOFacadeImpl : DAOFacade {
    private fun resultRowToCustomer(row: ResultRow) = Customer(
        id = row[Customers.id],
        firstName = row[Customers.firstName],
        lastName = row[Customers.lastName],
        email = row[Customers.email]
    )
    override suspend fun allCustomers(): List<Customer> = dbQuery {
        Customers.selectAll().map(::resultRowToCustomer)
    }
}

val dao: DAOFacade = DAOFacadeImpl().apply {
    runBlocking {
        if (allCustomers().isEmpty()) {
        }
    }
}
package com.example.dao

import com.example.dao.DatabaseFactory.dbQuery
import com.example.models.Customer
import com.example.models.Customers
import kotlinx.coroutines.runBlocking
import org.jetbrains.exposed.sql.*

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

    override suspend fun customer(id: Int): Customer? {
        TODO("Not yet implemented")
    }

    override suspend fun addNewCustomer(firstName: String, lastName: String, email: String): Customer? = dbQuery {
        val insertStatement = Customers.insert {
            it[Customers.firstName] = firstName
            it[Customers.lastName] = lastName
            it[Customers.email] = email
        }
        insertStatement.resultedValues?.singleOrNull()?.let(::resultRowToCustomer)
    }

    override suspend fun editCustomer(id: Int, firstName: String, lastName: String, email: String): Boolean {
        TODO("Not yet implemented")
    }

    override suspend fun deleteCustomer(id: Int): Boolean {
        TODO("Not yet implemented")
    }
}

val dao: DAOFacade = DAOFacadeImpl().apply {
    runBlocking {
        if (allCustomers().isEmpty()) {
        }
    }
}
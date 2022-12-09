package com.example.dao

import com.example.models.*

interface DAOFacade {
    suspend fun allCustomers(): List<Customer>
}
class Client(val name: String, val postalCode: Int) {
    override fun toString() = "Client(name=$name, postalCode=$postalCode)"

    override fun equals(other: Any?): Boolean {
        if (other == null || other !is Client)
            return false

        return name == other.name && postalCode == other.postalCode
    }

    override fun hashCode(): Int = name.hashCode() * 31 + postalCode
}

val client1 = Client("Alice", 342562)
println(client1)

val client2 = Client("Alice", 342562)
println(client1 == client2)

val processed = hashSetOf(Client("Alice", 342562))
println(processed.contains(Client("Alice", 342562)))
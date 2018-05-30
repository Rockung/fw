data class Client(val name: String, val postalCode: Int)

val client1 = Client("Alice", 342562)
println(client1)

val client2 = Client("Alice", 342562)
println(client1 == client2)

val processed = hashSetOf(Client("Alice", 342562))
println(processed.contains(Client("Alice", 342562)))
fun printMessageWithPrefix(messages: Collection<String>, prefix: String) {
    messages.forEach {
        println("$prefix $it")
    }
}

fun printProblemCounts(response: Collection<String>) {
    var clientErrors = 0
    var serverErrors = 0

    response.forEach {
        if (it.startsWith("4")) {
            clientErrors++
        } else if (it.startsWith("5")) {
            serverErrors++
        }
    }
    println("$clientErrors client errors, $serverErrors server errors")
}

val errors = listOf("403 Forbidden", "404 Not Found")
printMessageWithPrefix(errors, "Error:")

val responses = listOf("200 OK", "418 I'm a teapot",
                       "500 Internal Server Error")
printProblemCounts(responses)

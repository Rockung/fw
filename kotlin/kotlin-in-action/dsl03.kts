// appendExcl is a value of an extension function type
val appendExcl: StringBuilder.() -> Unit = { this.append("!") }

val stringBuilder = StringBuilder("Hi")
stringBuilder.appendExcl()

println(stringBuilder)

fun buildString(
    builderAction: StringBuilder.() -> Unit
): String {
    val sb = StringBuilder()
    sb.builderAction()
    return sb.toString()
}

// passes appendExcl as an argument
println(buildString(appendExcl))

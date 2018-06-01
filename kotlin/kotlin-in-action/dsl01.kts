fun buildString(
    // declares a parameter of a function type
    builderAction: (StringBuilder) -> Unit
): String {
    val sb = StringBuilder()
    // passes a StringBuilder as an argument to the lambda
    builderAction(sb)
    return sb.toString()
}

val s = buildString {
    // `it` refers to the StringBuilder instance
    it.append("Hello, ")
    it.append("World!")
}

println(s)

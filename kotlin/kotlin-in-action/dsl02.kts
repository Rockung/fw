fun buildString(
    // declares a parameter of a function type with a receiver
    builderAction: StringBuilder.() -> Unit
): String {
    val sb = StringBuilder()
    // passes a StringBuilder as a receiver to the lambda
    sb.builderAction()
    return sb.toString()
}

val s = buildString {
    // `this` refers to the StringBuilder instance
    this.append("Hello, ")
    // refers to StringBuilder implicitly
    append("World!")
}

println(s)

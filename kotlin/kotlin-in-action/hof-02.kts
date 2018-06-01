fun String.filter(predicate: (Char) -> Boolean) : String {
    val sb = StringBuilder()
    for (index in 0 until length) {
        val element = get(index)
        if (predicate(element)) sb.append(element)
    }
    return sb.toString()
}

println("ab1c".filter { it in 'a'..'z'})

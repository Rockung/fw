fun alphabet() = StringBuilder().apply {
    for (letter in 'A' .. 'Z') {
        append(letter)
    }
    append("\nNow I know the alphabet")
}.toString()

println(alphabet())

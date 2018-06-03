data class NameComponents(val name: String, val extension: String)

fun splitFilename(fullName: String): NameComponents {
    // val result = fullName.split('.', limit = 2)
    // return NameComponents(result[0], result[1])

    val (name, extension) = fullName.split('.', limit = 2)
    return NameComponents(name, extension)
}

val (name, ext) = splitFilename("example.kt")
println(name)
println(ext)
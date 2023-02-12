import java.io.File

object MainK {

  @JvmStatic fun main(args : Array<String>) {
    val start = System.currentTimeMillis()
    val words = File("words.txt").readLines().filterNot { it.length != it.toSet().size }.toList()
    val solution = process("", words)!!
    println("Solution: ${solution.chunked(5)} found in ${System.currentTimeMillis() - start} ms")
  }

  private fun process(current: String, words: List<String>): String? {
    if (current.length >= 25) return current
    val letter = LETTER_FREQUENCIES.first { !current.contains(it) }
    val parts = words.partition { it.contains(letter) }
    return parts.first.firstNotNullOfOrNull {
        word -> process(word + current, parts.second.filterNot { w -> w.any { word.contains(it) } })
    }
  }

  private const val LETTER_FREQUENCIES = "etaonrishdlfcmugypwbvkjxzq"
}

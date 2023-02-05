package test

import java.io.File
import kotlin.system.measureTimeMillis


object Main {

  @JvmStatic fun main(args : Array<String>) {
    val words = File("words.txt").readLines().filterNot { it.length != it.toSet().size }.toList()
    val elapsed = measureTimeMillis {
      process("", words)?.let { println ("found ${it.chunked(5)}") }
    }
    println("Elasped = ${elapsed/1000.0}")
  }

  private fun process(current: String, words: List<String>): String? {
    if (current.length >= 25)
      return current
    val letter = LETTER_FREQUENCIES.first { !current.contains(it) }
    val parts = words.partition { it.contains(letter) }
    return parts.first.firstNotNullOfOrNull {
      word -> process(word + current, parts.second.filterNot { w -> w.any { word.contains(it) } })
    }
  }

  private const val LETTER_FREQUENCIES = "etaonrishdlfcmugypwbvkjxzq"
}


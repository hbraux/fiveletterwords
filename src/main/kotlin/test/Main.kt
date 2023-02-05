package test

import java.io.File
import kotlin.system.measureTimeMillis


object Main {

  @JvmStatic fun main(args : Array<String>) {
    val words = File("words.txt").readLines().filterNot { it.length != it.toSet().size }.toMutableList()
    val elapsed = measureTimeMillis {
      process(words)?.let { println ("found ${it.chunked(5)}") }
    }
    println("Elasped = ${elapsed/1000.0}")
  }

  private fun process(words: MutableList<String>): String? {
    LETTER_FREQUENCIES.forEach { letter ->
      while (words.isNotEmpty()) {
        val word = pickWord(words, letter) ?: break
        findNext(word, words)?.let { return(it) }
        words.remove(word)
        println("Remaining: ${words.size}")
      }
    }
    return null
  }

  private fun findNext(current: String, words: List<String>): String? {
    if (current.length >= 22)
      return current
    val wordchars = current.toSet()
    val filtered = words.filterNot { w -> w.any { wordchars.contains(it) } }
    return filtered.firstNotNullOfOrNull {
      findNext(it + current, filtered)
    }
  }

  private fun pickWord(words: List<String>, letter: Char): String? = words.firstOrNull { it.contains(letter) }


  private const val LETTER_FREQUENCIES = "etaonrishdlfcmugypwbvkjxzq"
}


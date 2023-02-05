package test

import java.io.File
import kotlin.system.measureTimeMillis


object Main {

  @JvmStatic fun main(args : Array<String>) {
    val words = File("words.txt").readLines().filterNot { it.length != it.toSet().size }.toMutableList()
    val elapsed = measureTimeMillis {
      process(words)
    }
    println(elapsed)
  }

  private fun process(words: MutableList<String>) {
    LETTER_FREQUENCIES.forEach { letter ->
      println("processing words with letter $letter")
      while (words.isNotEmpty()) {
        val word = pickWord(words, letter) ?: break
        findNext(word, words)
        words.remove(word)
        println("Remaining: ${words.size}")
      }
    }
  }

  private fun findNext(current: String, words: List<String>) {
    if (current.length >= 25) {
      println("Found: $current !")
      System.exit(0)
    }
    val wordchars = current.toSet()
    val filtered = words.filterNot { w -> w.any { wordchars.contains(it) } }
    filtered.forEach { findNext(it + current, filtered) }
  }

  private fun pickWord(words: List<String>, letter: Char): String? = words.firstOrNull { it.contains(letter) }


  private const val LETTER_FREQUENCIES = "etaonrishdlfcmugypwbvkjxzq"
}


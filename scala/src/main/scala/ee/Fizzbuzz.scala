package ee

import scala.collection._

/**
  * Fizzbuzz with a twist, ie. map:
  * - if contains a '3' - lucky
  * - every 3rd - fizz
  * - every 5th - buzz
  * - every 15th - fizzbuzz
  * - otherwise a digit
  */
object Fizzbuzz {

  def fizzbuzz(i: Int) = {
    if (i < 1)
      throw new IndexOutOfBoundsException(s"invalid fizzbuzz index: $i")
    val fizz = i % 3 == 0
    val buzz = i % 5 == 0
    val lucky = i.toString contains '3'
    (fizz, buzz, lucky) match {
      case (_, _, true) => "lucky"
      case (true, true, _) => "fizzbuzz"
      case (true, _, _) => "fizz"
      case (_, true, _) => "buzz"
      case _ => i
    }
  }

  def fizzbuzzList(start: Int, end: Int) = {
    if (start > end)
      throw new IndexOutOfBoundsException(s"invalid fizzbuzzList range: $start to $end")
    for (x <- start to end) yield fizzbuzz(x)
  }

  def printFizzbuzzList(start: Int, end: Int) {
    print(fizzbuzzList(start, end).mkString(" "))
  }

  def fizzbuzzReport(start: Int, end: Int) = {
    val report = mutable.Map[String, Int]().withDefaultValue(0)
    for (i <- start to end) fizzbuzz(i) match {
      case _: Int => report("integer") += 1
      case x: String => report(x) += 1
    }
    report
  }

}
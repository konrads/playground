package ee

import java.io._

import ee.Fizzbuzz._
import org.scalatest._

class FizzbuzzTest extends FlatSpec with Matchers {

  val _1To20 = List(1, 2, "lucky", 4, "buzz", "fizz", 7, 8, "fizz", "buzz", 11, "fizz", "lucky", 14, "fizzbuzz", 16, 17, "fizz", 19, "buzz")

  "printFizzbuzzList" should "be correct for 1 to 20" in {
    val baos = new ByteArrayOutputStream
    scala.Console.withOut(baos)(printFizzbuzzList(1, 20))
    baos.toString should equal (_1To20.mkString(" "))
  }

  "fizzbuzzList" should "be correct for 1 to 20" in {
    fizzbuzzList(1, 20) should equal (_1To20)
  }
  it should "be correct for a list of 1" in {
    fizzbuzzList(1, 1) should equal (List(1))
  }
  it should "fail for negative indices" in {
    an [IndexOutOfBoundsException] should be thrownBy fizzbuzzList(-1, 5)
    an [IndexOutOfBoundsException] should be thrownBy fizzbuzzList(1, -5)
  }

  "fizzbuzz" should "fail for negative indices" in {
    an [IndexOutOfBoundsException] should be thrownBy fizzbuzz(-1)
  }
  it should "'lucky' every /.*3/" in {
    fizzbuzz(3) should equal ("lucky")
    fizzbuzz(13) should equal ("lucky")
    fizzbuzz(53) should equal ("lucky")
  }
  it should "'fizz' every 3rd element" in {
    fizzbuzz(6) should equal ("fizz")
    fizzbuzz(12) should equal ("fizz")
    fizzbuzz(999) should equal ("fizz")
  }
  it should "'buzz' every 5th element" in {
    fizzbuzz(5) should equal ("buzz")
    fizzbuzz(10) should equal ("buzz")
    fizzbuzz(100) should equal ("buzz")
  }
  it should "'fizzbuzz' every 15th element" in {
    fizzbuzz(15) should equal ("fizzbuzz")
    fizzbuzz(45) should equal ("fizzbuzz")
  }
  it should "preserve number otherwise" in {
    fizzbuzz(2) should equal (2)
    fizzbuzz(4) should equal (4)
    fizzbuzz(101) should equal (101)
  }

  "fizzbuzzReport" should "be valid for 1 to 20" in {
    val report = fizzbuzzReport(1, 20)
    val expected = Map("fizz" -> 4, "buzz" -> 3, "fizzbuzz" -> 1, "lucky" -> 2, "integer" -> 10)
    report should equal (expected)
  }
  it should "be valid for 6 to 7" in {
    val report = fizzbuzzReport(6, 7)
    val expected = Map("fizz" -> 1, "integer" -> 1)
    report should equal (expected)
  }
  it should "be valid for 100 to 105" in {
    val report = fizzbuzzReport(100, 105)
    val expected = Map("fizz" -> 1, "buzz" -> 1, "fizzbuzz" -> 1, "lucky" -> 1, "integer" -> 2)
    report should equal (expected)
  }

}

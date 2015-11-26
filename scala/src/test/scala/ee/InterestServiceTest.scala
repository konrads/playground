package ee

import org.scalatest.{FlatSpec, Matchers}

import scala.collection.immutable.TreeMap

class InterestServiceTest extends FlatSpec with Matchers {

  val service = new InterestService(TreeMap(
    0.0 -> (_ => 0),
    1000.0 -> (_ * 0.01),
    5000.0 -> (_ * 0.02),
    Double.PositiveInfinity -> (_ * 0.03)
  ))

  "interest-service" should "work for £0 to £1000" in {
    service.calc(0) shouldEqual 0
    service.calc(1000) shouldEqual 10
    service.calc(500) shouldEqual 5
  }
  it should "work for fractions" in {
    service.calc(1) shouldEqual 0.01
    service.calc(13.7) shouldEqual 0.14
    service.calc(13.3) shouldEqual 0.13
  }
  it should "work for less than £0" in {
    service.calc(-200) shouldEqual 0
    service.calc(-5000) shouldEqual 0
  }
  it should "work for £1000 to £5000" in {
    service.calc(2000) shouldEqual 40
    service.calc(5000) shouldEqual 100
  }
  it should "work for £5000+" in {
    service.calc(10000) shouldEqual 300
  }

}

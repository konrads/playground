package patterns

import org.scalatest.{FlatSpec, Matchers}

class LLTest extends FlatSpec with Matchers {
  "LL" should "construct" in {
    LL(1,2,3).toList shouldEqual List(1,2,3)
  }

  it should "prepend" in {
    (1 :: LL(2,3,4)).toList shouldEqual List(1,2,3,4)
  }

  it should "map" in {
    LL(1,2).map(_*2).toList shouldEqual List(2,4)
  }

  it should "filter" in {
    LL(1,2,3,4,5).filter(_ % 2 !=0).toList shouldEqual List(1,3,5)
  }

  it should "foldLeft" in {
    LL(1,2,3).foldLeft(_*_, 10) shouldEqual 60
  }

}

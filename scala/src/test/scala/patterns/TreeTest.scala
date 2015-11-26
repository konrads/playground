package patterns

import org.scalatest.{Matchers, FlatSpec}

class TreeTest extends FlatSpec with Matchers {
  val tree = new Branch(new Leaf(11), new Branch(new Leaf(22), new Leaf(33), new Leaf(44), new Branch(new Branch(new Leaf(121)))))

  "Tree" should "map correctly" in {
    val mapped = tree.map(_/11)
    mapped should have length 5
    mapped shouldEqual Seq(1, 2, 3, 4 ,11)
  }

  it should "reduce correctly" in {
    tree.reduce(_+_, 0) shouldEqual 231
  }

}

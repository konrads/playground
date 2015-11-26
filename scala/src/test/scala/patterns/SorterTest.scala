package patterns

import langfeatures.Sorter
import org.scalatest.{Matchers, FlatSpec}

class SorterTest extends FlatSpec with Matchers {
//  type T <: Any
//  implicit val listLenOrdering = new Ordering[List[T]] {
//    override def compare(x: List[T], y: List[T]): Int = x.length - y.length
//  }

  implicit val listLenOrdering = new Ordering[List[Int]] {
    override def compare(x: List[Int], y: List[Int]): Int = x.length - y.length
  }

  "MergeSort" should "sort 1,2,8,7" in {
    Sorter.mergeSort(List(1,2,8,7)) shouldEqual List(1,2,7,8)
  }

  "MyMergeSort" should "sort Ints" in {
    Sorter.myMergeSort(List(1,2,8,7)) shouldEqual List(1,2,7,8)
  }
  it should "sort Strings" in {
    Sorter.myMergeSort(List("aa","bb","dd","cc")) shouldEqual List("aa","bb","cc","dd")
  }
  it should "sort Lists" in {
    Sorter.myMergeSort(List(
      List(1,2,3),
      List(4,5),
      List(1)
    ))(listLenOrdering) shouldEqual List(  // not necessary, implicit kicked in
    // )) shouldEqual List(
      List(1),
      List(4,5),
      List(1,2,3)
    )
  }
}

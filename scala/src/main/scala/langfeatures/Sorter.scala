package langfeatures

object Sorter {
  def mergeSort(xs: List[Int]): List[Int] = {
    val n = xs.length / 2
    if (n == 0) xs
    else {
      def merge(xs: List[Int], ys: List[Int]): List[Int] = {
        (xs, ys) match {
          case (Nil, ys) => ys
          case (xs, Nil) => xs
          case (x :: xs1, y :: ys1) =>
            if (x < y) x :: merge(xs1, ys)
            else y :: merge(xs, ys1)
        }
      }
      val (left, right) = xs splitAt (n)
      merge(mergeSort(left), mergeSort(right))
    }
  }

  def myMergeSort[T](xs: List[T])(implicit cmp: Ordering[T]): List[T] = {
    if (xs.length < 2)
      xs
    else {
      def myMerge(sortedLeft: List[T], sortedRight: List[T]): List[T] =
        (sortedLeft, sortedRight) match {
          case (Nil, _) => sortedRight
          case (_, Nil) => sortedLeft
          case (l :: sortedLeft2, r :: sortedRight2) if cmp.lt(l, r) => l :: myMerge(sortedLeft2, sortedRight)
          case (_, r :: sortedRight2) => r :: myMerge(sortedLeft, sortedRight2)
        }
      val (left, right) = xs splitAt (xs.length/2)
      myMerge(myMergeSort(left), myMergeSort(right))
    }
  }
}

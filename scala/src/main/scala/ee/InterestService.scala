package ee

import scala.collection.SortedMap

/**
  * Interest calculating service, with boundaries defining different calculations.
  * Notes to anal recruiters:
  * - boundaries are treated as inclusive
  * - trenches are calculated not building on top of the previous trench
  * - in this case - trench < 0 produces 0, not exception
  */
class InterestService(trenches: SortedMap[Double, Double => Double], precision: Int=2) {

  def calc(amount: Double) = {
    val interest = trenches.dropWhile(_._1 < amount).head._2(amount)
    round(interest)
  }

  def round(d: Double) = {
    val multiplier = math.pow(10, precision)
    math.round(d * multiplier) / multiplier
  }
}

package ee.shoppingcart.with_discounts_and_notifications

import ee.shoppingcart.{Deal, Item}

/**
  * Discount rules that also incorporate Notices if product is eligible for discount,
  * but discount criteria hasn't been finalized
  */
object DiscountRule {
  type Tag = String
  type Notice = String
  type DiscountRule = Seq[Item] => Seq[(Item, Option[Notice])]

  // eg. 10% off Dove Soap
  def percOff(tag: Tag, discount: BigDecimal): DiscountRule = {
    def implementation(items: Seq[Item]) = {
      items.collect {
        // treating each item as a deal in itself
        case i if isDealCandidate(tag)(i) => (Deal(price=i.price * (1 - discount), tags=i.tags, contents=Seq(i)), None)
        case i                            => (i, None)
      }
    }
    implementation
  }

  // eg. 3 Axe Soaps for the price of 2
  def xForThePriceOfY(tag: Tag, x: Int, y: Int): DiscountRule = {
    assert(x > y, s"invalid x & y: $x <= $y")
    def implementation(items: Seq[Item]) = {
      val noDeals = items.filterNot(isDealCandidate(tag))
      val dealCandidates = items.filter(isDealCandidate(tag))
      val withDeals = dealCandidates.sliding(x, x).flatMap {
        case slice if slice.length == x =>
          // FIXME: presuming all prices in the deal are the same, but averaging them just in case...
          val avgPrice = {
            val allPrices = slice.map(_.price)
            allPrices.sum / slice.length
          }
          Seq((Deal(avgPrice * y, Nil, slice), None))
        case slice =>
          slice.map(i => (i, Some(s"${i.getClass.getSimpleName}: eligible for $x for the price of $y")))
      }
      withDeals.toSeq ++ noDeals.map((_, None))
    }
    implementation
  }

  private def isDealCandidate(tag: Tag)(item: Item) = {
    item match {
      case _:Deal                    => false
      case i if i.tags.contains(tag) => true
      case _                         => false
    }
  }

  val soapDiscounts = Seq(
    DiscountRule.percOff("soap", .1)
  )

  val shampooDiscounts = Seq(
    DiscountRule.xForThePriceOfY("shampoo", 3, 2)
  )

  val allDiscounts = soapDiscounts ++ shampooDiscounts
}

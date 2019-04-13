package ee.shoppingcart.with_discounts

import ee.shoppingcart.{Deal, Item}

/**
  * Discount Rules based on:
  * - % off
  * - x for the price of y (x>y)
  * - 'meal deal'
  */
object DiscountRule {
  type Tag = String
  type DiscountRule = Seq[Item] => Seq[Item]

  // eg. 10% off Dove Soap
  def percOff(tag: Tag, discount: BigDecimal): DiscountRule = {
    def implementation(items: Seq[Item]) = {
      items.collect {
        // treating each item as a deal in itself
        case i if isDealCandidate(tag)(i) => Deal(price=i.price * (1 - discount), tags=i.tags, contents=Seq(i))
        case i                            => i
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
          Seq(Deal(avgPrice * y, Nil, slice))
        case slice =>
          slice
      }
      withDeals.toSeq ++ noDeals
    }
    implementation
  }

  def deal(tags: Seq[Tag], price: BigDecimal): DiscountRule = {
    // for customer's sake, apply discount to most expensive items first
    def implementation(items: Seq[Item]) = {
      val mostExpensiveFirst = items.sortBy(- _.price)
      def getDeal(is: Seq[Item], tags: Seq[Tag]): (Option[Deal], Seq[Item]) = {
        def getDeal2(is2: Seq[Item], tags2: Seq[Tag], candidates: Seq[Item]=Nil): (Seq[Item] /*non deal candidates*/, Seq[Tag] /*unused tags*/, Seq[Item] /*deal candidates so far*/) = {
          val tagHead :: tagTail = tags2
          (tagTail, is2.span(! _.tags.contains(tagHead))) match {
            case (Nil, (nonCandidates, candidate :: others)) =>  (nonCandidates ++ others, Nil, candidate +: candidates)
            case (_,   (nonCandidates, candidate :: others)) =>  getDeal2(nonCandidates ++ others, tagTail, candidate +: candidates)
            case (_,   (nonCandidates, Nil))                 =>  (nonCandidates, tagTail, Nil)
          }
        }
        getDeal2(is, tags) match {
          case (is3, Nil, candidates) => (Some(Deal(price, contents=candidates)), is3)
          case _                      => (None, is)
        }
      }
      def getDeals(is: Seq[Item], deals: Seq[Deal]): (Seq[Deal], Seq[Item]) =
        getDeal(is, tags) match {
          case (None, is2)       => (deals, is2)                 // no more deals
          case (Some(deal), Nil) => (deals :+ deal, Nil)         // got more deals, keep looking
          case (Some(deal), is2) => getDeals(is2, deals :+ deal) // got more deals, keep looking
        }
      val (deals, noDeals) = getDeals(mostExpensiveFirst, Nil)
      deals ++ noDeals
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

  val dealDiscounts = Seq(
    DiscountRule.deal(Seq("axe-deodorant-deal", "axe-shampoo-deal"), 10)
  )

  val allDiscounts = soapDiscounts ++ shampooDiscounts ++ dealDiscounts
}

package ee.shoppingcart.with_discounts

import DiscountRule.DiscountRule
import ee.shoppingcart.Item

import scala.math.BigDecimal.RoundingMode

/**
  * Shopping cart integrating discount rules.
  */
case class ShoppingCart(contents: Seq[Item] = Nil) {
  def add(items: Item*): ShoppingCart =
    ShoppingCart(contents ++ items)

  def total(discountRules: Seq[DiscountRule] = Nil): BigDecimal =
    (preTaxTotal(discountRules) * (BigDecimal(1) + ShoppingCart.salesTax)).setScale(2, RoundingMode.HALF_UP)

  def salesTax(discountRules: Seq[DiscountRule] = Nil): BigDecimal =
    (preTaxTotal(discountRules) * ShoppingCart.salesTax).setScale(2, RoundingMode.HALF_UP)

  private def preTaxTotal(discountRules: Seq[DiscountRule]): BigDecimal = {
    val discounted = discountRules.foldLeft(contents) {
      case (items, rule) => rule(items)
    }
    discounted.map(_.price).sum
  }
}

object ShoppingCart {
  val salesTax: BigDecimal = BigDecimal(0.125)
}
